# Vendor Command Catalog — YAML Schema

Each `<vendor>.yaml` file follows this shape. The schema mirrors the
shape used by [`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog),
adapted for the IP-managed display domain (different protocol slots,
different command vocabulary).

```yaml
# ── Vendor identity ────────────────────────────────────────────────
vendor: samsung_mdc          # Stable id; snake_case.
display_name: "Samsung MDC"
manufacturer: "Samsung Electronics"
product_family: "Commercial / SSSP / Tizen displays"
notes: |                     # Free-form notes (multi-line ok).
  Samsung's Multiple Display Control (MDC) is a binary protocol over
  TCP/1515 (or RS-232 on legacy panels). Frames carry a header byte
  (0xAA), command id, display id, length, data, and a checksum.

# ── Documentation sources ──────────────────────────────────────────
# Where the data in this file was extracted from.
sources:
  - title: "MDC Protocol Reference"
    url: "..."
    accessed: "2026-04-29"

# ── Protocol capabilities ──────────────────────────────────────────
# Per-protocol availability metadata. Display vendors don't speak
# NETCONF/RESTCONF/gNMI; the slots here are the actually-deployed
# IP-managed-display protocols.
protocol_capabilities:
  mdc:
    introduced_in: "..."
    notes: "..."
  webos_ssap: null
  bravia_psi: null
  pjlink: null
  cec:
    notes: "AnyNet+ implementation; reliable cross-vendor only for power on/off."
  rest_api: null

# ── Commands ───────────────────────────────────────────────────────
# Each entry maps an abstract command type (from COMMAND_TYPES.md) to
# its concrete vendor implementation + protocol alternatives.
commands:
  - type: POWER_ON
    description: "Power the display on (from standby)"
    versions:
      - applies_to: ">=2020"
        cli: "AA:11:FE:01:01:11"     # MDC frame as colon-separated hex
        sample_output: "AA:FF:01:03:A:01:11:0F"  # ack frame
        parser_notes: "Bytes 0..1 ack; byte 5 echoes command id."
        config_required: ""
        notes: ""
    protocol_alternatives:
      mdc:
        command_id: "0x11"
        request_frame: "AA 11 FE 01 01 11"
        response_frame: "AA FF 01 03 A 01 11 0F"
      cec:
        opcode: "IMAGE_VIEW_ON"
        opcode_byte: "0x04"
        cec_version: "1.4"
        cross_vendor_note: "Reliable across vendors per blueprint §8.7."
```

## Field semantics

### `vendor` (string)
Stable identifier in snake_case (`samsung_mdc`, `lg_webos`,
`sony_bravia`, `nec_naviset`, `pjlink`).

### `versions[].applies_to` (version range expression)
Same SemVer-flavored range syntax as `mediacast-netcatalog`:

- `">=2020"` — version >= 2020
- `">=2020,<2024"` — comma-separated AND (range)
- `"*"` — any version (use when version awareness genuinely doesn't
  matter for this command on this vendor)

When multiple `versions` blocks match, the **most specific** wins
(narrower range > wider range).

For PJLink, version awareness is class-based not date-based; use
`applies_to: "*"` and rely on the per-protocol `class` field.

### `versions[].cli` (string)
The concrete payload. Encoding depends on the protocol:

- **MDC** — colon-separated hex bytes (`AA:11:FE:01:01:11`)
- **PJLink** — ASCII command (`%1POWR 1`)
- **webOS SSAP** — JSON (`{"id":"0","type":"request","uri":"ssap://system/turnOff"}`)
- **Bravia PSI** — URL + JSON params, or IRCC opcode for legacy
- **CEC** — opcode mnemonic; the byte goes in `opcode_byte`
- **REST** — full URL path with method prefix (`POST /api/v1/power`)

Use `NOT_SUPPORTED` for commands the vendor genuinely doesn't expose
(e.g. NaViSet has no `OPERATING_HOURS` query on certain product lines).

### `versions[].sample_output` (string)
Real-world response. **Not invented** — pulled from vendor docs or real
captures. Mark as `unverified: true` if the source is heuristic.

### `versions[].config_required` (string, multi-line)
Configuration the device must have for this command to work. Examples:
- `Network Standby ON` (Samsung) for power-on commands to reach a fully-off panel
- `External Control enabled` (NEC NaViSet)
- `External Input authentication: PSK` (Sony Bravia)

### `protocol_alternatives` (object)
Per-protocol mapping when the same data is available via more than one
protocol. Use `null` for protocols where this command type isn't exposed.

Slots:

| Slot | Protocol | Used by |
|---|---|---|
| `mdc` | Samsung MDC binary over TCP/1515 (or RS-232) | Samsung |
| `webos_ssap` | LG webOS Secure Second Screen JSON-over-WebSocket on TCP/3000 (3001 secure) | LG webOS |
| `bravia_psi` | Sony Bravia Professional Display Simple IP Control HTTP / IRCC | Sony |
| `pjlink` | PJLink ASCII over TCP/4352 (cross-vendor projector standard) | All conforming projectors |
| `cec` | HDMI-CEC opcodes over the HDMI cable | All HDMI-equipped displays (last-mile) |
| `rest_api` | Generic vendor REST/HTTP | NEC NaViSet, others |

NETCONF / RESTCONF / gNMI are intentionally **absent** — those are
network-switch protocols, served by the sister crate
[`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog).

## Validation expectations

A complete catalog file:

- Has all command types from [`COMMAND_TYPES.md`](COMMAND_TYPES.md).
  Use `"NOT_SUPPORTED"` as the `cli` value when the vendor genuinely
  doesn't expose the operation; document why in `notes`.
- Has at least one `versions` block per command type. Multiple blocks
  for firmware-version-sensitive commands.
- Has citations in `sources` for the firmware versions covered.
- Has `sample_output` populated where output-parsing is involved.

v0.1 ships placeholder skeletons — `commands: []` is the intentional
v0.1 state until the doc-crawl effort lands real data.
