use eframe::egui;
use eframe::egui::{Color32, CornerRadius, Margin, RichText, Stroke};

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    ACTION_BTN_MIN_HEIGHT, CARD_MARGIN_I8, FONT_SIZE_HEADING, FONT_SIZE_SMALL,
    RADIUS_CARD_U8, RADIUS_PILL_U8, RADIUS_WIDGET_U8,
};

// Vẽ phần đầu chuẩn cho mỗi công cụ gồm tiêu đề, mô tả và nhãn thống kê tùy chọn
pub fn renderToolHeader(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    title: &str,
    subtitle: &str,
    badgeTextOpt: Option<&str>,
) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                RichText::new(title)
                    .size(FONT_SIZE_HEADING)
                    .strong()
                    .color(palette.textPrimary),
            );
            ui.add_space(2.0);
            ui.label(RichText::new(subtitle).color(palette.textSecondary));
        });

        if let Some(badgeText) = badgeTextOpt {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                renderBadgePill(ui, badgeText, palette.infoFg, palette.infoSoftBg);
            });
        }
    });
    ui.add_space(12.0);
}

// Bọc nội dung trong khung thẻ nổi đồng bộ kích thước toàn chiều rộng
pub fn renderCardContainer(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    addContents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::NONE
        .fill(palette.bgCard)
        .stroke(Stroke::new(1.0, palette.borderSubtle))
        .corner_radius(CornerRadius::same(RADIUS_CARD_U8))
        .inner_margin(Margin::same(CARD_MARGIN_I8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            addContents(ui);
        });
}

// Vẽ nhãn trạng thái dạng viên thuốc bo tròn có độ tương phản cao
pub fn renderBadgePill(ui: &mut egui::Ui, text: &str, fgColor: Color32, bgColor: Color32) {
    egui::Frame::NONE
        .fill(bgColor)
        .stroke(Stroke::new(1.0, fgColor.gamma_multiply(0.35)))
        .corner_radius(CornerRadius::same(RADIUS_PILL_U8))
        .inner_margin(Margin::symmetric(10, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .size(FONT_SIZE_SMALL)
                    .strong()
                    .color(fgColor),
            );
        });
}

// Tạo nút hành động chính nổi bật theo màu chủ đạo của giao diện
pub fn buildPrimaryButton(palette: &ThemePalette, label: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(label.to_owned()).color(Color32::WHITE).strong())
        .fill(palette.accentPrimary)
        .stroke(Stroke::new(1.0, palette.accentPrimary))
        .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
        .min_size(egui::vec2(0.0, ACTION_BTN_MIN_HEIGHT))
}

// Tạo nút hành động phụ có viền rõ và kích thước dễ bấm
pub fn buildSecondaryButton(palette: &ThemePalette, label: &str) -> egui::Button<'static> {
    egui::Button::new(
        RichText::new(label.to_owned())
            .color(palette.textPrimary)
            .strong(),
    )
    .fill(palette.controlBg)
    .stroke(Stroke::new(1.0, palette.borderStrong))
    .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
    .min_size(egui::vec2(0.0, ACTION_BTN_MIN_HEIGHT))
}

// Tạo nút cảnh báo cho các thao tác nguy hiểm như Kill hoặc Dừng quét
pub fn buildDangerButton(palette: &ThemePalette, label: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(label.to_owned()).color(Color32::WHITE).strong())
        .fill(palette.dangerBtnBg)
        .stroke(Stroke::new(1.0, palette.dangerBtnBg))
        .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
        .min_size(egui::vec2(0.0, ACTION_BTN_MIN_HEIGHT))
}

// Hiển thị trạng thái trống kèm hướng dẫn rõ ràng thay vì để bảng trắng trơn
pub fn renderEmptyStateBox(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    title: &str,
    guidance: &str,
) {
    ui.add_space(18.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().color(palette.textPrimary));
        ui.add_space(4.0);
        ui.label(RichText::new(guidance).color(palette.textSecondary));
    });
    ui.add_space(18.0);
}
