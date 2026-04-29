# Vendor Command Catalog — Cross-vendor Findings

Synthesis of cross-vendor research patterns. Lands here when the
doc-crawl effort populates at least three of the five v0.1 vendor
files with real command data.

**v0.1 status: research not yet performed.** This file is a stub.

When the doc-crawl effort runs, the following questions are pre-loaded
for the synthesis:

1. **Cross-protocol command coverage** — which abstract command types
   are universally available across all five vendors, and which are
   vendor-specific? Hypothesis: power on/off and input select are
   universal; panel temperature and operating hours are vendor-specific.
2. **PJLink as common denominator** — for projectors specifically, does
   PJLink class 1 cover enough of the abstract command set that
   per-vendor projector files are redundant? Or do Epson/Panasonic/etc.
   need their own files for vendor-extensions?
3. **HDMI-CEC reliability matrix** — beyond the platform's documented
   "only power on/off is reliable cross-vendor" finding, are there
   command types where CEC is *more* reliable than vendor-specific
   protocols (e.g. is `STANDBY` opcode more universally honored than
   any vendor's "soft power" command)?
4. **Authentication patterns** — Samsung MDC has none; LG webOS SSAP
   uses a paired client-key; Sony Bravia uses a PSK header; NEC's
   newer panels use HTTP basic auth. Document the pattern; flag the
   security implications.
5. **Network Standby gotchas** — most vendors require an opt-in setting
   to keep the NIC alive when the panel is off. Without it, only
   Wake-on-LAN can reach a fully-off display. Confirm which vendors
   require this and document the setting name.
6. **Volume-scale normalization** — Samsung accepts 0–100; Sony
   typically 0–100 but the IRCC fallback uses 0–100 quantized; LG webOS
   accepts 0–100; NEC varies by product line. Confirm the normalization
   rule.
7. **Picture-mode taxonomy** — every vendor has a different list of
   picture modes (Samsung: Standard/Dynamic/Movie/Game; LG: Vivid/
   Standard/Cinema/Sports/Game; Sony: Vivid/Standard/Cinema/Game/
   Calibrated). Decide whether to normalize to a common set or pass
   through vendor-specific names.
