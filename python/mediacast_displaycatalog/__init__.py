"""mediacast-displaycatalog: vendor command catalog + version matcher + protocol probe for IP-managed displays.

This is a thin Python re-export of the Rust core. The native extension lives
in ``mediacast_displaycatalog._native``; the public API is here.
"""

from __future__ import annotations

from enum import Enum

from ._native import Catalog, __version__

__all__ = ["Catalog", "CommandType", "__version__"]


class CommandType(str, Enum):
    """Abstract command vocabulary. Mirrors the Rust ``CommandType`` enum."""

    POWER_ON = "POWER_ON"
    POWER_OFF = "POWER_OFF"
    POWER_STATUS = "POWER_STATUS"
    INPUT_SELECT = "INPUT_SELECT"
    INPUT_LIST = "INPUT_LIST"
    INPUT_CURRENT = "INPUT_CURRENT"
    VOLUME_GET = "VOLUME_GET"
    VOLUME_SET = "VOLUME_SET"
    MUTE_TOGGLE = "MUTE_TOGGLE"
    BRIGHTNESS_GET = "BRIGHTNESS_GET"
    BRIGHTNESS_SET = "BRIGHTNESS_SET"
    CONTRAST_GET = "CONTRAST_GET"
    CONTRAST_SET = "CONTRAST_SET"
    PICTURE_MODE = "PICTURE_MODE"
    SLEEP_TIMER = "SLEEP_TIMER"
    AUTO_OFF_SCHEDULE = "AUTO_OFF_SCHEDULE"
    SCREEN_BLANK = "SCREEN_BLANK"
    SCREEN_UNBLANK = "SCREEN_UNBLANK"
    HARDWARE_IDENTITY = "HARDWARE_IDENTITY"
    FIRMWARE_VERSION = "FIRMWARE_VERSION"
    PANEL_TEMPERATURE = "PANEL_TEMPERATURE"
    OPERATING_HOURS = "OPERATING_HOURS"
    WAKE_ON_LAN = "WAKE_ON_LAN"
    REBOOT = "REBOOT"
    FACTORY_RESET = "FACTORY_RESET"
