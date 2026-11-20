use eframe::egui;
use eframe::egui::{Color32, CornerRadius, FontFamily, FontId, RichText, Stroke, StrokeKind};

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    APP_SUBTITLE, APP_TITLE, FONT_SIZE_BODY, FONT_SIZE_HEADING, FONT_SIZE_SMALL,
    NAV_DOT_OFFSET_X, NAV_DOT_RADIUS, NAV_ITEM_HEIGHT, NAV_TAG_OFFSET_Y, NAV_TEXT_OFFSET_X,
    NAV_TITLE_OFFSET_Y, RADIUS_CARD_U8, SIDEBAR_CLEAR_RESERVE_WIDTH, SIDEBAR_INPUT_HEIGHT,
    SIDEBAR_SEARCH_MIN_WIDTH,
};
use crate::utils::ui_helpers::{buildSecondaryButton, renderEmptyStateBox};

// Mục đang chọn trên Sidebar: một công cụ chức năng hoặc trang Cài đặt hệ thống
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveViewTarget {
    Tool(usize),
    Settings,
}

// Vẽ toàn bộ thanh điều hướng bên trái gồm ô tìm kiếm, danh sách công cụ và mục Cài đặt cố định
pub fn renderSidebarPanel(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    tools: &[Box<dyn ToolModule>],
    activeTarget: &mut ActiveViewTarget,
    searchQuery: &mut String,
) {
    ui.label(
        RichText::new(APP_TITLE)
            .size(FONT_SIZE_HEADING)
            .strong()
            .color(palette.textPrimary),
    );
    ui.label(RichText::new(APP_SUBTITLE).color(palette.textSecondary));
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        let inputWidth = if searchQuery.is_empty() {
            ui.available_width()
        } else {
            (ui.available_width() - SIDEBAR_CLEAR_RESERVE_WIDTH).max(SIDEBAR_SEARCH_MIN_WIDTH)
        };
        ui.add_sized(
            [inputWidth, SIDEBAR_INPUT_HEIGHT],
            egui::TextEdit::singleline(searchQuery).hint_text("Tìm công cụ nhanh..."),
        );
        if !searchQuery.is_empty()
            && ui.add(buildSecondaryButton(palette, "Xóa")).clicked()
        {
            searchQuery.clear();
        }
    });

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(10.0);

    let queryLower = searchQuery.trim().to_lowercase();
    let mut matchedCount = 0usize;

    for (idx, toolRef) in tools.iter().enumerate() {
        if !matchesToolQuery(toolRef.as_ref(), &queryLower) {
            continue;
        }
        matchedCount += 1;
        let isSelected = *activeTarget == ActiveViewTarget::Tool(idx);
        if renderSidebarNavCard(
            ui,
            palette,
            toolRef.toolTitle(),
            toolRef.toolShortTag(),
            isSelected,
        )
        .clicked()
        {
            *activeTarget = ActiveViewTarget::Tool(idx);
        }
        ui.add_space(6.0);
    }

    let settingsMatches = matchesSettingsQuery(&queryLower);
    if matchedCount == 0 && !settingsMatches {
        renderEmptyStateBox(
            ui,
            palette,
            "Không tìm thấy công cụ",
            "Thử xóa từ khóa hoặc nhập tên như port, dns, json, cài đặt...",
        );
    }

    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        let isSettingsActive = *activeTarget == ActiveViewTarget::Settings;
        if renderSidebarNavCard(
            ui,
            palette,
            "Cài đặt hệ thống",
            "Giao diện, tỷ lệ & mặc định",
            isSettingsActive,
        )
        .clicked()
        {
            *activeTarget = ActiveViewTarget::Settings;
        }
        ui.add_space(6.0);
        ui.separator();
    });
}

// Kiểm tra một công cụ có khớp từ khóa tìm kiếm tiếng Việt/Anh trên Sidebar hay không
fn matchesToolQuery(tool: &dyn ToolModule, queryLower: &str) -> bool {
    if queryLower.is_empty() {
        return true;
    }
    if tool.toolTitle().to_lowercase().contains(queryLower)
        || tool.toolShortTag().to_lowercase().contains(queryLower)
        || tool.toolDescription().to_lowercase().contains(queryLower)
        || tool.toolId().to_lowercase().contains(queryLower)
    {
        return true;
    }
    tool.searchKeywords()
        .iter()
        .any(|kw| kw.to_lowercase().contains(queryLower))
}

// Kiểm tra từ khóa tìm kiếm có khớp với trang Cài đặt hay không
fn matchesSettingsQuery(queryLower: &str) -> bool {
    if queryLower.is_empty() {
        return true;
    }
    ["setting", "cài đặt", "theme", "zoom", "scale", "tỷ lệ", "config"]
        .iter()
        .any(|kw| kw.contains(queryLower) || queryLower.contains(kw))
}

// Vẽ một thẻ điều hướng công cụ trên Sidebar với vùng bấm lớn dễ thao tác
fn renderSidebarNavCard(
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    title: &str,
    shortTag: &str,
    isSelected: bool,
) -> egui::Response {
    let desiredSize = egui::vec2(ui.available_width(), NAV_ITEM_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desiredSize, egui::Sense::click());

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let bgColor = if isSelected {
        palette.accentSoftBg
    } else if response.hovered() {
        palette.controlHoverBg
    } else {
        Color32::TRANSPARENT
    };

    let borderStroke = if isSelected {
        Stroke::new(1.0, palette.accentPrimary)
    } else if response.hovered() {
        Stroke::new(1.0, palette.borderSubtle)
    } else {
        Stroke::NONE
    };

    let painter = ui.painter();
    painter.rect(
        rect,
        CornerRadius::same(RADIUS_CARD_U8),
        bgColor,
        borderStroke,
        StrokeKind::Inside,
    );

    let dotColor = if isSelected {
        palette.accentPrimary
    } else {
        palette.borderStrong
    };
    painter.circle_filled(
        egui::pos2(rect.left() + NAV_DOT_OFFSET_X, rect.center().y),
        NAV_DOT_RADIUS,
        dotColor,
    );

    let tagColor = if isSelected {
        palette.infoFg
    } else {
        palette.textSecondary
    };

    painter.text(
        egui::pos2(rect.left() + NAV_TEXT_OFFSET_X, rect.top() + NAV_TITLE_OFFSET_Y),
        egui::Align2::LEFT_TOP,
        title,
        FontId::new(FONT_SIZE_BODY, FontFamily::Proportional),
        palette.textPrimary,
    );
    painter.text(
        egui::pos2(rect.left() + NAV_TEXT_OFFSET_X, rect.top() + NAV_TAG_OFFSET_Y),
        egui::Align2::LEFT_TOP,
        shortTag,
        FontId::new(FONT_SIZE_SMALL, FontFamily::Proportional),
        tagColor,
    );

    response
}
