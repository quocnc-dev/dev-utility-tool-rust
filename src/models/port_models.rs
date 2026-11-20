// Giao thức mạng của socket đang mở
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportProtocol {
    Tcp,
    Udp,
}

impl TransportProtocol {
    // Trả về tên hiển thị ngắn gọn của giao thức
    pub fn asLabel(&self) -> &'static str {
        match self {
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
        }
    }
}

// Bộ lọc giao thức trong bảng Port Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolFilter {
    All,
    TcpOnly,
    UdpOnly,
}

// Tiêu chí sắp xếp bảng cổng đang mở
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortSortColumn {
    Port,
    Protocol,
    Pid,
    ProcessName,
    State,
}

// Thông tin một cổng đang hoạt động trên máy cục bộ
#[derive(Debug, Clone)]
pub struct PortEntry {
    pub protocol: TransportProtocol,
    pub localAddr: String,
    pub localPort: u16,
    pub remoteAddr: String,
    pub stateText: String,
    pub isListening: bool,
    pub pid: Option<u32>,
    pub processName: String,
    pub exePath: String,
}

// Thông tin tiến trình chờ xác nhận trước khi thực hiện Kill
#[derive(Debug, Clone)]
pub struct KillConfirmTarget {
    pub pid: u32,
    pub processName: String,
    pub portNumber: Option<u16>,
}

// Chế độ chọn danh sách cổng để quét trong Port Scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanPresetMode {
    CommonDevPorts,
    CustomRange,
}

// Kết quả quét của một cổng đơn lẻ
#[derive(Debug, Clone)]
pub struct ScanPortItem {
    pub port: u16,
    pub isOpen: bool,
    pub latencyMs: Option<u128>,
    pub serviceName: String,
}

// Thông điệp cập nhật tiến độ từ các luồng quét nền gửi về giao diện
#[derive(Debug, Clone)]
pub enum ScanProgressMessage {
    PortChecked(ScanPortItem),
    Completed,
    Cancelled,
    Failed(String),
}
