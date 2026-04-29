//! Protocol-capability probe.
//!
//! Fingerprints which display-control protocols a real device exposes —
//! Samsung MDC (TCP/1515), LG webOS SSAP (TCP/3000 or 3001), Sony Bravia
//! PSI (HTTP/80), PJLink (TCP/4352), HDMI-CEC (via local CEC adapter).
//!
//! Stdlib + `tokio` for async TCP. Implementation lands in v0.2 — this
//! file is the stable signature contract.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;

/// One device's probe report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeReport {
    /// Target host (DNS or IP).
    pub host: String,
    /// Vendor identifier the probe was run with.
    pub vendor: String,
    /// True if Samsung MDC TCP/1515 accepted a connection + a probe frame.
    pub mdc_available: bool,
    /// True if LG webOS SSAP WebSocket on TCP/3000 (or 3001 secure) responded.
    pub webos_ssap_available: bool,
    /// True if Sony Bravia PSI HTTP /sony/system endpoint returned JSON.
    pub bravia_psi_available: bool,
    /// True if PJLink TCP/4352 returned a class-1 or class-2 challenge.
    pub pjlink_available: bool,
    /// PJLink class advertised (1 or 2), if reachable.
    pub pjlink_class: Option<u8>,
    /// Parsed firmware version, if extractable from any probe response.
    pub firmware: Option<String>,
}

/// Per-probe configuration.
#[derive(Debug, Clone)]
pub struct ProbeConfig {
    /// Connect timeout per protocol.
    pub timeout: Duration,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}

/// Run all enabled probes against a target. Implementation lands in v0.2 —
/// this signature is the stable contract.
#[cfg(feature = "bin")]
pub async fn probe_device(_host: &str, _vendor: &str, _cfg: &ProbeConfig) -> Result<ProbeReport> {
    todo!("probe implementation lands in v0.2")
}

/// Convenience helper.
#[allow(dead_code)]
fn parse_endpoint(host: &str, port: u16) -> Option<SocketAddr> {
    use std::net::ToSocketAddrs;
    (host, port).to_socket_addrs().ok()?.next()
}
