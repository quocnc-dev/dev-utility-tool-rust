use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use eframe::egui;
use eframe::egui::RichText;

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    DEFAULT_DNS_HOST, DEFAULT_REPAINT_INTERVAL_MS, DEFAULT_TCP_PING_PORT, FONT_SIZE_SUBHEADING,
    HOST_INPUT_WIDTH, MAX_TIMEOUT_MS, MIN_TIMEOUT_MS, PORT_SINGLE_INPUT_WIDTH, SEARCH_INPUT_WIDTH,
};
use crate::config::env_config::AppEnvConfig;
use crate::models::process_models::{DnsLookupResult, TcpPingResult};
use crate::services::network_service::{checkTcpConnection, resolveHostAddresses};
use crate::utils::ui_helpers::{
    buildPrimaryButton, buildSecondaryButton, renderBadgePill, renderCardContainer,
    renderToolHeader,
};

// Công cụ kiểm tra mạng: Phân giải tên miền DNS và kiểm tra kết nối TCP bất đồng bộ
pub struct NetworkInspectorTool {
    dnsHostInput: String,
    dnsResult: Option<DnsLookupResult>,
    dnsError: Option<String>,
    dnsReceiver: Option<mpsc::Receiver<Result<DnsLookupResult, String>>>,
    pingHostInput: String,
    pingPortInput: String,
    pingTimeoutMs: u64,
    pingResult: Option<TcpPingResult>,
    pingError: Option<String>,
    pingReceiver: Option<mpsc::Receiver<Result<TcpPingResult, String>>>,
}

impl NetworkInspectorTool {
    // Khởi tạo giá trị mặc định cho kiểm tra DNS và TCP Ping
    pub fn new(defaultTimeoutMs: u64) -> Self {
        Self {
            dnsHostInput: DEFAULT_DNS_HOST.to_string(),
            dnsResult: None,
            dnsError: None,
            dnsReceiver: None,
            pingHostInput: DEFAULT_DNS_HOST.to_string(),
            pingPortInput: DEFAULT_TCP_PING_PORT.to_string(),
            pingTimeoutMs: defaultTimeoutMs,
            pingResult: None,
            pingError: None,
            pingReceiver: None,
        }
    }

    // Khởi chạy phân giải DNS trên luồng nền để không làm đứng giao diện
    fn runDnsLookup(&mut self) {
        if self.dnsReceiver.is_some() {
            return;
        }
        self.dnsError = None;
        self.dnsResult = None;
        let hostArg = self.dnsHostInput.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(resolveHostAddresses(&hostArg));
        });
        self.dnsReceiver = Some(rx);
    }

    // Khởi chạy kiểm tra kết nối TCP trên luồng nền để không treo UI khi timeout lớn
    fn runTcpPing(&mut self) {
        if self.pingReceiver.is_some() {
            return;
        }
        self.pingError = None;
        self.pingResult = None;
        let hostArg = self.pingHostInput.clone();
        let portArg = self.pingPortInput.clone();
        let timeoutVal = self.pingTimeoutMs;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(checkTcpConnection(&hostArg, &portArg, timeoutVal));
        });
        self.pingReceiver = Some(rx);
    }

    // Nhận kết quả từ luồng nền cho cả DNS Lookup và TCP Ping
    fn pollAsyncTasks(&mut self, eguiCtx: &egui::Context) {
        if let Some(rx) = &self.dnsReceiver {
            if let Ok(outcome) = rx.try_recv() {
                match outcome {
                    Ok(result) => self.dnsResult = Some(result),
                    Err(errMsg) => self.dnsError = Some(errMsg),
                }
                self.dnsReceiver = None;
            }
        }
        if let Some(rx) = &self.pingReceiver {
            if let Ok(outcome) = rx.try_recv() {
                match outcome {
                    Ok(result) => self.pingResult = Some(result),
                    Err(errMsg) => self.pingError = Some(errMsg),
                }
                self.pingReceiver = None;
            }
        }
        if self.dnsReceiver.is_some() || self.pingReceiver.is_some() {
            eguiCtx.request_repaint_after(Duration::from_millis(DEFAULT_REPAINT_INTERVAL_MS));
        }
    }

    // Vẽ thẻ tra cứu DNS
    fn renderDnsCard(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        let isDnsBusy = self.dnsReceiver.is_some();
        renderCardContainer(ui, palette, |ui| {
            ui.label(RichText::new("Phân giải địa chỉ IP (DNS Lookup)").size(FONT_SIZE_SUBHEADING).strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Tên miền / Host:").color(palette.textSecondary));
                ui.add_enabled(
                    !isDnsBusy,
                    egui::TextEdit::singleline(&mut self.dnsHostInput)
                        .desired_width(SEARCH_INPUT_WIDTH)
                        .hint_text("Ví dụ: localhost hoặc ::1"),
                );
                let dnsLabel = if isDnsBusy { "Đang phân giải..." } else { "Phân giải DNS" };
                if ui.add_enabled(!isDnsBusy, buildPrimaryButton(palette, dnsLabel)).clicked() {
                    self.runDnsLookup();
                }
            });

            if let Some(errMsg) = &self.dnsError {
                ui.add_space(8.0);
                ui.colored_label(palette.dangerFg, errMsg);
            }
            if let Some(res) = &self.dnsResult {
                ui.add_space(10.0);
                ui.label(
                    RichText::new(format!("Tìm thấy {} bản ghi IP cho '{}' ({} ms):", res.addresses.len(), res.host, res.elapsedMs))
                        .color(palette.textSecondary),
                );
                ui.add_space(6.0);
                for ipStr in &res.addresses {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(ipStr).monospace().strong());
                        ui.add_space(8.0);
                        if ui.add(buildSecondaryButton(palette, "Sao chép")).clicked() {
                            ui.ctx().copy_text(ipStr.clone());
                        }
                    });
                }
            }
        });
    }

    // Vẽ thẻ kiểm tra kết nối TCP Ping
    fn renderTcpPingCard(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        let isPingBusy = self.pingReceiver.is_some();
        renderCardContainer(ui, palette, |ui| {
            ui.label(RichText::new("Kiểm tra kết nối TCP (TCP Ping)").size(FONT_SIZE_SUBHEADING).strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Host:").color(palette.textSecondary));
                ui.add_enabled(
                    !isPingBusy,
                    egui::TextEdit::singleline(&mut self.pingHostInput)
                        .desired_width(HOST_INPUT_WIDTH)
                        .hint_text("127.0.0.1 hoặc ::1"),
                );
                ui.label(RichText::new("Cổng:").color(palette.textSecondary));
                ui.add_enabled(
                    !isPingBusy,
                    egui::TextEdit::singleline(&mut self.pingPortInput)
                        .desired_width(PORT_SINGLE_INPUT_WIDTH)
                        .hint_text("80"),
                );
                ui.label(RichText::new("Timeout (ms):").color(palette.textSecondary));
                ui.add_enabled(
                    !isPingBusy,
                    egui::DragValue::new(&mut self.pingTimeoutMs).range(MIN_TIMEOUT_MS..=MAX_TIMEOUT_MS).speed(10.0),
                );
                let pingLabel = if isPingBusy { "Đang kiểm tra..." } else { "Kiểm tra kết nối" };
                if ui.add_enabled(!isPingBusy, buildPrimaryButton(palette, pingLabel)).clicked() {
                    self.runTcpPing();
                }
            });

            if let Some(errMsg) = &self.pingError {
                ui.add_space(8.0);
                ui.colored_label(palette.dangerFg, errMsg);
            }
            if let Some(res) = &self.pingResult {
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    if res.isReachable {
                        renderBadgePill(ui, "KẾT NỐI THÀNH CÔNG", palette.successFg, palette.successSoftBg);
                    } else {
                        renderBadgePill(ui, "KHÔNG THỂ KẾT NỐI", palette.dangerFg, palette.dangerSoftBg);
                    }
                    ui.label(RichText::new(format!("Đích: {}", res.resolvedEndpoint)).monospace().color(palette.textSecondary));
                });
                ui.add_space(4.0);
                let detailColor = if res.isReachable { palette.successFg } else { palette.dangerFg };
                ui.colored_label(detailColor, &res.detailMessage);
            }
        });
    }
}

impl ToolModule for NetworkInspectorTool {
    fn toolId(&self) -> &'static str {
        "network_inspector"
    }

    fn toolTitle(&self) -> &'static str {
        "Network & DNS Inspector"
    }

    fn toolShortTag(&self) -> &'static str {
        "Phân giải IP & TCP Ping"
    }

    fn toolDescription(&self) -> &'static str {
        "Phân giải tên miền ra địa chỉ IP và kiểm tra kết nối TCP tới bất kỳ host:port nào."
    }

    fn searchKeywords(&self) -> &'static [&'static str] {
        &["dns", "ip", "ping", "tcp", "host", "network", "kiểm tra"]
    }

    fn applyUpdatedConfig(&mut self, config: &AppEnvConfig) {
        if self.pingReceiver.is_none() {
            self.pingTimeoutMs = config.defaultTimeoutMs;
        }
    }

    fn renderToolUi(&mut self, ui: &mut egui::Ui, eguiCtx: &egui::Context, palette: &ThemePalette) {
        self.pollAsyncTasks(eguiCtx);
        renderToolHeader(ui, palette, "Kiểm tra DNS & Kết nối TCP", self.toolDescription(), None);
        self.renderDnsCard(ui, palette);
        ui.add_space(14.0);
        self.renderTcpPingCard(ui, palette);
    }
}
