use eframe::egui;
use eframe::egui::{Color32, CornerRadius, Margin, RichText, Stroke};

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    FONT_SIZE_MONOSPACE, FONT_SIZE_SMALL, FONT_SIZE_SUBHEADING, RADIUS_WIDGET_U8, SPACE_ITEM_Y,
};
use crate::models::diff_models::{DiffLine, DiffOp, DiffViewMode};
use crate::services::diff_service::{DiffAnalysisResult, SplitDiffRow};
use crate::utils::ui_helpers::renderCardContainer;

// Khung hiển thị toàn bộ kết quả diff
pub fn renderDiffResults(
    diffResult: &DiffAnalysisResult,
    viewMode: DiffViewMode,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
) {
    renderCardContainer(ui, palette, |ui| {
        ui.label(
            RichText::new("Kết quả đối chiếu chi tiết")
                .strong()
                .color(palette.textPrimary)
                .size(FONT_SIZE_SUBHEADING),
        );
        ui.add_space(SPACE_ITEM_Y * 0.5);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(420.0)
            .show(ui, |ui| match viewMode {
                DiffViewMode::Split => renderSplitDiffView(&diffResult.splitRows, ui, palette),
                DiffViewMode::Unified => {
                    renderUnifiedDiffView(&diffResult.unifiedLines, ui, palette)
                }
            });
    });
}

// Hiển thị dạng Split View song song 2 cột
fn renderSplitDiffView(rows: &[SplitDiffRow], ui: &mut egui::Ui, palette: &ThemePalette) {
    if rows.is_empty() {
        ui.label(RichText::new("Không có dữ liệu so sánh.").color(palette.textSecondary));
        return;
    }

    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            for row in rows {
                renderSplitCell(ui, &row.left, palette);
            }
        });

        cols[1].vertical(|ui| {
            for row in rows {
                renderSplitCell(ui, &row.right, palette);
            }
        });
    });
}

// Vẽ một ô dòng trong bảng Split View
fn renderSplitCell(
    ui: &mut egui::Ui,
    cell: &Option<(usize, String, DiffOp)>,
    palette: &ThemePalette,
) {
    let (bg, fg, lineStr, content) = match cell {
        Some((lineNum, text, DiffOp::Delete)) => (
            palette.dangerSoftBg,
            palette.dangerFg,
            format!("{:3}", lineNum),
            format!("- {}", text),
        ),
        Some((lineNum, text, DiffOp::Insert)) => (
            palette.successSoftBg,
            palette.successFg,
            format!("{:3}", lineNum),
            format!("+ {}", text),
        ),
        Some((lineNum, text, DiffOp::Equal)) => (
            Color32::TRANSPARENT,
            palette.textPrimary,
            format!("{:3}", lineNum),
            format!("  {}", text),
        ),
        None => (
            palette.bgStripe,
            palette.textSecondary,
            "   ".to_string(),
            String::new(),
        ),
    };

    let frame = egui::Frame::NONE
        .fill(bg)
        .inner_margin(Margin::symmetric(6, 2));

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(lineStr)
                    .monospace()
                    .size(FONT_SIZE_SMALL)
                    .color(palette.textSecondary),
            );
            ui.separator();
            ui.label(
                RichText::new(content)
                    .monospace()
                    .size(FONT_SIZE_MONOSPACE)
                    .color(fg),
            );
        });
    });
}

// Hiển thị dạng Unified View hợp nhất 1 cột
fn renderUnifiedDiffView(lines: &[DiffLine], ui: &mut egui::Ui, palette: &ThemePalette) {
    if lines.is_empty() {
        ui.label(RichText::new("Không có dữ liệu so sánh.").color(palette.textSecondary));
        return;
    }

    for item in lines {
        let (bg, fg, prefix) = match item.op {
            DiffOp::Delete => (palette.dangerSoftBg, palette.dangerFg, "-"),
            DiffOp::Insert => (palette.successSoftBg, palette.successFg, "+"),
            DiffOp::Equal => (Color32::TRANSPARENT, palette.textPrimary, " "),
        };

        let oldStr = item
            .oldLineNum
            .map(|n| format!("{:3}", n))
            .unwrap_or_else(|| "   ".to_string());
        let newStr = item
            .newLineNum
            .map(|n| format!("{:3}", n))
            .unwrap_or_else(|| "   ".to_string());

        let frame = egui::Frame::NONE
            .fill(bg)
            .inner_margin(Margin::symmetric(6, 2));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} {}", oldStr, newStr))
                        .monospace()
                        .size(FONT_SIZE_SMALL)
                        .color(palette.textSecondary),
                );
                ui.label(
                    RichText::new(prefix)
                        .monospace()
                        .size(FONT_SIZE_MONOSPACE)
                        .color(fg)
                        .strong(),
                );
                ui.label(
                    RichText::new(&item.text)
                        .monospace()
                        .size(FONT_SIZE_MONOSPACE)
                        .color(fg),
                );
            });
        });
    }
}

// Hiển thị huy hiệu (Badge) số liệu
pub fn renderBadge(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) {
    let frame = egui::Frame::NONE
        .fill(bg)
        .corner_radius(CornerRadius::same(RADIUS_WIDGET_U8))
        .inner_margin(Margin::symmetric(8, 3))
        .stroke(Stroke::new(1.0, fg.linear_multiply(0.3)));

    frame.show(ui, |ui| {
        ui.label(RichText::new(text).color(fg).size(FONT_SIZE_SMALL).strong());
    });
}
