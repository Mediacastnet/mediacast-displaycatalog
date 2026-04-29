# Vendor Command Catalog — Status

Roll-up of research progress per vendor. Updated as each `<vendor>.yaml`
file lands.

## Coverage matrix

| Vendor | File | Status | Coverage | Notes |
|---|---|---|---|---|
| Samsung MDC | [`samsung-mdc.yaml`](samsung-mdc.yaml) | ⚪ Skeleton | 0/25 | Vendor identity populated; MDC binary protocol slot reserved. Awaits doc-crawl. |
| LG webOS Signage | [`lg-webos.yaml`](lg-webos.yaml) | ⚪ Skeleton | 0/25 | Vendor identity populated; SSAP slot reserved. Awaits doc-crawl. |
| Sony Bravia Professional | [`sony-bravia.yaml`](sony-bravia.yaml) | ⚪ Skeleton | 0/25 | Vendor identity populated; PSI slot reserved. Awaits doc-crawl. |
| NEC NaViSet | [`nec-naviset.yaml`](nec-naviset.yaml) | ⚪ Skeleton | 0/25 | Vendor identity populated; REST + NaViSet binary slots reserved. Awaits doc-crawl. |
| PJLink (cross-vendor projector standard) | [`pjlink-projectors.yaml`](pjlink-projectors.yaml) | ⚪ Skeleton | 0/25 | Vendor identity populated; PJLink standardized command surface reserved. Awaits doc-crawl. |

## Status legend

- ⚪ Not started / skeleton only
- 🟡 In progress
- 🟢 Complete
- ⚠️ Partial (some types missing or unverified)
- 🔴 Blocked

## Coverage breakdown

**v0.1 ships every vendor file as a placeholder skeleton** — vendor
identity populated, `commands: []` empty, protocol-capability slot
reserved. Real catalog data lands when the doc-crawl effort runs
against vendor documentation (Samsung MDC Protocol Reference, LG webOS
SSAP API docs, Sony Bravia IP Control reference, NEC External Control
manuals, JBMIA PJLink specification).

Aggregate (v0.1): **0 of 125 vendor-command pairs populated** (5 × 25).

## What v0.2 should land

- **Samsung MDC** — full 25/25 with real command bytes (0x11 power,
  0x14 input, 0x12 volume, 0x36 brightness, 0x37 contrast, …) and
  request/response frame layouts.
- **LG webOS SSAP** — full 25/25 with real SSAP URIs (`ssap://system/turnOff`,
  `ssap://com.webos.applicationManager/launch`, `ssap://audio/setVolume`, …).
- **Sony Bravia PSI** — full 25/25 with JSON-RPC method names per service
  (`system.setPowerStatus`, `videoScreen.setPictureQualitySettings`,
  `audio.setAudioVolume`, …) plus IRCC fallbacks for legacy commands.
- **NEC NaViSet** — full 25/25 with REST endpoints + binary fallback
  for legacy panels.
- **PJLink** — full 25/25 with standardized commands (POWR / INPT / AVMT
  / INST / NAME / INF1 / INF2 / INFO / CLSS / ERST / LAMP), class noted.

Cross-vendor synthesis lands in [`FINDINGS.md`](FINDINGS.md) when at
least three of the five files have populated commands.

## Last updated

2026-04-29 — repository scaffold + skeleton vendor files.
