# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-04-29

### Added

- Initial scaffold. Sister crate to
  [`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog) —
  same architectural shape, different vendor data domain (IP-managed
  displays instead of network switches).
- Catalog YAML placeholder skeletons for five vendors:
  - Samsung MDC (`catalog/samsung-mdc.yaml`)
  - LG webOS Signage (`catalog/lg-webos.yaml`)
  - Sony Bravia Professional (`catalog/sony-bravia.yaml`)
  - NEC NaViSet (`catalog/nec-naviset.yaml`)
  - PJLink (cross-vendor projector standard) (`catalog/pjlink-projectors.yaml`)
- 25 abstract command types defined (see `catalog/COMMAND_TYPES.md`):
  power on/off/status, input select/list/current, volume get/set,
  mute toggle, brightness/contrast get/set, picture mode, sleep timer,
  auto-off schedule, screen blank/unblank, hardware identity, firmware
  version, panel temperature, operating hours, Wake-on-LAN, reboot,
  factory reset.
- Schema documented in `catalog/SCHEMA.md` with display-domain protocol
  slots: `mdc`, `webos_ssap`, `bravia_psi`, `pjlink`, `cec`, `rest_api`.
  Switch-domain protocols (NETCONF / RESTCONF / gNMI) intentionally
  absent — those live in the sister crate.
- Rust core scaffold:
  - `Catalog::load_bundled` + `Catalog::load_dir`
  - `FirmwareVersion` parser (borrowed verbatim from netcatalog;
    domain-agnostic SemVer-with-quirks shape)
  - `VersionRange` with `>=`, `<`, `,` (AND), `||` (OR), `*` (wildcard)
    and most-specific-wins matching
- PyO3 bindings under `mediacast_displaycatalog._native` (feature `python`).
- Stub `mediacast-displaycatalog probe` CLI binary (feature `bin`).
- CI: cargo fmt + clippy + test on stable/MSRV across Linux/macOS/Windows;
  maturin wheel build + smoke test on Python 3.9 + 3.12; yamllint on catalog.

### Status

**Research-stage.** Every catalog file ships as a placeholder skeleton
(vendor identity populated, `commands: []` empty). Real catalog data
lands when the doc-crawl effort runs against vendor documentation.
**API is unstable until v0.2.**

### Known limitations

- Catalog data not yet populated. v0.2 lands real Samsung MDC command
  bytes, LG webOS SSAP URIs, Sony Bravia PSI methods, NEC NaViSet
  commands, and standardized PJLink commands.
- Probe implementation is a stub. v0.2 wires up TCP/1515 (MDC) +
  TCP/3000 (webOS) + HTTP (Bravia) + TCP/4352 (PJLink) fingerprints.
- API is unstable. Expect breaking changes before v0.2.

[Unreleased]: https://github.com/Mediacastnet/mediacast-displaycatalog/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Mediacastnet/mediacast-displaycatalog/releases/tag/v0.1.0
