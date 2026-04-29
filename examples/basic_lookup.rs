//! Minimal usage example. v0.1 ships placeholder catalog skeletons, so
//! `catalog.lookup` will return `Ok(None)` for every command — the
//! example demonstrates the loader + vendor enumeration shape, and the
//! lookup call returns `(no entry)` until real data lands.
//!
//! ```bash
//! cargo run --example basic_lookup
//! ```

use mediacast_displaycatalog::{Catalog, CommandType};

fn main() -> anyhow::Result<()> {
    let catalog = Catalog::load_bundled()?;

    println!("Vendors loaded:");
    for v in catalog.vendors() {
        println!("  - {}", v);
    }
    println!();

    let cases = [
        ("samsung_mdc", "5.0.0", CommandType::PowerOn),
        ("lg_webos", "5.5.0", CommandType::InputSelect),
        ("sony_bravia", "6.2940", CommandType::VolumeSet),
        ("nec_naviset", "1.0.0", CommandType::FirmwareVersion),
        ("pjlink", "1", CommandType::PowerStatus),
    ];

    for (vendor, fw, cmd) in cases {
        match catalog.lookup(vendor, fw, cmd) {
            Ok(Some(entry)) => println!("{:>14} {:>10} {:?} → {}", vendor, fw, cmd, entry.cli),
            Ok(None) => println!(
                "{:>14} {:>10} {:?} → (no entry — v0.1 placeholder skeleton)",
                vendor, fw, cmd
            ),
            Err(e) => println!("{:>14} {:>10} {:?} → ERR: {}", vendor, fw, cmd, e),
        }
    }

    Ok(())
}
