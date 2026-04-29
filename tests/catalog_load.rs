//! Smoke test: every bundled vendor file loads. v0.1 ships placeholder
//! skeletons (`commands: []`), so the per-command-type-coverage assertion
//! lives in netcatalog's analogous test but not here — until catalog
//! data lands, only the load+parse path is meaningful.

use mediacast_displaycatalog::Catalog;

#[test]
fn bundled_catalog_loads() {
    let catalog = Catalog::load_bundled().expect("bundled catalog must load");
    let vendors: Vec<&str> = catalog.vendors().collect();
    assert!(
        !vendors.is_empty(),
        "bundled catalog has at least one vendor"
    );

    let expected = [
        "samsung_mdc",
        "lg_webos",
        "sony_bravia",
        "nec_naviset",
        "pjlink",
    ];
    for v in expected {
        assert!(
            vendors.contains(&v),
            "expected vendor '{}' present (have {:?})",
            v,
            vendors,
        );
    }
}
