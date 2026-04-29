# Contributing to mediacast-displaycatalog

Thanks for your interest. This project is **research-grade data + a Rust
crate** maintained by [Mediacast Network Solutions](https://github.com/Mediacastnet).
The first production consumers are the Mediacast platform's display-control
services and MediaControl (planned).

This is a sister crate to
[`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog) —
same architectural shape, different vendor data domain. AV integrators
hit the same Samsung-MDC-vs-LG-webOS-vs-Sony-Bravia protocol divergences
that network engineers hit with Cisco-vs-Aruba-vs-Juniper. This catalog
is the data layer that lets a runtime pick the right command for the
right `(vendor, firmware)` instead of guessing.

## What we want

### Catalog contributions (highest value)

- **Real vendor command data** — v0.1 ships placeholder skeletons.
  Pulling actual Samsung MDC command-id bytes, LG webOS SSAP URIs, Sony
  Bravia PSI JSON-RPC method names, NEC NaViSet REST endpoints, or
  standardized PJLink commands from vendor documentation is the
  highest-impact contribution. See `catalog/SCHEMA.md` for the file
  shape.
- **Additional vendors** — Sharp displays, Panasonic Professional,
  Philips MMD, BenQ Interactive Displays, ViewSonic, Optoma projectors.
  Aim for ≥80% of the 25 abstract command types. Cite vendor docs in
  the `sources:` block.
- **Real-gear validation** — take an entry marked `unverified: true`,
  run it against actual hardware, capture the real response, and PR a
  fix that removes the flag.
- **Probe protocol additions** — v0.1 stubs MDC/SSAP/PSI/PJLink/CEC;
  HDBaseT control, Crestron/AMX integration, BACnet for facility
  systems would round it out.

### Crate contributions

- **Probe implementation** — v0.1 leaves the probe as a stub. Wiring up
  TCP/1515 (MDC), TCP/3000 (webOS), HTTP (Bravia), TCP/4352 (PJLink),
  HDMI-CEC discovery via local adapter is the v0.2 deliverable.
- **Version-matcher edge cases** — display vendors have weirder
  firmware strings than switches (Samsung Tizen `T-KTM2DEUC`, Sony
  Bravia `PKG6.2940.0030NAA`). If your vendor's firmware string format
  doesn't parse cleanly, send a failing test + the fix.

### What we don't want

- Drive-by formatting or rename PRs.
- New abstract command types without a real-world consumer requesting
  them. The 25 we have are a starting set extracted from Mediacast
  platform planning docs and will be re-validated against MediaControl
  when MediaControl reaches implementation.
- Catalog entries with invented data. If you don't have vendor docs or
  real-gear capture for it, open an issue instead of guessing.

## Catalog YAML conventions

See `catalog/SCHEMA.md` for the full spec. Quick rules:

- Every entry **must** cite its source. URL + access date in the
  `sources:` block.
- For commands the vendor genuinely doesn't expose, set `cli:
  "NOT_SUPPORTED"` and document why in `notes`.
- Mark `unverified: true` if the data was extracted from heuristic /
  community sources rather than vendor docs.
- For binary protocols (MDC), use colon-separated hex bytes
  (`AA:11:FE:01:01:11`). For ASCII protocols (PJLink), the literal
  command. For JSON (SSAP, Bravia PSI), the request payload as a
  string. The per-vendor file's `notes` documents the encoding.

## Development setup

```bash
git clone https://github.com/Mediacastnet/mediacast-displaycatalog
cd mediacast-displaycatalog

# Rust
cargo test --no-default-features
cargo run --example basic_lookup

# Python bindings
pip install "maturin>=1.5,<2.0"
maturin develop --features python
python -c "from mediacast_displaycatalog import Catalog; print(Catalog.load_bundled().vendors())"
```

## PR process

1. Open an issue first for substantive changes (new vendor, schema
   change, breaking API). For obvious fixes, just send the PR.
2. Run `cargo fmt`, `cargo clippy`, `cargo test`, and `yamllint catalog/`
   before submitting.
3. Add an entry to `CHANGELOG.md` under `[Unreleased]`.
4. CI must pass on Linux + macOS + Windows.

## License

By contributing, you agree your contribution is dual-licensed under
MIT or Apache-2.0, matching the rest of the project. See `LICENSE-MIT`
and `LICENSE-APACHE`.

## Code of conduct

Be kind. Disagree on technical merit, not on people. Maintainers will
moderate at their discretion.
