use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use eframe::egui;

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    DEFAULT_REPAINT_INTERVAL_MS, DEFAULT_SCAN_HOST, DEFAULT_SCAN_PORT_SPEC,
};
use crate::config::env_config::AppEnvConfig;
use crate::models::port_models::{ScanPortItem, ScanPresetMode, ScanProgressMessage};
use crate::services::scanner_service::{getCommonDevPortList, startBackgroundScan};
use crate::tools::port_scanner_view::renderPortScannerUi;
use crate::utils::input_validator::{
    parsePortSpecification, validateConcurrency, validateHost, validateTimeoutMs,
};

// Trạng thái và điều khiển cho công cụ Quét Port đa luồng
pub struct PortScannerTool {
    pub hostInput: String,
    pub presetMode: ScanPresetMode,
    pub customPortSpec: String,
    pub timeoutMsInput: u64,
    pub concurrencyInput: usize,
    pub resultFilterText: String,
    pub openOnlyFilter: bool,
    pub isScanning: bool,
    pub totalPlannedPorts: usize,
    pub scannedCount: usize,
    pub scanResults: Vec<ScanPortItem>,
    pub receiverOpt: Option<mpsc::Receiver<ScanProgressMessage>>,
    pub cancelFlagOpt: Option<Arc<AtomicBool>>,
    pub statusMessage: Option<(bool, String)>,
}

impl PortScannerTool {
    // Khởi tạo giá trị mặc định cho bộ quét cổng từ cấu hình môi trường
    pub fn new(defaultTimeoutMs: u64, defaultConcurrency: usize) -> Self {
        Self {
            hostInput: DEFAULT_SCAN_HOST.to_string(),
            presetMode: ScanPresetMode::CommonDevPorts,
            customPortSpec: DEFAULT_SCAN_PORT_SPEC.to_string(),
            timeoutMsInput: defaultTimeoutMs,
            concurrencyInput: defaultConcurrency,
            resultFilterText: String::new(),
            openOnlyFilter: false,
            isScanning: false,
            totalPlannedPorts: 0,
            scannedCount: 0,
            scanResults: Vec::new(),
            receiverOpt: None,
            cancelFlagOpt: None,
            statusMessage: None,
        }
    }

    // Kiểm tra đầu vào và khởi chạy tác vụ quét cổng trên luồng nền
    pub fn startScan(&mut self) {
        if self.isScanning {
            return;
        }

        let validHost = match validateHost(&self.hostInput) {
            Ok(host) => host,
            Err(errMsg) => {
                self.statusMessage = Some((false, errMsg));
                return;
            }
        };

        let validTimeout = match validateTimeoutMs(self.timeoutMsInput) {
            Ok(val) => val,
            Err(errMsg) => {
                self.statusMessage = Some((false, errMsg));
                return;
            }
        };

        let validConcurrency = match validateConcurrency(self.concurrencyInput) {
            Ok(val) => val,
            Err(errMsg) => {
                self.statusMessage = Some((false, errMsg));
                return;
            }
        };

        let targetPorts = match self.presetMode {
            ScanPresetMode::CommonDevPorts => getCommonDevPortList(),
            ScanPresetMode::CustomRange => match parsePortSpecification(&self.customPortSpec) {
                Ok(ports) => ports,
                Err(errMsg) => {
                    self.statusMessage = Some((false, errMsg));
                    return;
                }
            },
        };

        self.totalPlannedPorts = targetPorts.len();
        self.scannedCount = 0;
        self.scanResults.clear();
        self.statusMessage = None;
        self.isScanning = true;

        let cancelFlag = Arc::new(AtomicBool::new(false));
        let receiver = startBackgroundScan(
            validHost,
            targetPorts,
            validTimeout,
            validConcurrency,
            Arc::clone(&cancelFlag),
        );

        self.cancelFlagOpt = Some(cancelFlag);
        self.receiverOpt = Some(receiver);
    }

    // Yêu cầu dừng tiến trình quét cổng đang chạy
    pub fn stopScan(&mut self) {
        if let Some(flag) = &self.cancelFlagOpt {
            flag.store(true, Ordering::Relaxed);
        }
    }

    // Nhận các thông điệp tiến độ từ luồng nền để cập nhật giao diện
    pub fn pollScanProgress(&mut self, eguiCtx: &egui::Context) {
        let Some(receiver) = &self.receiverOpt else {
            return;
        };

        let mut finished = false;

        while let Ok(message) = receiver.try_recv() {
            match message {
                ScanProgressMessage::PortChecked(item) => {
                    self.scannedCount += 1;
                    self.scanResults.push(item);
                }
                ScanProgressMessage::Completed => {
                    finished = true;
                    let openCount = self.scanResults.iter().filter(|r| r.isOpen).count();
                    self.statusMessage = Some((
                        true,
                        format!(
                            "Hoàn tất quét {} cổng. Phát hiện {} cổng đang mở.",
                            self.scannedCount, openCount
                        ),
                    ));
                }
                ScanProgressMessage::Cancelled => {
                    finished = true;
                    self.statusMessage = Some((
                        false,
                        format!(
                            "Đã dừng quét theo yêu cầu ({}/{} cổng).",
                            self.scannedCount, self.totalPlannedPorts
                        ),
                    ));
                }
                ScanProgressMessage::Failed(errMsg) => {
                    finished = true;
                    self.statusMessage = Some((false, errMsg));
                }
            }
        }

        if finished {
            self.isScanning = false;
            self.receiverOpt = None;
            self.cancelFlagOpt = None;
            self.scanResults.sort_by_key(|item| item.port);
        } else if self.isScanning {
            eguiCtx.request_repaint_after(Duration::from_millis(DEFAULT_REPAINT_INTERVAL_MS));
        }
    }

    // Lọc danh sách chỉ số kết quả quét theo từ khóa và trạng thái mở
    pub fn buildFilteredIndices(&self) -> Vec<usize> {
        let queryLower = self.resultFilterText.trim().to_lowercase();

        let mut indices: Vec<usize> = self
            .scanResults
            .iter()
            .enumerate()
            .filter(|(_, item)| !self.openOnlyFilter || item.isOpen)
            .filter(|(_, item)| {
                if queryLower.is_empty() {
                    return true;
                }
                item.port.to_string().contains(&queryLower)
                    || item.serviceName.to_lowercase().contains(&queryLower)
            })
            .map(|(idx, _)| idx)
            .collect();

        indices.sort_by_key(|&idx| self.scanResults[idx].port);
        indices
    }
}

impl ToolModule for PortScannerTool {
    fn toolId(&self) -> &'static str {
        "port_scanner"
    }

    fn toolTitle(&self) -> &'static str {
        "Port Scanner"
    }

    fn toolShortTag(&self) -> &'static str {
        "Quét cổng TCP đa luồng"
    }

    fn toolDescription(&self) -> &'static str {
        "Quét cổng TCP đa luồng theo cổng Developer phổ biến hoặc dải tùy chỉnh trên máy nội bộ / LAN."
    }

    fn searchKeywords(&self) -> &'static [&'static str] {
        &["scan", "port", "quét", "range", "open", "tcp", "network"]
    }

    fn applyUpdatedConfig(&mut self, config: &AppEnvConfig) {
        if !self.isScanning {
            self.timeoutMsInput = config.defaultTimeoutMs;
            self.concurrencyInput = config.defaultConcurrency;
        }
    }

    fn renderToolUi(
        &mut self,
        ui: &mut egui::Ui,
        eguiCtx: &egui::Context,
        palette: &ThemePalette,
    ) {
        self.pollScanProgress(eguiCtx);
        renderPortScannerUi(self, ui, palette);
    }
}
