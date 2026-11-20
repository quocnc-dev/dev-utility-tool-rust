use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState,
};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use crate::config::app_constants::{UDP_UNCONN_STATE, UNKNOWN_PROCESS_NAME};
use crate::models::port_models::{PortEntry, TransportProtocol};
use crate::utils::input_validator::validateKillablePid;

// Khởi tạo trình đọc thông tin tiến trình hệ thống dùng chung cho nhiều công cụ
pub fn initSystemTracker() -> (Option<System>, Option<(bool, String)>) {
    match System::new() {
        Ok(sys) => (Some(sys), None),
        Err(err) => (
            None,
            Some((
                false,
                format!("Không thể khởi tạo trình đọc tiến trình hệ thống: {:?}", err),
            )),
        ),
    }
}

// Thu thập danh sách toàn bộ các cổng TCP/UDP đang hoạt động và tên tiến trình tương ứng
pub fn fetchActivePorts(systemInfo: &mut System) -> Result<Vec<PortEntry>, String> {
    let refreshKind = ProcessRefreshKind::nothing()
        .without_tasks()
        .with_exe(UpdateKind::OnlyIfNotSet);
    systemInfo.refresh_processes_specifics(ProcessesToUpdate::All, true, refreshKind);

    let addressFamilies = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let protocolFlags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets = get_sockets_info(addressFamilies, protocolFlags)
        .map_err(|err| format!("Không thể truy vấn danh sách socket hệ thống: {}", err))?;

    let mut entries = Vec::with_capacity(sockets.len());

    for socketInfo in sockets {
        let pidOpt = socketInfo.associated_pids.first().copied();
        let (processName, exePath) = resolveProcessMeta(systemInfo, pidOpt);

        match socketInfo.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcpInfo) => {
                let isListening = matches!(tcpInfo.state, TcpState::Listen);
                let remoteEndpoint = if tcpInfo.remote_port == 0 {
                    "-".to_string()
                } else {
                    format!("{}:{}", tcpInfo.remote_addr, tcpInfo.remote_port)
                };

                entries.push(PortEntry {
                    protocol: TransportProtocol::Tcp,
                    localAddr: tcpInfo.local_addr.to_string(),
                    localPort: tcpInfo.local_port,
                    remoteAddr: remoteEndpoint,
                    stateText: format!("{:?}", tcpInfo.state).to_ascii_uppercase(),
                    isListening,
                    pid: pidOpt,
                    processName,
                    exePath,
                });
            }
            ProtocolSocketInfo::Udp(udpInfo) => {
                entries.push(PortEntry {
                    protocol: TransportProtocol::Udp,
                    localAddr: udpInfo.local_addr.to_string(),
                    localPort: udpInfo.local_port,
                    remoteAddr: "*:*".to_string(),
                    stateText: UDP_UNCONN_STATE.to_string(),
                    isListening: false,
                    pid: pidOpt,
                    processName,
                    exePath,
                });
            }
        }
    }

    entries.sort_by_key(|item| item.localPort);
    Ok(entries)
}

// Dừng một tiến trình theo PID sau khi kiểm tra điều kiện an toàn
pub fn killProcessByPid(systemInfo: &mut System, targetPid: u32) -> Result<String, String> {
    let validPid = validateKillablePid(targetPid, std::process::id())?;
    let sysPid = Pid::from_u32(validPid);
    systemInfo.refresh_processes(ProcessesToUpdate::Some(&[sysPid]), true);

    let Some(processRef) = systemInfo.process(sysPid) else {
        return Err(format!(
            "Không tìm thấy tiến trình PID {} (có thể tiến trình đã tự thoát).",
            validPid
        ));
    };

    let procName = processRef.name().to_string_lossy().into_owned();
    if processRef.kill() {
        Ok(format!(
            "Đã gửi tín hiệu dừng tiến trình '{}' (PID {}).",
            procName, validPid
        ))
    } else {
        Err(format!(
            "Không thể dừng tiến trình '{}' (PID {}). Vui lòng kiểm tra quyền quản trị.",
            procName, validPid
        ))
    }
}

// Tra cứu tên tiến trình và đường dẫn thực thi từ bảng tiến trình hệ thống
fn resolveProcessMeta(systemInfo: &System, pidOpt: Option<u32>) -> (String, String) {
    let Some(pidValue) = pidOpt else {
        return (UNKNOWN_PROCESS_NAME.to_string(), String::new());
    };

    let sysPid = Pid::from_u32(pidValue);
    if let Some(processRef) = systemInfo.process(sysPid) {
        let name = processRef.name().to_string_lossy().into_owned();
        let exePath = processRef
            .exe()
            .map(|path| path.display().to_string())
            .unwrap_or_default();
        (name, exePath)
    } else {
        (UNKNOWN_PROCESS_NAME.to_string(), String::new())
    }
}
