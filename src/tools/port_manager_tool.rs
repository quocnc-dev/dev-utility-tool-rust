use std::cmp::Ordering;
use std::time::{Duration, Instant};

use eframe::egui;
use sysinfo::System;

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::env_config::AppEnvConfig;
use crate::models::port_models::{
    KillConfirmTarget, PortEntry, PortSortColumn, ProtocolFilter, TransportProtocol,
};
use crate::services::port_service::{fetchActivePorts, initSystemTracker, killProcessByPid};
use crate::tools::port_manager_view::renderPortManagerUi;

// Trạng thái và logic điều khiển cho công cụ Quản lý Port & Kill Port
pub struct PortManagerTool {
    pub systemInfo: Option<System>,
    pub portEntries: Vec<PortEntry>,
    pub searchQuery: String,
    pub protocolFilter: ProtocolFilter,
    pub listeningOnly: bool,
    pub autoRefresh: bool,
    pub refreshIntervalSecs: u64,
    pub lastRefreshAt: Instant,
    pub sortColumn: PortSortColumn,
    pub sortAscending: bool,
    pub pendingKillTarget: Option<KillConfirmTarget>,
    pub statusMessage: Option<(bool, String)>,
}

impl PortManagerTool {
    // Khởi tạo trạng thái ban đầu và tải danh sách cổng lần đầu tiên
    pub fn new(refreshIntervalSecs: u64, defaultListenOnly: bool) -> Self {
        let (systemInfo, initError) = initSystemTracker();

        let mut instance = Self {
            systemInfo,
            portEntries: Vec::new(),
            searchQuery: String::new(),
            protocolFilter: ProtocolFilter::All,
            listeningOnly: defaultListenOnly,
            autoRefresh: true,
            refreshIntervalSecs,
            lastRefreshAt: Instant::now(),
            sortColumn: PortSortColumn::Port,
            sortAscending: true,
            pendingKillTarget: None,
            statusMessage: initError,
        };
        instance.refreshPorts();
        instance
    }

    // Tải lại danh sách các cổng đang mở từ hệ điều hành
    pub fn refreshPorts(&mut self) {
        if let Some(sysRef) = &mut self.systemInfo {
            match fetchActivePorts(sysRef) {
                Ok(entries) => {
                    self.portEntries = entries;
                }
                Err(errMsg) => {
                    self.statusMessage = Some((false, errMsg));
                }
            }
        }
        self.lastRefreshAt = Instant::now();
    }

    // Tự động làm mới danh sách nếu người dùng bật chế độ Auto Refresh
    pub fn checkAutoRefresh(&mut self, eguiCtx: &egui::Context) {
        if !self.autoRefresh {
            return;
        }
        let interval = Duration::from_secs(self.refreshIntervalSecs);
        if self.lastRefreshAt.elapsed() >= interval {
            self.refreshPorts();
        }
        eguiCtx.request_repaint_after(Duration::from_secs(1));
    }

    // Đổi cột sắp xếp hoặc đảo chiều sắp xếp nếu nhấn lại cùng một cột
    pub fn toggleSort(&mut self, column: PortSortColumn) {
        if self.sortColumn == column {
            self.sortAscending = !self.sortAscending;
        } else {
            self.sortColumn = column;
            self.sortAscending = true;
        }
    }

    // Thực hiện dừng tiến trình đang chờ xác nhận và cập nhật lại bảng cổng
    pub fn executePendingKill(&mut self) {
        let Some(target) = self.pendingKillTarget.take() else {
            return;
        };
        let Some(sysRef) = &mut self.systemInfo else {
            self.statusMessage = Some((
                false,
                "Không thể dừng tiến trình do bộ đọc hệ thống chưa sẵn sàng.".to_string(),
            ));
            return;
        };

        match killProcessByPid(sysRef, target.pid) {
            Ok(successMsg) => {
                self.statusMessage = Some((true, successMsg));
                self.refreshPorts();
            }
            Err(errMsg) => {
                self.statusMessage = Some((false, errMsg));
            }
        }
    }

    // Lọc và sắp xếp theo danh sách chỉ số để tránh sao chép chuỗi trên mỗi khung hình
    pub fn buildVisibleIndices(&self) -> Vec<usize> {
        let normalizedQuery = self.searchQuery.trim().to_lowercase();

        let mut indices: Vec<usize> = self
            .portEntries
            .iter()
            .enumerate()
            .filter(|(_, entry)| match self.protocolFilter {
                ProtocolFilter::All => true,
                ProtocolFilter::TcpOnly => entry.protocol == TransportProtocol::Tcp,
                ProtocolFilter::UdpOnly => entry.protocol == TransportProtocol::Udp,
            })
            .filter(|(_, entry)| !self.listeningOnly || entry.isListening)
            .filter(|(_, entry)| matchesPortQuery(entry, &normalizedQuery))
            .map(|(idx, _)| idx)
            .collect();

        indices.sort_by(|&leftIdx, &rightIdx| {
            let cmp = comparePortEntries(
                &self.portEntries[leftIdx],
                &self.portEntries[rightIdx],
                self.sortColumn,
            );
            if self.sortAscending {
                cmp
            } else {
                cmp.reverse()
            }
        });

        indices
    }
}

impl ToolModule for PortManagerTool {
    fn toolId(&self) -> &'static str {
        "port_manager"
    }

    fn toolTitle(&self) -> &'static str {
        "Port Manager & Kill"
    }

    fn toolShortTag(&self) -> &'static str {
        "TCP/UDP đang mở & giải phóng PID"
    }

    fn toolDescription(&self) -> &'static str {
        "Xem danh sách cổng TCP/UDP đang mở, tìm kiếm theo port/PID/tiến trình và giải phóng cổng an toàn."
    }

    fn searchKeywords(&self) -> &'static [&'static str] {
        &["port", "kill", "pid", "tcp", "udp", "listen", "socket", "cổng"]
    }

    fn applyUpdatedConfig(&mut self, config: &AppEnvConfig) {
        self.refreshIntervalSecs = config.refreshIntervalSecs;
        self.listeningOnly = config.defaultListenOnly;
    }

    fn renderToolUi(
        &mut self,
        ui: &mut egui::Ui,
        eguiCtx: &egui::Context,
        palette: &ThemePalette,
    ) {
        self.checkAutoRefresh(eguiCtx);
        renderPortManagerUi(self, ui, eguiCtx, palette);
    }
}

// Kiểm tra một mục cổng có khớp từ khóa tìm kiếm (hỗ trợ cả tiếng Việt có dấu)
fn matchesPortQuery(entry: &PortEntry, queryLower: &str) -> bool {
    if queryLower.is_empty() {
        return true;
    }

    let portMatches = entry.localPort.to_string().contains(queryLower);
    let pidMatches = entry
        .pid
        .map(|pid| pid.to_string().contains(queryLower))
        .unwrap_or(false);
    let processMatches = entry.processName.to_lowercase().contains(queryLower);
    let addrMatches = entry.localAddr.to_lowercase().contains(queryLower);
    let stateMatches = entry.stateText.to_lowercase().contains(queryLower);

    portMatches || pidMatches || processMatches || addrMatches || stateMatches
}

// So sánh hai mục cổng theo cột sắp xếp được chọn
fn comparePortEntries(left: &PortEntry, right: &PortEntry, column: PortSortColumn) -> Ordering {
    match column {
        PortSortColumn::Port => left.localPort.cmp(&right.localPort),
        PortSortColumn::Protocol => left.protocol.asLabel().cmp(right.protocol.asLabel()),
        PortSortColumn::Pid => left.pid.unwrap_or(u32::MAX).cmp(&right.pid.unwrap_or(u32::MAX)),
        PortSortColumn::ProcessName => left
            .processName
            .to_lowercase()
            .cmp(&right.processName.to_lowercase()),
        PortSortColumn::State => left.stateText.cmp(&right.stateText),
    }
}
