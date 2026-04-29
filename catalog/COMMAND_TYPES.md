# Vendor Command Catalog — Abstract Command Types

The abstract types each `<vendor>.yaml` file should cover. The starting
set was extracted from Mediacast platform planning docs:
[`mediacast-platform-blueprint.md`](https://github.com/Mediacastnet/platform/blob/main/docs/architecture/mediacast-platform-blueprint.md),
[`platform-architecture-2026-04.md`](https://github.com/Mediacastnet/platform/blob/main/docs/architecture/platform-architecture-2026-04.md)
§10.2 (`send_lan` / `send_cec` / `send_rs232` taxonomy), and the
display-connectivity-tiers vertical work (HDMI-CEC reliable only for
power-on/off cross-vendor).

Use these exact identifiers (uppercase + underscores) as the `type`
field in `<vendor>.yaml::commands`.

## Power

| Type | Description |
|---|---|
| `POWER_ON` | Power the display on (from standby). For fully-off panels, see `WAKE_ON_LAN`. |
| `POWER_OFF` | Power the display off into standby. |
| `POWER_STATUS` | Read current power state (on / standby / off / warming up / cooling down). |

## Input source

| Type | Description |
|---|---|
| `INPUT_SELECT` | Switch the active input (HDMI 1, HDMI 2, DisplayPort, OPS, …). Argument: input identifier. |
| `INPUT_LIST` | List the inputs the display advertises. |
| `INPUT_CURRENT` | Read the currently active input. |

## Audio

| Type | Description |
|---|---|
| `VOLUME_GET` | Read current volume. Returned value normalized to 0–100; vendor scale handled in catalog. |
| `VOLUME_SET` | Set the volume. Argument: 0–100. |
| `MUTE_TOGGLE` | Toggle mute state. |

## Picture

| Type | Description |
|---|---|
| `BRIGHTNESS_GET` | Read current brightness (0–100 normalized). |
| `BRIGHTNESS_SET` | Set the brightness. Argument: 0–100. |
| `CONTRAST_GET` | Read current contrast (0–100 normalized). |
| `CONTRAST_SET` | Set the contrast. Argument: 0–100. |
| `PICTURE_MODE` | Set the picture mode. Argument: vendor-recognized name (Standard, Vivid, Cinema, Sport, Game, Calibrated, …). Some vendors expose a numeric mode index instead — handled in catalog. |

## Scheduling / lifecycle

| Type | Description |
|---|---|
| `SLEEP_TIMER` | Configure / read the sleep timer (auto-power-off after N minutes). |
| `AUTO_OFF_SCHEDULE` | Configure the on/off schedule. Vendor-specific shape (Samsung's "On/Off Timer" vs. LG's "Schedule"). |

## Screen state (signage primitives)

| Type | Description |
|---|---|
| `SCREEN_BLANK` | Blank / black-out the screen without powering off. Used for signage takeover. |
| `SCREEN_UNBLANK` | Restore from blank. |

## Identity / inventory

| Type | Description |
|---|---|
| `HARDWARE_IDENTITY` | Stable hardware identity: model, serial, MAC. Composite — typically a few queries combined. |
| `FIRMWARE_VERSION` | Firmware version string. **Critical for the catalog itself** — used for version-aware selection. |

## Telemetry

| Type | Description |
|---|---|
| `PANEL_TEMPERATURE` | Read panel temperature, when the vendor exposes it. Many do not — `NOT_SUPPORTED` is fine. |
| `OPERATING_HOURS` | Total operating hours / panel lifetime hours. |

## Network / power-cycle

| Type | Description |
|---|---|
| `WAKE_ON_LAN` | Send Wake-on-LAN magic packet to power on a sleeping/off display. Some panels need this (and only this) when fully powered off — TCP-based protocols can't reach a panel that isn't running its NIC. |
| `REBOOT` | Soft-reboot the display's controller. |
| `FACTORY_RESET` | Factory reset. **Gated; dangerous.** Document the consequences in `notes`. Do not invoke in routine operations. |

## Coverage checklist

A complete `<vendor>.yaml` has an entry for **every** type above (25
total). When the vendor genuinely doesn't expose an operation (e.g.
PJLink class 1 doesn't expose `PANEL_TEMPERATURE`; NaViSet legacy
panels lack `OPERATING_HOURS`), use `cli: "NOT_SUPPORTED"` and explain
in `notes`.

## Variables in `cli` strings

Recognized placeholders for parameterized commands:

| Placeholder | Meaning |
|---|---|
| `{value}` | A single integer value (volume, brightness, contrast). |
| `{input}` | Input identifier in the vendor's preferred form. |
| `{minutes}` | Sleep-timer duration in minutes. |
| `{mac}` | MAC address (for WoL) in the vendor's preferred format. |
| `{psk}` | Pre-shared key (Sony Bravia auth). |

For commands that take no parameters, no placeholders. Keep the string
literal.

## Re-validation against MediaControl

The starting set above will be re-validated against MediaControl's
actual usage when MediaControl reaches implementation. Expect minor
additions (e.g. `STANDBY_NETWORK` for fully-off-vs-standby distinction
on Samsung) or removals (e.g. `OPERATING_HOURS` may move to a
vendor-extension slot if too few vendors expose it). The schema is
designed to make these additions/removals additive — `#[non_exhaustive]`
on the Rust enum, named slots in the YAML.
