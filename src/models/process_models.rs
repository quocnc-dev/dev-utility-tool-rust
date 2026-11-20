// Kết quả phân giải tên miền DNS ra địa chỉ IP
#[derive(Debug, Clone)]
pub struct DnsLookupResult {
    pub host: String,
    pub addresses: Vec<String>,
    pub elapsedMs: u128,
}

// Kết quả kiểm tra kết nối TCP tới host:port
#[derive(Debug, Clone)]
pub struct TcpPingResult {
    pub host: String,
    pub port: u16,
    pub resolvedEndpoint: String,
    pub isReachable: bool,
    pub latencyMs: u128,
    pub detailMessage: String,
}
