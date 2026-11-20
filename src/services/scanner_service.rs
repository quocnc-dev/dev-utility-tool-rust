use std::collections::VecDeque;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::app_constants::{COMMON_DEV_PORTS, UNKNOWN_SERVICE_NAME};
use crate::models::port_models::{ScanPortItem, ScanProgressMessage};

// Tra cứu tên dịch vụ Developer quen thuộc gắn với số cổng
pub fn lookupCommonService(portNumber: u16) -> &'static str {
    for (knownPort, serviceName) in COMMON_DEV_PORTS {
        if *knownPort == portNumber {
            return serviceName;
        }
    }
    UNKNOWN_SERVICE_NAME
}

// Trả về danh sách các cổng Developer mặc định để quét nhanh
pub fn getCommonDevPortList() -> Vec<u16> {
    COMMON_DEV_PORTS.iter().map(|(port, _)| *port).collect()
}

// Khởi chạy tác vụ quét cổng đa luồng trên nền để không làm đứng giao diện
pub fn startBackgroundScan(
    targetHost: String,
    portsToScan: Vec<u16>,
    timeoutMs: u64,
    workerCount: usize,
    cancelFlag: Arc<AtomicBool>,
) -> mpsc::Receiver<ScanProgressMessage> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let resolvedIp = match resolveTargetIp(&targetHost) {
            Ok(ipAddr) => ipAddr,
            Err(errMsg) => {
                let _ = sender.send(ScanProgressMessage::Failed(errMsg));
                return;
            }
        };

        let portQueue = Arc::new(Mutex::new(VecDeque::from(portsToScan.clone())));
        let actualWorkers = workerCount.min(portsToScan.len().max(1));
        let connectTimeout = Duration::from_millis(timeoutMs);
        let mut workerHandles = Vec::with_capacity(actualWorkers);

        for _ in 0..actualWorkers {
            let queueRef = Arc::clone(&portQueue);
            let cancelRef = Arc::clone(&cancelFlag);
            let txRef = sender.clone();

            let handle = thread::spawn(move || {
                runWorkerLoop(resolvedIp, connectTimeout, queueRef, cancelRef, txRef);
            });
            workerHandles.push(handle);
        }

        for handle in workerHandles {
            let _ = handle.join();
        }

        if cancelFlag.load(Ordering::Relaxed) {
            let _ = sender.send(ScanProgressMessage::Cancelled);
        } else {
            let _ = sender.send(ScanProgressMessage::Completed);
        }
    });

    receiver
}

// Vòng lặp của mỗi luồng con lấy từng cổng từ hàng đợi và kiểm tra kết nối TCP
fn runWorkerLoop(
    targetIp: IpAddr,
    connectTimeout: Duration,
    portQueue: Arc<Mutex<VecDeque<u16>>>,
    cancelFlag: Arc<AtomicBool>,
    sender: mpsc::Sender<ScanProgressMessage>,
) {
    loop {
        if cancelFlag.load(Ordering::Relaxed) {
            break;
        }

        let nextPort = {
            let mut guard = match portQueue.lock() {
                Ok(g) => g,
                Err(_) => break,
            };
            guard.pop_front()
        };

        let Some(portNumber) = nextPort else {
            break;
        };

        let endpoint = SocketAddr::new(targetIp, portNumber);
        let startedAt = Instant::now();
        let isOpen = TcpStream::connect_timeout(&endpoint, connectTimeout).is_ok();
        let latencyMs = if isOpen {
            Some(startedAt.elapsed().as_millis())
        } else {
            None
        };

        let item = ScanPortItem {
            port: portNumber,
            isOpen,
            latencyMs,
            serviceName: lookupCommonService(portNumber).to_string(),
        };

        if sender.send(ScanProgressMessage::PortChecked(item)).is_err() {
            break;
        }
    }
}

// Phân giải host sang một địa chỉ IP cụ thể (hỗ trợ cả IPv6 và ưu tiên IPv4 nếu có)
fn resolveTargetIp(targetHost: &str) -> Result<IpAddr, String> {
    if let Ok(parsedIp) = targetHost.parse::<IpAddr>() {
        return Ok(parsedIp);
    }

    let mut addrs: Vec<SocketAddr> = (targetHost, 0u16)
        .to_socket_addrs()
        .map_err(|err| format!("Không thể phân giải địa chỉ '{}': {}", targetHost, err))?
        .collect();

    if addrs.is_empty() {
        return Err(format!("Không tìm thấy địa chỉ IP cho '{}'.", targetHost));
    }

    // Ưu tiên chọn IPv4 trước để tương thích tốt khi quét cổng nội bộ/LAN
    addrs.sort_by_key(|addr| if addr.is_ipv4() { 0 } else { 1 });
    Ok(addrs[0].ip())
}
