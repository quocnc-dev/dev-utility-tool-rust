use std::collections::BTreeSet;
use std::net::IpAddr;

use crate::config::app_constants::{
    MAX_CONCURRENCY, MAX_DNS_LABEL_LENGTH, MAX_HOST_LENGTH, MAX_PORT, MAX_SCAN_PORTS_COUNT,
    MAX_TIMEOUT_MS, MIN_CONCURRENCY, MIN_PORT, MIN_TIMEOUT_MS, PROTECTED_SYSTEM_PIDS,
};

// Kiểm tra tính hợp lệ của tên miền hoặc địa chỉ IP trước khi thao tác mạng
pub fn validateHost(rawHost: &str) -> Result<String, String> {
    let trimmedHost = rawHost.trim();
    if trimmedHost.is_empty() {
        return Err("Vui lòng nhập tên miền hoặc địa chỉ IP.".to_string());
    }
    if trimmedHost.len() > MAX_HOST_LENGTH {
        return Err(format!(
            "Địa chỉ host vượt quá giới hạn {} ký tự.",
            MAX_HOST_LENGTH
        ));
    }

    // Chấp nhận trực tiếp nếu là địa chỉ IPv4 hoặc IPv6 hợp lệ
    if trimmedHost.parse::<IpAddr>().is_ok() {
        return Ok(trimmedHost.to_string());
    }

    // Chặn các chuỗi toàn chữ số phân tách bởi dấu chấm như "999.999.999.999" (IPv4 sai octet)
    let segments: Vec<&str> = trimmedHost.split('.').collect();
    if segments
        .iter()
        .all(|seg| !seg.is_empty() && seg.chars().all(|ch| ch.is_ascii_digit()))
    {
        return Err("Địa chỉ IPv4 không hợp lệ (mỗi octet phải từ 0 đến 255).".to_string());
    }

    // Kiểm tra từng nhãn của hostname theo quy chuẩn DNS
    for label in segments {
        if label.is_empty() || label.len() > MAX_DNS_LABEL_LENGTH {
            return Err("Tên miền không hợp lệ.".to_string());
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err("Nhãn tên miền không được bắt đầu hoặc kết thúc bằng dấu '-'.".to_string());
        }
        let validLabelChars = label
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_');
        if !validLabelChars {
            return Err(
                "Tên miền chứa ký tự không hợp lệ (chỉ hỗ trợ chữ, số, '.', '-', '_')."
                    .to_string(),
            );
        }
    }

    Ok(trimmedHost.to_string())
}

// Chuyển đổi và kiểm tra một giá trị cổng đơn lẻ trong phạm vi hợp lệ
pub fn parsePortInput(rawPort: &str) -> Result<u16, String> {
    let trimmedPort = rawPort.trim();
    if trimmedPort.is_empty() {
        return Err("Vui lòng nhập số cổng.".to_string());
    }

    let parsedPort = trimmedPort
        .parse::<u16>()
        .map_err(|_| format!("Số cổng '{}' không hợp lệ ({} - {}).", trimmedPort, MIN_PORT, MAX_PORT))?;

    if parsedPort < MIN_PORT {
        return Err(format!(
            "Số cổng phải nằm trong khoảng {} đến {}.",
            MIN_PORT, MAX_PORT
        ));
    }

    Ok(parsedPort)
}

// Phân tích chuỗi danh sách và dải cổng (ví dụ: "80, 443, 3000-3010")
pub fn parsePortSpecification(rawSpec: &str) -> Result<Vec<u16>, String> {
    let trimmedSpec = rawSpec.trim();
    if trimmedSpec.is_empty() {
        return Err("Vui lòng nhập danh sách hoặc dải cổng cần quét.".to_string());
    }

    let mut uniquePorts = BTreeSet::new();

    for rawSegment in trimmedSpec.split(',') {
        let segment = rawSegment.trim();
        if segment.is_empty() {
            return Err("Danh sách cổng không hợp lệ (có dấu phẩy thừa).".to_string());
        }

        if segment.contains('-') {
            let parts: Vec<&str> = segment.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Dải cổng '{}' không đúng định dạng start-end.", segment));
            }
            let startPort = parsePortInput(parts[0])?;
            let endPort = parsePortInput(parts[1])?;
            if startPort > endPort {
                return Err(format!(
                    "Dải cổng '{}' không hợp lệ: cổng đầu lớn hơn cổng cuối.",
                    segment
                ));
            }

            let rangeCount = (endPort as usize) - (startPort as usize) + 1;
            if uniquePorts.len() + rangeCount > MAX_SCAN_PORTS_COUNT {
                return Err(format!(
                    "Tổng số cổng quét vượt quá giới hạn {} cổng mỗi lượt.",
                    MAX_SCAN_PORTS_COUNT
                ));
            }

            for portNum in startPort..=endPort {
                uniquePorts.insert(portNum);
            }
        } else {
            let singlePort = parsePortInput(segment)?;
            if uniquePorts.len() + 1 > MAX_SCAN_PORTS_COUNT && !uniquePorts.contains(&singlePort) {
                return Err(format!(
                    "Tổng số cổng quét vượt quá giới hạn {} cổng mỗi lượt.",
                    MAX_SCAN_PORTS_COUNT
                ));
            }
            uniquePorts.insert(singlePort);
        }
    }

    if uniquePorts.is_empty() {
        return Err("Không có cổng hợp lệ nào để quét.".to_string());
    }

    Ok(uniquePorts.into_iter().collect())
}

// Kiểm tra giá trị thời gian chờ (ms) nằm trong phạm vi an toàn
pub fn validateTimeoutMs(timeoutMs: u64) -> Result<u64, String> {
    if timeoutMs < MIN_TIMEOUT_MS || timeoutMs > MAX_TIMEOUT_MS {
        return Err(format!(
            "Timeout phải từ {} đến {} ms.",
            MIN_TIMEOUT_MS, MAX_TIMEOUT_MS
        ));
    }
    Ok(timeoutMs)
}

// Kiểm tra số luồng quét đồng thời nằm trong giới hạn cho phép
pub fn validateConcurrency(workerCount: usize) -> Result<usize, String> {
    if workerCount < MIN_CONCURRENCY || workerCount > MAX_CONCURRENCY {
        return Err(format!(
            "Số luồng quét phải từ {} đến {}.",
            MIN_CONCURRENCY, MAX_CONCURRENCY
        ));
    }
    Ok(workerCount)
}

// Kiểm tra PID trước khi dừng tiến trình để bảo vệ hệ thống và chính ứng dụng
pub fn validateKillablePid(targetPid: u32, currentAppPid: u32) -> Result<u32, String> {
    if PROTECTED_SYSTEM_PIDS.contains(&targetPid) {
        return Err(format!(
            "Từ chối thao tác: PID {} là tiến trình hệ thống được bảo vệ.",
            targetPid
        ));
    }
    if targetPid == currentAppPid {
        return Err("Không thể tự dừng chính ứng dụng DevUtility Hub đang chạy.".to_string());
    }
    Ok(targetPid)
}
