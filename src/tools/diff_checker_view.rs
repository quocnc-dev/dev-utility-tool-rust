use eframe::egui;
use eframe::egui::RichText;

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    FONT_SIZE_BODY, FONT_SIZE_SMALL, FONT_SIZE_SUBHEADING, SPACE_ITEM_X, SPACE_ITEM_Y,
};
use crate::models::diff_models::DiffViewMode;
use crate::tools::diff_checker_tool::DiffCheckerTool;
use crate::tools::diff_view_components::{renderBadge, renderDiffResults};
use crate::utils::ui_helpers::{
    buildPrimaryButton, buildSecondaryButton, renderCardContainer, renderToolHeader,
};

// Giao diện người dùng cho công cụ So sánh văn bản & Code
pub fn renderDiffCheckerUi(tool: &mut DiffCheckerTool, ui: &mut egui::Ui, palette: &ThemePalette) {
    renderToolHeader(
        ui,
        palette,
        "Diff Checker",
        "So sánh đối chiếu chi tiết từng dòng giữa hai phiên bản văn bản hoặc mã nguồn",
        None,
    );

    ui.add_space(SPACE_ITEM_Y);

    // Thanh điều khiển và tùy chọn so sánh
    renderControlBar(tool, ui, palette);

    ui.add_space(SPACE_ITEM_Y);

    // Khung nhập hai văn bản đầu vào
    renderInputEditors(tool, ui, palette);

    ui.add_space(SPACE_ITEM_Y);

    // Khung hiển thị kết quả diff
    renderDiffResults(&tool.diffResult, tool.viewMode, ui, palette);
}

// Thanh công cụ gồm các nút thao tác và tùy chọn
fn renderControlBar(tool: &mut DiffCheckerTool, ui: &mut egui::Ui, palette: &ThemePalette) {
    renderCardContainer(ui, palette, |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(buildPrimaryButton(palette, "🔍 So sánh Diff"))
                .clicked()
            {
                tool.executeDiff();
            }

            if ui
                .add(buildSecondaryButton(palette, "⇄ Hoán đổi vị trí"))
                .clicked()
            {
                tool.swapInputs();
            }

            if ui
                .add(buildSecondaryButton(palette, "📄 Nạp mẫu"))
                .clicked()
            {
                tool.loadSample();
            }

            if ui
                .add(buildSecondaryButton(palette, "🗑 Xóa hết"))
                .clicked()
            {
                tool.clearAll();
            }

            ui.add_space(SPACE_ITEM_X * 2.0);
            ui.separator();
            ui.add_space(SPACE_ITEM_X);

            // Chuyển đổi chế độ xem: Split vs Unified
            ui.label(
                RichText::new("Chế độ:")
                    .color(palette.textSecondary)
                    .size(FONT_SIZE_BODY),
            );

            let isSplit = tool.viewMode == DiffViewMode::Split;
            if ui
                .selectable_label(isSplit, "Song song (Split)")
                .clicked()
            {
                tool.viewMode = DiffViewMode::Split;
            }

            let isUnified = tool.viewMode == DiffViewMode::Unified;
            if ui
                .selectable_label(isUnified, "Hợp nhất (Unified)")
                .clicked()
            {
                tool.viewMode = DiffViewMode::Unified;
            }

            ui.add_space(SPACE_ITEM_X);
            ui.separator();
            ui.add_space(SPACE_ITEM_X);

            // Tùy chọn so sánh
            let mut optsChanged = false;
            if ui
                .checkbox(&mut tool.options.ignoreWhitespace, "Bỏ qua khoảng trắng")
                .changed()
            {
                optsChanged = true;
            }

            if ui
                .checkbox(&mut tool.options.caseSensitive, "Phân biệt hoa/thường")
                .changed()
            {
                optsChanged = true;
            }

            if optsChanged {
                tool.executeDiff();
            }
        });

        // Thanh tóm tắt kết quả nếu đã so sánh
        if tool.hasComputed {
            ui.add_space(SPACE_ITEM_Y * 0.8);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Tổng kết:")
                        .color(palette.textSecondary)
                        .size(FONT_SIZE_SMALL),
                );

                renderBadge(
                    ui,
                    &format!("+{} Thêm mới", tool.diffResult.summary.additions),
                    palette.successFg,
                    palette.successSoftBg,
                );

                renderBadge(
                    ui,
                    &format!("-{} Đã xóa", tool.diffResult.summary.deletions),
                    palette.dangerFg,
                    palette.dangerSoftBg,
                );

                renderBadge(
                    ui,
                    &format!("={} Giữ nguyên", tool.diffResult.summary.unchanged),
                    palette.textSecondary,
                    palette.controlBg,
                );
            });
        }
    });
}

// Khung nhập hai văn bản cần so sánh
fn renderInputEditors(tool: &mut DiffCheckerTool, ui: &mut egui::Ui, palette: &ThemePalette) {
    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            renderCardContainer(ui, palette, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Văn bản gốc (Bên trái / Bản cũ)")
                            .strong()
                            .color(palette.textPrimary)
                            .size(FONT_SIZE_SUBHEADING),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} dòng", tool.leftText.lines().count()))
                                .color(palette.textSecondary)
                                .size(FONT_SIZE_SMALL),
                        );
                    });
                });
                ui.add_space(SPACE_ITEM_Y * 0.5);

                let textEdit = egui::TextEdit::multiline(&mut tool.leftText)
                    .code_editor()
                    .desired_rows(8)
                    .desired_width(f32::INFINITY);
                if ui.add(textEdit).changed() {
                    tool.executeDiff();
                }
            });
        });

        cols[1].vertical(|ui| {
            renderCardContainer(ui, palette, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Văn bản mới (Bên phải / Bản sửa đổi)")
                            .strong()
                            .color(palette.textPrimary)
                            .size(FONT_SIZE_SUBHEADING),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} dòng", tool.rightText.lines().count()))
                                .color(palette.textSecondary)
                                .size(FONT_SIZE_SMALL),
                        );
                    });
                });
                ui.add_space(SPACE_ITEM_Y * 0.5);

                let textEdit = egui::TextEdit::multiline(&mut tool.rightText)
                    .code_editor()
                    .desired_rows(8)
                    .desired_width(f32::INFINITY);
                if ui.add(textEdit).changed() {
                    tool.executeDiff();
                }
            });
        });
    });
}
