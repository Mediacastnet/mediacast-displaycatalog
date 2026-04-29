//! Version-matcher tests. These are a near-copy of mediacast-netcatalog's
//! suite because the matcher is borrowed verbatim. Display-vendor-specific
//! firmware-string corners (Samsung Tizen prefixes, Sony PKG suffix, …)
//! get added in v0.2 when real catalog data lands.

use mediacast_displaycatalog::{FirmwareVersion, VersionRange};

#[test]
fn parses_plain_semver() {
    let fw = FirmwareVersion::parse("5.5.0").unwrap();
    assert_eq!((fw.major, fw.minor, fw.patch), (5, 5, 0));
    assert_eq!(fw.family_prefix, None);
}

#[test]
fn aoscx_family_prefix_still_works() {
    // Borrowed-verbatim sanity check: the FL./GL./LL. logic is dead code
    // for the display domain but must not regress the netcatalog parity.
    let fw = FirmwareVersion::parse("FL.10.13.1000").unwrap();
    assert_eq!(fw.family_prefix.as_deref(), Some("FL"));
    assert_eq!(fw.major, 10);
    assert_eq!(fw.patch, 1000);
}

#[test]
fn range_specificity_ordering() {
    let wide = VersionRange::parse(">=2020").unwrap();
    let narrow = VersionRange::parse(">=2020,<2024").unwrap();
    assert!(narrow.specificity() > wide.specificity());
}

#[test]
fn wildcard_matches_anything() {
    let r = VersionRange::parse("*").unwrap();
    assert!(r.matches(&FirmwareVersion::parse("1.0.0").unwrap()));
    assert!(r.matches(&FirmwareVersion::parse("5.5.0").unwrap()));
}

#[test]
fn ge_lt_range_matches_inside() {
    let r = VersionRange::parse(">=5.0,<7.0").unwrap();
    assert!(r.matches(&FirmwareVersion::parse("5.0.0").unwrap()));
    assert!(r.matches(&FirmwareVersion::parse("6.5.4").unwrap()));
    assert!(!r.matches(&FirmwareVersion::parse("7.0.0").unwrap()));
    assert!(!r.matches(&FirmwareVersion::parse("4.9.99").unwrap()));
}
