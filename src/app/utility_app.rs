use eframe::egui;
use eframe::egui::{Margin, Stroke};

use crate::app::sidebar_view::{renderSidebarPanel, ActiveViewTarget};
use crate::app::theme::{applyAppThemeAndScale, getPalette};
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    CANVAS_MARGIN_I8, SIDEBAR_DEFAULT_WIDTH, SIDEBAR_MARGIN_I8, SIDEBAR_MAX_WIDTH,
    SIDEBAR_MIN_WIDTH,
};
use crate::config::env_config::AppEnvConfig;
use crate::tools::network_inspector_tool::NetworkInspectorTool;
use crate::tools::port_manager_tool::PortManagerTool;
use crate::tools::port_scanner_tool::PortScannerTool;
use crate::tools::settings_tool::SettingsTool;
use crate::tools::text_formatter_tool::TextFormatterTool;

// Khung ứng dụng chính quản lý danh sách công cụ, trang Cài đặt và giao diện hiện hành
pub struct DevUtilityApp {
    tools: Vec<Box<dyn ToolModule>>,
    settingsPage: SettingsTool,
    activeTarget: ActiveViewTarget,
    toolSearchQuery: String,
    styleDirty: bool,
}

impl DevUtilityApp {
    // Khởi tạo toàn bộ danh sách công cụ và trang Cài đặt từ cấu hình môi trường
    pub fn new(envConfig: AppEnvConfig) -> Self {
        let tools: Vec<Box<dyn ToolModule>> = vec![
            Box::new(PortManagerTool::new(
                envConfig.refreshIntervalSecs,
                envConfig.defaultListenOnly,
            )),
            Box::new(PortScannerTool::new(
                envConfig.defaultTimeoutMs,
                envConfig.defaultConcurrency,
            )),
            Box::new(NetworkInspectorTool::new(envConfig.defaultTimeoutMs)),
            Box::new(TextFormatterTool::new()),
        ];

        Self {
            tools,
            settingsPage: SettingsTool::new(envConfig),
            activeTarget: ActiveViewTarget::Tool(0),
            toolSearchQuery: String::new(),
            styleDirty: true,
        }
    }

    // Đồng bộ cấu hình mới từ trang Cài đặt tới theme, tỷ lệ UI và các công cụ đang mở
    fn syncThemeAndTools(&mut self, eguiCtx: &egui::Context) {
        if self.settingsPage.liveConfigChanged {
            self.settingsPage.liveConfigChanged = false;
            self.styleDirty = true;
            let updatedConfig = self.settingsPage.draftConfig.clone();
            for tool in &mut self.tools {
                tool.applyUpdatedConfig(&updatedConfig);
            }
        }

        if self.styleDirty {
            let activeCfg = &self.settingsPage.draftConfig;
            applyAppThemeAndScale(eguiCtx, activeCfg.defaultDarkMode, activeCfg.defaultUiScale);
            self.styleDirty = false;
        }
    }
}

impl eframe::App for DevUtilityApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let eguiCtx = ui.ctx().clone();
        self.syncThemeAndTools(&eguiCtx);
        let palette = getPalette(self.settingsPage.draftConfig.defaultDarkMode);

        let sidebarFrame = egui::Frame::NONE
            .fill(palette.bgSidebar)
            .inner_margin(Margin::same(SIDEBAR_MARGIN_I8))
            .stroke(Stroke::new(1.0, palette.borderSubtle));

        egui::Panel::left("dev_utility_sidebar")
            .frame(sidebarFrame)
            .resizable(true)
            .default_size(SIDEBAR_DEFAULT_WIDTH)
            .min_size(SIDEBAR_MIN_WIDTH)
            .max_size(SIDEBAR_MAX_WIDTH)
            .show(ui, |ui| {
                renderSidebarPanel(
                    ui,
                    &palette,
                    &self.tools,
                    &mut self.activeTarget,
                    &mut self.toolSearchQuery,
                );
            });

        let canvasFrame = egui::Frame::NONE
            .fill(palette.bgCanvas)
            .inner_margin(Margin::same(CANVAS_MARGIN_I8));

        egui::CentralPanel::default()
            .frame(canvasFrame)
            .show(ui, |ui| match self.activeTarget {
                ActiveViewTarget::Tool(idx) => {
                    if let Some(activeTool) = self.tools.get_mut(idx) {
                        activeTool.renderToolUi(ui, &eguiCtx, &palette);
                    }
                }
                ActiveViewTarget::Settings => {
                    self.settingsPage.renderSettingsUi(ui, &palette);
                }
            });
    }
}
