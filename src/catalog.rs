//! Catalog: the typed in-memory representation of one or more vendor YAML files.
//!
//! Schema mirrors `catalog/SCHEMA.md`. See that doc for field semantics.

use crate::command_types::CommandType;
use crate::error::{Error, Result};
use crate::version::{FirmwareVersion, VersionRange};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Bundled catalog files — embedded at compile time so `Catalog::load_bundled()`
/// works without any filesystem access. Order is alphabetical; the loader
/// is order-independent.
///
/// v0.1: every bundled file is a placeholder skeleton (vendor identity
/// populated, `commands: []` empty). Real catalog data lands when the
/// doc-crawl effort runs against vendor docs.
const BUNDLED: &[(&str, &str)] = &[
    ("lg-webos.yaml", include_str!("../catalog/lg-webos.yaml")),
    (
        "nec-naviset.yaml",
        include_str!("../catalog/nec-naviset.yaml"),
    ),
    (
        "pjlink-projectors.yaml",
        include_str!("../catalog/pjlink-projectors.yaml"),
    ),
    (
        "samsung-mdc.yaml",
        include_str!("../catalog/samsung-mdc.yaml"),
    ),
    (
        "sony-bravia.yaml",
        include_str!("../catalog/sony-bravia.yaml"),
    ),
];

/// In-memory catalog. Indexed by vendor identifier (`samsung_mdc`, `lg_webos`,
/// etc.). Build via [`Catalog::load_bundled`] or [`Catalog::load_dir`].
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    vendors: IndexMap<String, VendorFile>,
}

impl Catalog {
    /// Load the catalog files bundled with this crate (no filesystem access).
    pub fn load_bundled() -> Result<Self> {
        let mut cat = Catalog::default();
        for (name, body) in BUNDLED {
            let parsed: VendorFile =
                serde_yaml::from_str(body).map_err(|source| Error::CatalogParse {
                    file: (*name).to_owned(),
                    source,
                })?;
            cat.vendors.insert(parsed.vendor.clone(), parsed);
        }
        Ok(cat)
    }

    /// Load every `*.yaml` file in a directory. Useful for consumer-supplied
    /// overrides on top of (or instead of) the bundled set.
    pub fn load_dir(dir: impl AsRef<Path>) -> Result<Self> {
        let mut cat = Catalog::default();
        for entry in std::fs::read_dir(dir.as_ref())? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let body = std::fs::read_to_string(&path)?;
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("?")
                .to_owned();
            let parsed: VendorFile = serde_yaml::from_str(&body)
                .map_err(|source| Error::CatalogParse { file: name, source })?;
            cat.vendors.insert(parsed.vendor.clone(), parsed);
        }
        Ok(cat)
    }

    /// All vendor identifiers in this catalog.
    pub fn vendors(&self) -> impl Iterator<Item = &str> {
        self.vendors.keys().map(String::as_str)
    }

    /// Get the full vendor file by id.
    pub fn vendor(&self, id: &str) -> Option<&VendorFile> {
        self.vendors.get(id)
    }

    /// Look up the most-specific catalog entry for `(vendor, firmware, command)`.
    /// Returns `None` if the vendor doesn't have an entry for this command type.
    pub fn lookup(
        &self,
        vendor: &str,
        firmware: &str,
        command: CommandType,
    ) -> Result<Option<&CommandEntry>> {
        let vf = self
            .vendors
            .get(vendor)
            .ok_or_else(|| Error::UnknownVendor(vendor.to_owned()))?;
        let fw = FirmwareVersion::parse(firmware)?;

        let Some(cmd) = vf.commands.iter().find(|c| c.command_type == command) else {
            return Ok(None);
        };

        // Pick the most-specific matching `versions` block.
        let mut best: Option<(&CommandEntry, usize)> = None;
        for entry in &cmd.versions {
            let range = VersionRange::parse(&entry.applies_to)?;
            if !range.matches(&fw) {
                continue;
            }
            let score = range.specificity();
            if best.is_none_or(|(_, s)| score > s) {
                best = Some((entry, score));
            }
        }

        match best {
            Some((entry, _)) => {
                if entry.cli == "NOT_SUPPORTED" {
                    return Err(Error::NotSupported {
                        vendor: vendor.to_owned(),
                        command,
                        reason: entry.notes.clone().unwrap_or_default(),
                    });
                }
                Ok(Some(entry))
            }
            None => Err(Error::NoMatchingEntry {
                vendor: vendor.to_owned(),
                firmware: Some(firmware.to_owned()),
                command,
            }),
        }
    }
}

// ── YAML schema types ───────────────────────────────────────────────

/// One vendor catalog file (`<vendor>.yaml`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorFile {
    /// Stable vendor identifier (`samsung_mdc`, `lg_webos`, etc.).
    pub vendor: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Manufacturer (Samsung Electronics, LG Electronics, etc.).
    pub manufacturer: String,
    /// Product family within the manufacturer's lineup.
    pub product_family: String,
    /// Free-form notes (multi-line OK).
    #[serde(default)]
    pub notes: Option<String>,
    /// Citations for the data in this file.
    #[serde(default)]
    pub sources: Vec<Source>,
    /// Per-protocol availability metadata.
    #[serde(default)]
    pub protocol_capabilities: ProtocolCapabilities,
    /// Per-command-type entries.
    pub commands: Vec<CommandBlock>,
}

/// A documentation citation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Document title.
    pub title: String,
    /// URL.
    pub url: String,
    /// ISO date (YYYY-MM-DD).
    pub accessed: String,
}

/// Top-level protocol-availability metadata for a vendor.
///
/// Display vendors don't speak NETCONF/RESTCONF/gNMI (those are
/// switch-domain protocols). The slots here are the actually-deployed
/// IP-managed-display protocols.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProtocolCapabilities {
    /// Samsung Multiple Display Control (MDC) — binary protocol over TCP/1515 or RS-232.
    #[serde(default)]
    pub mdc: Option<ProtocolCapability>,
    /// LG webOS Secure Second Screen (SSAP) — JSON-over-WebSocket on TCP/3000 (or 3001 secure).
    #[serde(default)]
    pub webos_ssap: Option<ProtocolCapability>,
    /// Sony Bravia Professional Display Simple IP Control (PSI) — HTTP / IRCC.
    #[serde(default)]
    pub bravia_psi: Option<ProtocolCapability>,
    /// PJLink — open ASCII protocol on TCP/4352, cross-vendor projector standard.
    #[serde(default)]
    pub pjlink: Option<ProtocolCapability>,
    /// HDMI-CEC — last-mile control over the HDMI cable. Vendor implementations fragment
    /// (Samsung AnyNet+, LG SimpLink, Sony BRAVIA Sync); only power-on/off is reliably
    /// cross-vendor.
    #[serde(default)]
    pub cec: Option<ProtocolCapability>,
    /// Generic vendor REST/HTTP API (NEC NaViSet, others without a more specific slot).
    #[serde(default)]
    pub rest_api: Option<ProtocolCapability>,
}

/// Per-protocol availability info — when introduced + freeform notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCapability {
    /// First firmware version that introduced this protocol.
    #[serde(default)]
    pub introduced_in: Option<String>,
    /// Vendor notes / config requirements (e.g. "Network Standby must be enabled").
    #[serde(default)]
    pub notes: Option<String>,
    /// Catch-all for vendor-specific extras (auth scheme, default port, rate limits).
    #[serde(flatten)]
    pub extras: IndexMap<String, serde_yaml::Value>,
}

/// A single command-type block in a vendor file. May contain multiple
/// `versions` entries for firmware-version-aware selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    /// Abstract command type.
    #[serde(rename = "type")]
    pub command_type: CommandType,
    /// Human description.
    #[serde(default)]
    pub description: Option<String>,
    /// One entry per firmware-version range.
    pub versions: Vec<CommandEntry>,
    /// Protocol alternatives (MDC, SSAP, PSI, PJLink, CEC, REST).
    #[serde(default)]
    pub protocol_alternatives: ProtocolAlternatives,
}

/// One concrete command entry — applies to a specific firmware-version range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEntry {
    /// Version range this entry applies to (e.g., `">=2020"`).
    pub applies_to: String,
    /// The CLI/payload string. Use `NOT_SUPPORTED` to mark vendor-absent commands.
    /// For binary protocols (MDC), this is a hex byte sequence; for ASCII (PJLink),
    /// the literal command; for SSAP / PSI, the JSON or URL template. The
    /// per-vendor catalog's `notes` documents the encoding convention used.
    pub cli: String,
    /// Real-world response sample (where applicable).
    #[serde(default)]
    pub sample_output: Option<String>,
    /// Parser hints.
    #[serde(default)]
    pub parser_notes: Option<String>,
    /// Required device configuration for this command to work
    /// (e.g. "Network Standby ON", "External Control enabled").
    #[serde(default)]
    pub config_required: Option<String>,
    /// Vendor-specific quirks / deprecation notes.
    #[serde(default)]
    pub notes: Option<String>,
    /// True if extracted from a heuristic source rather than vendor docs.
    #[serde(default)]
    pub unverified: bool,
}

/// Per-command protocol alternatives. All slots are optional / nullable in YAML.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProtocolAlternatives {
    /// Samsung MDC (binary) mapping.
    #[serde(default)]
    pub mdc: Option<MdcMapping>,
    /// LG webOS SSAP (JSON-over-WebSocket) mapping.
    #[serde(default)]
    pub webos_ssap: Option<WebosSsapMapping>,
    /// Sony Bravia PSI (HTTP/IRCC) mapping.
    #[serde(default)]
    pub bravia_psi: Option<BraviaPsiMapping>,
    /// PJLink (ASCII over TCP/4352) mapping.
    #[serde(default)]
    pub pjlink: Option<PjlinkMapping>,
    /// HDMI-CEC mapping.
    #[serde(default)]
    pub cec: Option<CecMapping>,
    /// Generic vendor REST mapping (NEC NaViSet, others).
    #[serde(default)]
    pub rest_api: Option<RestApiMapping>,
}

/// Samsung MDC binary command mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdcMapping {
    /// Command-id byte (e.g. `0x11` for power, `0x14` for input).
    pub command_id: String,
    /// Full request frame as a hex string (header + id + length + data + checksum).
    pub request_frame: String,
    /// Response frame layout description.
    #[serde(default)]
    pub response_frame: Option<String>,
    /// Minimum firmware version.
    #[serde(default)]
    pub firmware_required: Option<String>,
}

/// LG webOS SSAP command mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebosSsapMapping {
    /// SSAP URI (e.g. `ssap://system/turnOff`).
    pub uri: String,
    /// JSON payload, if any.
    #[serde(default)]
    pub payload: Option<String>,
    /// Minimum firmware version.
    #[serde(default)]
    pub firmware_required: Option<String>,
}

/// Sony Bravia PSI command mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraviaPsiMapping {
    /// JSON-RPC method name (Sony's IP Control "system" / "audio" / "videoScreen" services).
    pub method: String,
    /// Service name the method belongs to.
    pub service: String,
    /// JSON params object as a string.
    #[serde(default)]
    pub params: Option<String>,
    /// IRCC command code (some legacy commands only available via IRCC, not REST).
    #[serde(default)]
    pub ircc_code: Option<String>,
    /// Minimum firmware version.
    #[serde(default)]
    pub firmware_required: Option<String>,
}

/// PJLink command mapping. Cross-vendor projector standard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PjlinkMapping {
    /// PJLink command (e.g. `POWR`, `INPT`, `AVMT`, `INST`).
    pub command: String,
    /// PJLink class (1 or 2). Class 2 adds streaming + serial-pass-through.
    pub class: u8,
    /// Argument for set-style commands (`POWR 1` to power on, etc.).
    #[serde(default)]
    pub argument: Option<String>,
}

/// HDMI-CEC command mapping. v0.1 carries the abstract opcode; v0.2 may
/// extend with vendor-extension byte sequences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CecMapping {
    /// CEC opcode name (e.g. `IMAGE_VIEW_ON`, `STANDBY`, `ACTIVE_SOURCE`).
    pub opcode: String,
    /// Hex opcode byte.
    pub opcode_byte: String,
    /// Required CEC version (1.4, 2.0).
    #[serde(default)]
    pub cec_version: Option<String>,
    /// Cross-vendor reliability note (per blueprint: only power-on/off is reliable cross-vendor).
    #[serde(default)]
    pub cross_vendor_note: Option<String>,
}

/// Generic vendor REST mapping (NEC NaViSet, others).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiMapping {
    /// HTTP method.
    pub method: String,
    /// URL path (relative to vendor base URL).
    pub path: String,
    /// Minimum firmware version.
    #[serde(default)]
    pub firmware_required: Option<String>,
    /// Catch-all (auth scheme, response shape, etc.).
    #[serde(flatten)]
    pub extras: IndexMap<String, serde_yaml::Value>,
}
