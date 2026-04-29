//! # mediacast-displaycatalog
//!
//! Vendor command catalog + version matcher + protocol probe for
//! IP-managed displays.
//!
//! Sister crate to [`mediacast-netcatalog`](https://github.com/Mediacastnet/mediacast-netcatalog) —
//! same architectural shape, different vendor data domain. Where
//! `mediacast-netcatalog` covers network switches (Cisco IOS-XE, Aruba
//! AOS-CX, Juniper Junos, …), this crate covers professional displays
//! (Samsung MDC, LG webOS, NEC NaViSet, Sony Bravia, PJLink projectors).
//!
//! See the [README](https://github.com/Mediacastnet/mediacast-displaycatalog)
//! for an overview and the `catalog/` directory for the YAML data files.
//!
//! ## Status
//!
//! v0.1 — scaffold. The catalog YAML files are placeholder skeletons;
//! real vendor command data lands when the doc-crawl effort runs. The
//! Rust API is unstable until v0.2.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod catalog;
pub mod command_types;
pub mod error;
pub mod version;

#[cfg(feature = "bin")]
pub mod probe;

#[cfg(feature = "python")]
mod python;

pub use catalog::{Catalog, CommandEntry, ProtocolAlternatives, VendorFile};
pub use command_types::CommandType;
pub use error::{Error, Result};
pub use version::{FirmwareVersion, VersionRange};
