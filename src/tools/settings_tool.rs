use eframe::egui;
use eframe::egui::RichText;

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    APP_VERSION, FONT_SIZE_SUBHEADING, MAX_CONCURRENCY, MAX_REFRESH_INTERVAL_SECS, MAX_TIMEOUT_MS,
    MAX_UI_SCALE, MIN_CONCURRENCY, MIN_REFRESH_INTERVAL_SECS, MIN_TIMEOUT_MS, MIN_UI_SCALE,
    SETTINGS_LABEL_COL_WIDTH, UI_SCALE_PRESETS,
};
use crate::config::env_config::AppEnvConfig;
use crate::utils::ui_dialogs::renderStatusBanner;
use crate::utils::ui_helpers::{
    buildPrimaryButton, buildSecondaryButton, renderCardContainer, renderToolHeader,
};

// Trang Cài đặt hệ thống hỗ trợ chỉnh giao diện, tỷ lệ hiển thị và thông số mặc định
pub struct SettingsTool {
    pub draftConfig: AppEnvConfig,
    savedConfig: AppEnvConfig,
    pub liveConfigChanged: bool,
    statusMessage: Option<(bool, String)>,
}

impl SettingsTool {
    // Khởi tạo trạng thái ban đầu từ cấu hình vừa nạp
    pub fn new(initialConfig: AppEnvConfig) -> Self {
        Self {
            savedConfig: initialConfig.clone(),
            draftConfig: initialConfig,
            liveConfigChanged: false,
            statusMessage: None,
        }
    }

    // Lưu cấu hình hiện tại xuống file .env.local và cập nhật mốc đã lưu
    fn saveSettings(&mut self) {
        match self.draftConfig.saveToEnvLocal() {
            Ok(()) => {
                self.savedConfig = self.draftConfig.clone();
                self.statusMessage = Some((
                    true,
                    "Đã lưu toàn bộ cài đặt vào file .env.local thành công.".to_string(),
                ));
            }
            Err(errMsg) => {
                self.statusMessage = Some((false, errMsg));
            }
        }
    }

    // Khôi phục mọi thiết lập về giá trị mặc định ban đầu của ứng dụng
    fn resetToDefaults(&mut self) {
        self.draftConfig = AppEnvConfig::default();
        self.liveConfigChanged = true;
        self.statusMessage = Some((
            true,
            "Đã khôi phục cấu hình mặc định (nhấn 'Lưu cài đặt' nếu muốn ghi ra .env.local)."
                .to_string(),
        ));
    }

    // Vẽ giao diện toàn trang Cài đặt và phát hiện thay đổi để áp dụng tức thời
    pub fn renderSettingsUi(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        let snapshotBefore = self.draftConfig.clone();
        let hasUnsavedChanges = !self.draftConfig.matchesConfig(&self.savedConfig);
        let statusBadge = if hasUnsavedChanges {
            "Có thay đổi chưa lưu"
        } else {
            "Đang đồng bộ"
        };

        renderToolHeader(
            ui,
            palette,
            "Cài đặt hệ thống",
            "Tùy chỉnh giao diện, tỷ lệ hiển thị và các thông số mặc định áp dụng ngay cho toàn ứng dụng.",
            Some(statusBadge),
        );

        renderStatusBanner(ui, palette, &mut self.statusMessage);

        egui::ScrollArea::vertical()
            .id_salt("settings_scroll_area")
            .show(ui, |ui| {
                self.renderAppearanceCard(ui, palette);
                ui.add_space(12.0);
                self.renderToolDefaultsCard(ui, palette);
                ui.add_space(12.0);
                self.renderStorageAndAboutCard(ui, palette);
            });

        if !self.draftConfig.matchesConfig(&snapshotBefore) {
            self.liveConfigChanged = true;
        }
    }

    // Thẻ 1: Cài đặt giao diện Tối/Sáng và tỷ lệ phóng đại UI
    fn renderAppearanceCard(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        renderCardContainer(ui, palette, |ui| {
            ui.label(
                RichText::new("Giao diện & Tỷ lệ hiển thị")
                    .size(FONT_SIZE_SUBHEADING)
                    .strong(),
            );
            ui.add_space(10.0);

            egui::Grid::new("settings_appearance_grid")
                .num_columns(2)
                .spacing([18.0, 14.0])
                .show(ui, |ui| {
                    renderSettingLabel(
                        ui,
                        palette,
                        "Chế độ màu sắc",
                        "Chuyển đổi tức thời giữa giao diện Tối dịu mắt và Sáng rõ nét.",
                    );
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.draftConfig.defaultDarkMode,
                            true,
                            "Chế độ Tối (Dark)",
                        );
                        ui.selectable_value(
                            &mut self.draftConfig.defaultDarkMode,
                            false,
                            "Chế độ Sáng (Light)",
                        );
                    });
                    ui.end_row();

                    renderSettingLabel(
                        ui,
                        palette,
                        "Tỷ lệ hiển thị (UI Scale)",
                        "Phóng to hoặc thu nhỏ cỡ chữ, nút bấm và bảng theo độ phân giải màn hình.",
                    );
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            for (scaleVal, label) in UI_SCALE_PRESETS {
                                let isActive = (self.draftConfig.defaultUiScale - *scaleVal).abs() < 0.02;
                                if ui.selectable_label(isActive, *label).clicked() {
                                    self.draftConfig.defaultUiScale = *scaleVal;
                                }
                            }
                        });
                        ui.add_space(4.0);
                        ui.add(
                            egui::Slider::new(
                                &mut self.draftConfig.defaultUiScale,
                                MIN_UI_SCALE..=MAX_UI_SCALE,
                            )
                            .step_by(0.05)
                            .custom_formatter(|val, _| format!("{:.0}%", val * 100.0)),
                        );
                    });
                    ui.end_row();
                });
        });
    }

    // Thẻ 2: Thông số mặc định cho Port Manager, Port Scanner và Network Inspector
    fn renderToolDefaultsCard(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        renderCardContainer(ui, palette, |ui| {
            ui.label(
                RichText::new("Thông số mặc định cho Công cụ")
                    .size(FONT_SIZE_SUBHEADING)
                    .strong(),
            );
            ui.add_space(10.0);

            egui::Grid::new("settings_defaults_grid")
                .num_columns(2)
                .spacing([18.0, 14.0])
                .show(ui, |ui| {
                    renderSettingLabel(
                        ui,
                        palette,
                        "Chu kỳ làm mới Port Manager (giây)",
                        "Khoảng thời gian tự động quét lại cổng hệ thống (1 - 60 giây).",
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.draftConfig.refreshIntervalSecs)
                            .range(MIN_REFRESH_INTERVAL_SECS..=MAX_REFRESH_INTERVAL_SECS)
                            .suffix(" giây"),
                    );
                    ui.end_row();

                    renderSettingLabel(
                        ui,
                        palette,
                        "Bộ lọc cổng mặc định",
                        "Chỉ hiển thị các cổng đang ở trạng thái lắng nghe (LISTEN).",
                    );
                    ui.checkbox(
                        &mut self.draftConfig.defaultListenOnly,
                        "Bật lọc 'Chỉ cổng LISTEN'",
                    );
                    ui.end_row();

                    renderSettingLabel(
                        ui,
                        palette,
                        "Timeout kết nối mặc định (ms)",
                        "Thời gian chờ phản hồi cho Port Scanner và TCP Ping (20 - 10000 ms).",
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.draftConfig.defaultTimeoutMs)
                            .range(MIN_TIMEOUT_MS..=MAX_TIMEOUT_MS)
                            .speed(10.0)
                            .suffix(" ms"),
                    );
                    ui.end_row();

                    renderSettingLabel(
                        ui,
                        palette,
                        "Số luồng quét đồng thời",
                        "Số luồng TCP song song khi quét cổng (1 - 256 luồng).",
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.draftConfig.defaultConcurrency)
                            .range(MIN_CONCURRENCY..=MAX_CONCURRENCY)
                            .suffix(" luồng"),
                    );
                    ui.end_row();
                });
        });
    }

    // Thẻ 3: Thao tác lưu bền vững xuống .env.local và thông tin môi trường chạy
    fn renderStorageAndAboutCard(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        renderCardContainer(ui, palette, |ui| {
            ui.label(
                RichText::new("Lưu trữ cấu hình & Thông tin ứng dụng")
                    .size(FONT_SIZE_SUBHEADING)
                    .strong(),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!(
                    "Phiên bản v{}  •  Hệ thống: {}/{}  •  PID hiện tại: {}",
                    APP_VERSION,
                    std::env::consts::OS,
                    std::env::consts::ARCH,
                    std::process::id()
                ))
                .monospace()
                .color(palette.textSecondary),
            );
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(buildPrimaryButton(palette, "Lưu cài đặt (.env.local)"))
                    .clicked()
                {
                    self.saveSettings();
                }
                if ui
                    .add(buildSecondaryButton(palette, "Khôi phục mặc định gốc"))
                    .clicked()
                {
                    self.resetToDefaults();
                }
            });
        });
    }
}

// Vẽ nhãn tên cài đặt kèm mô tả ngắn ở cột bên trái của Grid
fn renderSettingLabel(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    title: &str,
    description: &str,
) {
    ui.vertical(|ui| {
        ui.set_max_width(SETTINGS_LABEL_COL_WIDTH);
        ui.label(RichText::new(title).strong().color(palette.textPrimary));
        ui.label(RichText::new(description).color(palette.textSecondary));
    });
}
