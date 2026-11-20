use std::collections::BTreeSet;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use crate::models::process_models::{DnsLookupResult, TcpPingResult};
use crate::utils::input_validator::{parsePortInput, validateHost, validateTimeoutMs};

// Phân giải tên miền sang danh sách địa chỉ IPv4/IPv6 bằng tuple chuẩn hỗ trợ cả IPv6
pub fn resolveHostAddresses(rawHost: &str) -> Result<DnsLookupResult, String> {
    let validHost = validateHost(rawHost)?;
    let startedAt = Instant::now();

    let socketAddrs = (validHost.as_str(), 0u16)
        .to_socket_addrs()
        .map_err(|err| format!("Lỗi phân giải DNS cho '{}': {}", validHost, err))?;

    let mut uniqueIps = BTreeSet::new();
    for addr in socketAddrs {
        uniqueIps.insert(addr.ip().to_string());
    }

    if uniqueIps.is_empty() {
        return Err(format!("Không tìm thấy bản ghi IP nào cho '{}'.", validHost));
    }

    Ok(DnsLookupResult {
        host: validHost,
        addresses: uniqueIps.into_iter().collect(),
        elapsedMs: startedAt.elapsed().as_millis(),
    })
}

// Kiểm tra khả năng kết nối TCP tới host:port và đo độ trễ phản hồi (ms)
pub fn checkTcpConnection(
    rawHost: &str,
    rawPort: &str,
    timeoutMs: u64,
) -> Result<TcpPingResult, String> {
    let validHost = validateHost(rawHost)?;
    let validPort = parsePortInput(rawPort)?;
    let validTimeout = validateTimeoutMs(timeoutMs)?;

    let mut resolvedAddrs: Vec<SocketAddr> = (validHost.as_str(), validPort)
        .to_socket_addrs()
        .map_err(|err| format!("Không thể phân giải '{}:{}': {}", validHost, validPort, err))?
        .collect();

    if resolvedAddrs.is_empty() {
        return Err(format!(
            "Không tìm thấy địa chỉ đích cho '{}:{}'.",
            validHost, validPort
        ));
    }

    resolvedAddrs.sort_by_key(|addr| if addr.is_ipv4() { 0 } else { 1 });
    let targetAddr = resolvedAddrs[0];

    let startedAt = Instant::now();
    let connectResult = TcpStream::connect_timeout(&targetAddr, Duration::from_millis(validTimeout));
    let latencyMs = startedAt.elapsed().as_millis();

    match connectResult {
        Ok(_) => Ok(TcpPingResult {
            host: validHost,
            port: validPort,
            resolvedEndpoint: targetAddr.to_string(),
            isReachable: true,
            latencyMs,
            detailMessage: format!("Kết nối TCP thành công ({} ms).", latencyMs),
        }),
        Err(err) => Ok(TcpPingResult {
            host: validHost,
            port: validPort,
            resolvedEndpoint: targetAddr.to_string(),
            isReachable: false,
            latencyMs,
            detailMessage: format!("Không thể kết nối: {}", err),
        }),
    }
}
