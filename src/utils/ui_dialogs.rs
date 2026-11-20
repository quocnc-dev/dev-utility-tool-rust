use eframe::egui;
use eframe::egui::{CornerRadius, Margin, RichText, Stroke};

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    FONT_SIZE_SUBHEADING, MODAL_MAX_WIDTH, RADIUS_WIDGET_U8,
};
use crate::models::port_models::KillConfirmTarget;
use crate::utils::ui_helpers::{buildDangerButton, buildSecondaryButton};

// Kết quả lựa chọn của người dùng trong hộp thoại xác nhận dừng tiến trình
pub enum KillDialogDecision {
    None,
    Confirmed,
    Cancelled,
}

// Hiển thị thanh thông báo trạng thái thành công hoặc lỗi kèm nút đóng dễ bấm
pub fn renderStatusBanner(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    statusMessage: &mut Option<(bool, String)>,
) {
    let Some((isSuccess, message)) = statusMessage.clone() else {
        return;
    };

    let (fgColor, bgColor) = if isSuccess {
        (palette.successFg, palette.successSoftBg)
    } else {
        (palette.dangerFg, palette.dangerSoftBg)
    };

    egui::Frame::NONE
        .fill(bgColor)
        .stroke(Stroke::new(1.0, fgColor.gamma_multiply(0.45)))
        .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
        .inner_margin(Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new(message).color(fgColor).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(buildSecondaryButton(palette, "Đóng")).clicked() {
                        *statusMessage = None;
                    }
                });
            });
        });
    ui.add_space(10.0);
}

// Hiển thị hộp thoại xác nhận rộng rãi trước khi dừng tiến trình theo PID
pub fn renderKillConfirmDialog(
    eguiCtx: &egui::Context,
    palette: &ThemePalette,
    targetOpt: &Option<KillConfirmTarget>,
) -> KillDialogDecision {
    let Some(target) = targetOpt.as_ref() else {
        return KillDialogDecision::None;
    };

    let mut decision = KillDialogDecision::None;

    egui::Window::new("Xác nhận dừng tiến trình")
        .collapsible(false)
        .resizable(false)
        .max_width(MODAL_MAX_WIDTH)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(eguiCtx, |ui| {
            ui.add_space(4.0);
            ui.label(
                RichText::new(format!("Tiến trình: {}", target.processName))
                    .size(FONT_SIZE_SUBHEADING)
                    .strong()
                    .color(palette.textPrimary),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new(format!("PID mục tiêu: {}", target.pid))
                    .monospace()
                    .color(palette.textSecondary),
            );
            if let Some(portNum) = target.portNumber {
                ui.label(
                    RichText::new(format!("Cổng đang chiếm giữ: {}", portNum))
                        .monospace()
                        .color(palette.infoFg),
                );
            }

            ui.add_space(10.0);
            egui::Frame::NONE
                .fill(palette.warningSoftBg)
                .stroke(Stroke::new(1.0, palette.warningFg.gamma_multiply(0.45)))
                .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
                .inner_margin(Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(
                            "Cảnh báo: Dữ liệu chưa lưu trong tiến trình này sẽ bị mất ngay lập tức.",
                        )
                        .color(palette.warningFg),
                    );
                });

            ui.add_space(14.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(buildDangerButton(palette, "Xác nhận Kill"))
                    .clicked()
                {
                    decision = KillDialogDecision::Confirmed;
                }
                if ui.add(buildSecondaryButton(palette, "Hủy bỏ")).clicked() {
                    decision = KillDialogDecision::Cancelled;
                }
            });
        });

    decision
}
