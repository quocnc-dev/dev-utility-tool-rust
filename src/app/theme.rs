use eframe::egui;
use eframe::egui::{Color32, CornerRadius, FontFamily, FontId, Margin, Shadow, Stroke, TextStyle};

use crate::config::app_constants::{
    BTN_PAD_X, BTN_PAD_Y, CARD_MARGIN_I8, CHECKBOX_ICON_INNER, CHECKBOX_ICON_SPACING,
    CHECKBOX_ICON_WIDTH, FONT_SIZE_BODY, FONT_SIZE_BUTTON, FONT_SIZE_HEADING,
    FONT_SIZE_MONOSPACE, FONT_SIZE_SMALL, FONT_SIZE_SUBHEADING, MIN_INTERACT_HEIGHT,
    MIN_INTERACT_WIDTH, RADIUS_CARD_U8, RADIUS_WIDGET_U8, SEARCH_INPUT_WIDTH, SPACE_ITEM_X,
    SPACE_ITEM_Y, TEXT_STYLE_SUBHEADING,
};

// Bộ token màu sắc phân lớp cho giao diện Tối và Sáng
#[derive(Clone, Copy)]
pub struct ThemePalette {
    pub bgSidebar: Color32,
    pub bgCanvas: Color32,
    pub bgCard: Color32,
    pub bgInput: Color32,
    pub bgStripe: Color32,
    pub controlBg: Color32,
    pub controlHoverBg: Color32,
    pub controlActiveBg: Color32,
    pub borderSubtle: Color32,
    pub borderStrong: Color32,
    pub textPrimary: Color32,
    pub textSecondary: Color32,
    pub accentPrimary: Color32,
    pub accentSoftBg: Color32,
    pub successFg: Color32,
    pub successSoftBg: Color32,
    pub warningFg: Color32,
    pub warningSoftBg: Color32,
    pub dangerFg: Color32,
    pub dangerBtnBg: Color32,
    pub dangerSoftBg: Color32,
    pub infoFg: Color32,
    pub infoSoftBg: Color32,
}

// Trả về bảng màu thiết kế tương ứng với chế độ hiển thị hiện tại
pub fn getPalette(isDarkMode: bool) -> ThemePalette {
    if isDarkMode {
        ThemePalette {
            bgSidebar: Color32::from_rgb(13, 19, 31),
            bgCanvas: Color32::from_rgb(17, 24, 39),
            bgCard: Color32::from_rgb(24, 34, 52),
            bgInput: Color32::from_rgb(12, 18, 30),
            bgStripe: Color32::from_rgb(20, 29, 46),
            controlBg: Color32::from_rgb(33, 46, 70),
            controlHoverBg: Color32::from_rgb(44, 62, 92),
            controlActiveBg: Color32::from_rgb(53, 74, 110),
            borderSubtle: Color32::from_rgb(48, 63, 90),
            borderStrong: Color32::from_rgb(76, 98, 134),
            textPrimary: Color32::from_rgb(241, 245, 249),
            textSecondary: Color32::from_rgb(148, 163, 184),
            accentPrimary: Color32::from_rgb(2, 132, 199),
            accentSoftBg: Color32::from_rgb(16, 48, 78),
            successFg: Color32::from_rgb(74, 222, 128),
            successSoftBg: Color32::from_rgb(19, 54, 42),
            warningFg: Color32::from_rgb(251, 191, 36),
            warningSoftBg: Color32::from_rgb(59, 42, 18),
            dangerFg: Color32::from_rgb(248, 113, 113),
            dangerBtnBg: Color32::from_rgb(190, 44, 44),
            dangerSoftBg: Color32::from_rgb(64, 24, 30),
            infoFg: Color32::from_rgb(56, 189, 248),
            infoSoftBg: Color32::from_rgb(17, 47, 74),
        }
    } else {
        ThemePalette {
            bgSidebar: Color32::from_rgb(235, 240, 246),
            bgCanvas: Color32::from_rgb(244, 247, 251),
            bgCard: Color32::from_rgb(255, 255, 255),
            bgInput: Color32::from_rgb(255, 255, 255),
            bgStripe: Color32::from_rgb(248, 250, 252),
            controlBg: Color32::from_rgb(238, 242, 247),
            controlHoverBg: Color32::from_rgb(225, 232, 242),
            controlActiveBg: Color32::from_rgb(209, 220, 235),
            borderSubtle: Color32::from_rgb(209, 217, 228),
            borderStrong: Color32::from_rgb(148, 163, 184),
            textPrimary: Color32::from_rgb(15, 23, 42),
            textSecondary: Color32::from_rgb(71, 85, 105),
            accentPrimary: Color32::from_rgb(3, 105, 161),
            accentSoftBg: Color32::from_rgb(224, 242, 254),
            successFg: Color32::from_rgb(21, 128, 61),
            successSoftBg: Color32::from_rgb(220, 252, 231),
            warningFg: Color32::from_rgb(161, 98, 7),
            warningSoftBg: Color32::from_rgb(254, 243, 199),
            dangerFg: Color32::from_rgb(185, 28, 28),
            dangerBtnBg: Color32::from_rgb(200, 38, 38),
            dangerSoftBg: Color32::from_rgb(254, 226, 226),
            infoFg: Color32::from_rgb(3, 105, 161),
            infoSoftBg: Color32::from_rgb(224, 242, 254),
        }
    }
}

// Áp dụng đồng bộ thang cỡ chữ, kích thước nút, màu sắc và mức phóng đại cho toàn ứng dụng
pub fn applyAppThemeAndScale(eguiCtx: &egui::Context, isDarkMode: bool, uiScale: f32) {
    let palette = getPalette(isDarkMode);
    eguiCtx.set_zoom_factor(uiScale);

    let mut style = egui::Style::default();
    configureTextStyles(&mut style);
    configureSpacing(&mut style);
    style.visuals = buildVisuals(isDarkMode, &palette);

    let activeTheme = if isDarkMode {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    eguiCtx.set_theme(activeTheme);
    eguiCtx.set_style_of(activeTheme, style);
}

// Cấu hình thang cỡ chữ rõ ràng, dễ đọc cho mọi thành phần giao diện
fn configureTextStyles(style: &mut egui::Style) {
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(FONT_SIZE_SMALL, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(FONT_SIZE_BODY, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(FONT_SIZE_BUTTON, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(FONT_SIZE_MONOSPACE, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(FONT_SIZE_HEADING, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Name(TEXT_STYLE_SUBHEADING.into()),
        FontId::new(FONT_SIZE_SUBHEADING, FontFamily::Proportional),
    );
}

// Cấu hình khoảng đệm và kích thước vùng bấm tối thiểu để dễ thao tác
fn configureSpacing(style: &mut egui::Style) {
    style.spacing.item_spacing = egui::vec2(SPACE_ITEM_X, SPACE_ITEM_Y);
    style.spacing.button_padding = egui::vec2(BTN_PAD_X, BTN_PAD_Y);
    style.spacing.interact_size = egui::vec2(MIN_INTERACT_WIDTH, MIN_INTERACT_HEIGHT);
    style.spacing.window_margin = Margin::same(CARD_MARGIN_I8);
    style.spacing.icon_width = CHECKBOX_ICON_WIDTH;
    style.spacing.icon_width_inner = CHECKBOX_ICON_INNER;
    style.spacing.icon_spacing = CHECKBOX_ICON_SPACING;
    style.spacing.text_edit_width = SEARCH_INPUT_WIDTH;
    style.spacing.scroll = egui::style::ScrollStyle::solid();
}

// Tạo Visuals hoàn chỉnh từ bảng token màu sắc
fn buildVisuals(isDarkMode: bool, palette: &ThemePalette) -> egui::Visuals {
    let mut visuals = if isDarkMode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    let widgetRadius = CornerRadius::same(RADIUS_WIDGET_U8);
    let cardRadius = CornerRadius::same(RADIUS_CARD_U8);

    visuals.override_text_color = Some(palette.textPrimary);
    visuals.weak_text_color = Some(palette.textSecondary);
    visuals.panel_fill = palette.bgCanvas;
    visuals.window_fill = palette.bgCard;
    visuals.window_stroke = Stroke::new(1.0, palette.borderSubtle);
    visuals.window_corner_radius = cardRadius;
    visuals.menu_corner_radius = widgetRadius;
    visuals.extreme_bg_color = palette.bgInput;
    visuals.text_edit_bg_color = Some(palette.bgInput);
    visuals.faint_bg_color = palette.bgStripe;
    visuals.code_bg_color = palette.controlBg;
    visuals.warn_fg_color = palette.warningFg;
    visuals.error_fg_color = palette.dangerFg;
    visuals.hyperlink_color = palette.accentPrimary;
    visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
    visuals.window_shadow = Shadow {
        offset: [0, 8],
        blur: 20,
        spread: 0,
        color: Color32::from_black_alpha(if isDarkMode { 110 } else { 32 }),
    };

    visuals.selection = egui::style::Selection {
        bg_fill: palette.accentSoftBg,
        stroke: Stroke::new(1.5, palette.accentPrimary),
    };
    visuals.text_cursor.stroke = Stroke::new(2.0, palette.accentPrimary);

    visuals.widgets.noninteractive.bg_fill = palette.bgCard;
    visuals.widgets.noninteractive.weak_bg_fill = palette.bgCard;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, palette.borderSubtle);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette.textPrimary);
    visuals.widgets.noninteractive.corner_radius = widgetRadius;

    visuals.widgets.inactive.bg_fill = palette.controlBg;
    visuals.widgets.inactive.weak_bg_fill = palette.controlBg;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, palette.borderSubtle);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, palette.textPrimary);
    visuals.widgets.inactive.corner_radius = widgetRadius;

    visuals.widgets.hovered.bg_fill = palette.controlHoverBg;
    visuals.widgets.hovered.weak_bg_fill = palette.controlHoverBg;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, palette.accentPrimary);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, palette.textPrimary);
    visuals.widgets.hovered.corner_radius = widgetRadius;

    visuals.widgets.active.bg_fill = palette.controlActiveBg;
    visuals.widgets.active.weak_bg_fill = palette.controlActiveBg;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, palette.accentPrimary);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, palette.textPrimary);
    visuals.widgets.active.corner_radius = widgetRadius;

    visuals.widgets.open.bg_fill = palette.controlHoverBg;
    visuals.widgets.open.weak_bg_fill = palette.controlHoverBg;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, palette.borderStrong);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, palette.textPrimary);
    visuals.widgets.open.corner_radius = widgetRadius;

    visuals
}
