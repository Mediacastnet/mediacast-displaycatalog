# mediacast-displaycatalog

**Vendor command catalog + version matcher + protocol probe for
IP-managed displays.** Rust core; Python bindings via PyO3.

[![Crates.io](https://img.shields.io/crates/v/mediacast-displaycatalog.svg)](https://crates.io/crates/mediacast-displaycatalog)
[![PyPI](https://img.shields.io/pypi/v/mediacast-displaycatalog.svg)](https://pypi.org/project/mediacast-displaycatalog/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

## What this is

A YAML-backed library that maps **abstract command types** (e.g.
`POWER_ON`, `INPUT_SELECT`, `VOLUME_SET`, `BRIGHTNESS_SET`) to
**concrete vendor payloads (Samsung MDC byte frames, LG webOS SSAP
JSON, Sony Bravia PSI JSON-RPC, PJLink ASCII, HDMI-CEC opcodes), parser
hints, and protocol alternatives** — selectable by `(vendor, firmware
version)`.

It's a sister crate to
[`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog) —
same architectural shape, different vendor data domain (IP-managed
displays instead of network switches).

## Why

Every AV integrator hits the same protocol divergences. Shipping a
single command across a venue full of mixed displays — say, "turn off
all signage at curfew" — means knowing that:

- **Samsung MDC** wants a binary frame on TCP/1515 with a header byte
  `0xAA`, command id `0x11`, and a checksum trailer.
- **LG webOS Signage** wants a JSON payload on a paired SSAP WebSocket
  with the URI `ssap://system/turnOff`.
- **Sony Bravia Professional** wants a JSON-RPC POST to `/sony/system`
  with a PSK auth header and the method `setPowerStatus`.
- **NEC NaViSet** wants a binary command over TCP/7142 (or RS-232 on
  legacy panels), with a different REST surface on modern panels.
- **PJLink projectors** want ASCII (`%1POWR 0`) on TCP/4352 — and
  thankfully this is the cross-vendor projector standard.
- **HDMI-CEC** can do power on/off cross-vendor — but only that;
  vendor-extension opcodes vary wildly.

This library is the **data layer** that lets a runtime pick the right
command for the right `(vendor, firmware)` instead of hand-rolling
each vendor's protocol in product code. It also ships a
**protocol-capability probe** (CLI tool + library) that fingerprints
which programmatic interfaces a real display actually exposes.

See [`catalog/SCHEMA.md`](catalog/SCHEMA.md) for the YAML format,
[`catalog/COMMAND_TYPES.md`](catalog/COMMAND_TYPES.md) for the abstract
command vocabulary, and [`catalog/STATUS.md`](catalog/STATUS.md) for
coverage progress per vendor.

## Status

**v0.1 — scaffold.** The crate structure is in place; vendor catalog
files ship as **placeholder skeletons** (vendor identity populated,
`commands: []` empty). Real catalog data lands when the doc-crawl
effort runs against vendor documentation.

**API is unstable until v0.2.** Coverage roadmap in
[`catalog/STATUS.md`](catalog/STATUS.md). Cross-vendor synthesis lands
in [`catalog/FINDINGS.md`](catalog/FINDINGS.md) once at least three of
the five v0.1 files have populated commands.

## Quick start (Rust)

```toml
[dependencies]
mediacast-displaycatalog = "0.1"
```

```rust
use mediacast_displaycatalog::{Catalog, CommandType};

fn main() -> anyhow::Result<()> {
    let catalog = Catalog::load_bundled()?;            // ships embedded YAML
    // v0.1 returns Ok(None) — placeholder skeletons. v0.2 has real data.
    match catalog.lookup("samsung_mdc", "5.0.0", CommandType::PowerOn)? {
        Some(entry) => println!("payload: {}", entry.cli),
        None => println!("v0.1 placeholder; awaits doc-crawl"),
    }
    Ok(())
}
```

## Quick start (Python)

```bash
pip install mediacast-displaycatalog
```

```python
from mediacast_displaycatalog import Catalog, CommandType

catalog = Catalog.load_bundled()
print(catalog.vendors())   # ['samsung_mdc', 'lg_webos', 'sony_bravia', 'nec_naviset', 'pjlink']
```

The Python bindings re-export the same type vocabulary as the Rust
crate.

## Protocol probe

```bash
cargo install mediacast-displaycatalog --features bin
mediacast-displaycatalog probe --host 10.0.0.42 --vendor samsung_mdc
```

Or from Python (v0.2):

```python
from mediacast_displaycatalog.probe import probe_device
report = probe_device(host="10.0.0.42", vendor="samsung_mdc")
print(report.mdc_available, report.cec_available, report.firmware)
```

The probe uses **stdlib-only Rust** (no proprietary SDKs) — it issues
TCP connects + minimal protocol handshakes for MDC (1515), webOS SSAP
(3000 / 3001), Bravia PSI (HTTP), and PJLink (4352).

## Catalog as data

If you don't want a Rust dependency, just consume the YAML directly:

```bash
git clone https://github.com/Mediacastnet/mediacast-displaycatalog
cd mediacast-displaycatalog/catalog
ls *.yaml
```

The schema is documented in [`catalog/SCHEMA.md`](catalog/SCHEMA.md).
Files are pure YAML — load them with whatever tool you prefer.

## Project layout

```
mediacast-displaycatalog/
├── catalog/                       # Canonical YAML data + research docs
│   ├── samsung-mdc.yaml
│   ├── lg-webos.yaml
│   ├── sony-bravia.yaml
│   ├── nec-naviset.yaml
│   ├── pjlink-projectors.yaml
│   ├── SCHEMA.md
│   ├── COMMAND_TYPES.md
│   ├── STATUS.md
│   └── FINDINGS.md
├── src/                           # Rust core
│   ├── lib.rs
│   ├── catalog.rs                 # YAML → typed Catalog
│   ├── version.rs                 # Version range matcher (borrowed from netcatalog)
│   ├── command_types.rs           # CommandType enum
│   ├── error.rs
│   ├── probe.rs                   # Protocol-capability probe
│   ├── python.rs                  # PyO3 bindings (feature = "python")
│   └── bin/probe.rs               # CLI binary (feature = "bin")
├── examples/
│   └── basic_lookup.rs
├── tests/
│   ├── catalog_load.rs
│   └── version_matcher.rs
├── pyproject.toml                 # maturin build config
├── Cargo.toml
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE-MIT
└── LICENSE-APACHE
```

## Contributing

Contributions welcome — see [`CONTRIBUTING.md`](CONTRIBUTING.md). The
highest-value contribution today is **real vendor command data** —
v0.1 ships placeholder skeletons; pulling actual Samsung MDC command-id
bytes, LG webOS SSAP URIs, Sony Bravia PSI JSON-RPC methods, NEC
NaViSet endpoints, or standardized PJLink commands from vendor
documentation lands the crate at v0.2.

## Related projects

- **[mediacast-netcatalog](https://github.com/Mediacastnet/mediacast-netcatalog)** —
  sister crate for multi-vendor network switches (Cisco, Aruba, Juniper,
  Arista, HPE, Meraki). Same architectural shape; same matcher; different
  protocol slots.
- **[Mediacast Platform](https://github.com/Mediacastnet)** — broader
  Rust-first IPTV + signage platform from the same org. Display-control
  services consume this crate.

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE))
- MIT license ([`LICENSE-MIT`](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
