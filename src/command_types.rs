//! Abstract command types — vendor-neutral vocabulary for IP-managed display operations.
//!
//! See [`catalog/COMMAND_TYPES.md`](https://github.com/Mediacastnet/mediacast-displaycatalog/blob/main/catalog/COMMAND_TYPES.md)
//! for prose descriptions.
//!
//! ## Provenance
//!
//! The starting set was extracted from Mediacast platform planning docs:
//! [`mediacast-platform-blueprint.md`](https://github.com/Mediacastnet/platform/blob/main/docs/architecture/mediacast-platform-blueprint.md)
//! (vendor-specific protocols enumerated there) plus
//! [`platform-architecture-2026-04.md`](https://github.com/Mediacastnet/platform/blob/main/docs/architecture/platform-architecture-2026-04.md)
//! §10.2 (the platform's `send_lan` / `send_cec` / `send_rs232` command
//! taxonomy) and the
//! [Display connectivity tiers memory](https://github.com/Mediacastnet/platform)
//! (HDMI-CEC reliable only for power-on/off cross-vendor).
//!
//! No display-control runtime code exists in the platform repo today;
//! the vocabulary will be re-validated against MediaControl's actual
//! usage when MediaControl reaches implementation. Expect minor
//! additions/removals at v0.2.

use serde::{Deserialize, Serialize};

/// Abstract command vocabulary. Each variant maps to a concrete
/// vendor-specific payload (Samsung MDC byte string, LG webOS SSAP
/// JSON, Sony PSI URL, PJLink ASCII, …) per `(vendor, firmware
/// version)` in the catalog YAML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum CommandType {
    // ── Power ────────────────────────────────────────────────────────
    /// Power the display on.
    PowerOn,
    /// Power the display off (typically standby, not unplugged).
    PowerOff,
    /// Read current power state.
    PowerStatus,

    // ── Input source ─────────────────────────────────────────────────
    /// Switch the active input (HDMI 1, HDMI 2, DisplayPort, etc.).
    InputSelect,
    /// List the inputs the display advertises.
    InputList,
    /// Read the currently active input.
    InputCurrent,

    // ── Audio ────────────────────────────────────────────────────────
    /// Read current volume (0–100 normalized; vendor scale handled in catalog).
    VolumeGet,
    /// Set the volume.
    VolumeSet,
    /// Toggle mute state.
    MuteToggle,

    // ── Picture ──────────────────────────────────────────────────────
    /// Read current brightness (0–100 normalized).
    BrightnessGet,
    /// Set the brightness.
    BrightnessSet,
    /// Read current contrast (0–100 normalized).
    ContrastGet,
    /// Set the contrast.
    ContrastSet,
    /// Set the picture mode (Standard, Vivid, Cinema, Sport, Game, …).
    PictureMode,

    // ── Scheduling / lifecycle ───────────────────────────────────────
    /// Configure / read the sleep timer (auto-power-off after N minutes).
    SleepTimer,
    /// Configure the on/off schedule (vendor-specific shape).
    AutoOffSchedule,

    // ── Screen state (signage primitives) ────────────────────────────
    /// Blank / black-out the screen without powering off (signage takeover).
    ScreenBlank,
    /// Restore from blank.
    ScreenUnblank,

    // ── Identity / inventory ─────────────────────────────────────────
    /// Read stable hardware identity (model, serial, MAC).
    HardwareIdentity,
    /// Read firmware version. **Critical for the catalog's own selection logic.**
    FirmwareVersion,

    // ── Telemetry ────────────────────────────────────────────────────
    /// Read panel temperature (vendor-exposed; otherwise NOT_SUPPORTED).
    PanelTemperature,
    /// Read total operating hours / panel lifetime hours.
    OperatingHours,

    // ── Network / power-cycle ────────────────────────────────────────
    /// Send Wake-on-LAN magic packet to power on a sleeping/off display.
    WakeOnLan,
    /// Soft-reboot the display's controller.
    Reboot,
    /// Factory reset (gated; dangerous; document the consequences in `notes`).
    FactoryReset,
}

impl CommandType {
    /// Iterate over every defined command type. Useful for catalog
    /// completeness checks.
    pub fn all() -> &'static [CommandType] {
        &[
            CommandType::PowerOn,
            CommandType::PowerOff,
            CommandType::PowerStatus,
            CommandType::InputSelect,
            CommandType::InputList,
            CommandType::InputCurrent,
            CommandType::VolumeGet,
            CommandType::VolumeSet,
            CommandType::MuteToggle,
            CommandType::BrightnessGet,
            CommandType::BrightnessSet,
            CommandType::ContrastGet,
            CommandType::ContrastSet,
            CommandType::PictureMode,
            CommandType::SleepTimer,
            CommandType::AutoOffSchedule,
            CommandType::ScreenBlank,
            CommandType::ScreenUnblank,
            CommandType::HardwareIdentity,
            CommandType::FirmwareVersion,
            CommandType::PanelTemperature,
            CommandType::OperatingHours,
            CommandType::WakeOnLan,
            CommandType::Reboot,
            CommandType::FactoryReset,
        ]
    }
}
