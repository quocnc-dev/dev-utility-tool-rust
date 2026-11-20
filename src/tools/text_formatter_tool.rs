use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use eframe::egui;
use eframe::egui::RichText;

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    DEFAULT_JSON_SAMPLE, FONT_SIZE_SUBHEADING, JSON_SAMPLE_OUTPUT_ROWS, TEXT_AREA_MIN_ROWS,
    TWO_COL_BREAKPOINT,
};
use crate::utils::ui_helpers::{
    buildPrimaryButton, buildSecondaryButton, renderCardContainer, renderToolHeader,
};

// Chế độ xử lý văn bản trong công cụ Formatter & Encoder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FormatterMode {
    JsonFormat,
    Base64Codec,
    UrlCodec,
}

// Công cụ định dạng JSON, nén JSON và mã hóa/giải mã Base64, URL
pub struct TextFormatterTool {
    activeMode: FormatterMode,
    inputText: String,
    outputText: String,
    errorMessage: Option<String>,
}

impl TextFormatterTool {
    // Khởi tạo với dữ liệu JSON mẫu để người dùng thử nghiệm ngay
    pub fn new() -> Self {
        Self {
            activeMode: FormatterMode::JsonFormat,
            inputText: DEFAULT_JSON_SAMPLE.to_string(),
            outputText: String::new(),
            errorMessage: None,
        }
    }

    // Định dạng JSON đẹp với thụt đầu dòng chuẩn
    fn formatJsonPretty(&mut self) {
        self.errorMessage = None;
        match serde_json::from_str::<serde_json::Value>(&self.inputText) {
            Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                Ok(formatted) => self.outputText = formatted,
                Err(err) => self.errorMessage = Some(format!("Lỗi xuất JSON: {}", err)),
            },
            Err(err) => self.errorMessage = Some(format!("JSON không hợp lệ: {}", err)),
        }
    }

    // Nén JSON thành một dòng duy nhất
    fn minifyJson(&mut self) {
        self.errorMessage = None;
        match serde_json::from_str::<serde_json::Value>(&self.inputText) {
            Ok(parsed) => match serde_json::to_string(&parsed) {
                Ok(minified) => self.outputText = minified,
                Err(err) => self.errorMessage = Some(format!("Lỗi nén JSON: {}", err)),
            },
            Err(err) => self.errorMessage = Some(format!("JSON không hợp lệ: {}", err)),
        }
    }

    // Mã hóa chuỗi sang Base64
    fn encodeBase64(&mut self) {
        self.errorMessage = None;
        self.outputText = BASE64_STANDARD.encode(self.inputText.as_bytes());
    }

    // Giải mã chuỗi Base64 về UTF-8
    fn decodeBase64(&mut self) {
        self.errorMessage = None;
        match BASE64_STANDARD.decode(self.inputText.trim()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(decoded) => self.outputText = decoded,
                Err(err) => self.errorMessage = Some(format!("Dữ liệu không phải UTF-8: {}", err)),
            },
            Err(err) => self.errorMessage = Some(format!("Chuỗi Base64 không hợp lệ: {}", err)),
        }
    }

    // Mã hóa chuỗi theo chuẩn URL
    fn encodeUrl(&mut self) {
        self.errorMessage = None;
        self.outputText = urlencoding::encode(&self.inputText).into_owned();
    }

    // Giải mã chuỗi URL
    fn decodeUrl(&mut self) {
        self.errorMessage = None;
        match urlencoding::decode(&self.inputText) {
            Ok(decoded) => self.outputText = decoded.into_owned(),
            Err(err) => self.errorMessage = Some(format!("Chuỗi URL không hợp lệ: {}", err)),
        }
    }

    // Vẽ thẻ nhập dữ liệu đầu vào
    fn renderInputPane(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        renderCardContainer(ui, palette, |ui| {
            ui.label(
                RichText::new("Dữ liệu đầu vào")
                    .size(FONT_SIZE_SUBHEADING)
                    .strong(),
            );
            ui.add_space(8.0);
            egui::ScrollArea::vertical()
                .id_salt("formatter_input_scroll")
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.inputText)
                            .desired_width(f32::INFINITY)
                            .desired_rows(TEXT_AREA_MIN_ROWS)
                            .font(egui::TextStyle::Monospace),
                    );
                });
        });
    }

    // Vẽ thẻ hiển thị kết quả đầu ra
    fn renderOutputPane(&mut self, ui: &mut egui::Ui, palette: &ThemePalette) {
        renderCardContainer(ui, palette, |ui| {
            ui.label(
                RichText::new("Kết quả xử lý")
                    .size(FONT_SIZE_SUBHEADING)
                    .strong(),
            );
            ui.add_space(8.0);
            egui::ScrollArea::vertical()
                .id_salt("formatter_output_scroll")
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.outputText)
                            .desired_width(f32::INFINITY)
                            .desired_rows(JSON_SAMPLE_OUTPUT_ROWS)
                            .font(egui::TextStyle::Monospace),
                    );
                });
        });
    }
}

impl ToolModule for TextFormatterTool {
    fn toolId(&self) -> &'static str {
        "text_formatter"
    }

    fn toolTitle(&self) -> &'static str {
        "JSON / Base64 / URL"
    }

    fn toolShortTag(&self) -> &'static str {
        "Định dạng & Mã hóa nhanh"
    }

    fn toolDescription(&self) -> &'static str {
        "Định dạng, kiểm tra cú pháp JSON và mã hóa/giải mã nhanh Base64, URL."
    }

    fn searchKeywords(&self) -> &'static [&'static str] {
        &["json", "base64", "url", "format", "encode", "decode", "minify"]
    }

    fn renderToolUi(
        &mut self,
        ui: &mut egui::Ui,
        _eguiCtx: &egui::Context,
        palette: &ThemePalette,
    ) {
        renderToolHeader(
            ui,
            palette,
            "Định dạng JSON & Mã hóa Base64 / URL",
            self.toolDescription(),
            None,
        );

        renderCardContainer(ui, palette, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.activeMode, FormatterMode::JsonFormat, "JSON");
                ui.selectable_value(&mut self.activeMode, FormatterMode::Base64Codec, "Base64");
                ui.selectable_value(&mut self.activeMode, FormatterMode::UrlCodec, "URL");
                ui.separator();

                match self.activeMode {
                    FormatterMode::JsonFormat => {
                        if ui.add(buildPrimaryButton(palette, "Định dạng (Pretty)")).clicked() {
                            self.formatJsonPretty();
                        }
                        if ui.add(buildSecondaryButton(palette, "Nén gọn (Minify)")).clicked() {
                            self.minifyJson();
                        }
                    }
                    FormatterMode::Base64Codec => {
                        if ui.add(buildPrimaryButton(palette, "Mã hóa Base64")).clicked() {
                            self.encodeBase64();
                        }
                        if ui.add(buildSecondaryButton(palette, "Giải mã Base64")).clicked() {
                            self.decodeBase64();
                        }
                    }
                    FormatterMode::UrlCodec => {
                        if ui.add(buildPrimaryButton(palette, "Mã hóa URL")).clicked() {
                            self.encodeUrl();
                        }
                        if ui.add(buildSecondaryButton(palette, "Giải mã URL")).clicked() {
                            self.decodeUrl();
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(buildSecondaryButton(palette, "Xóa trắng")).clicked() {
                        self.inputText.clear();
                        self.outputText.clear();
                        self.errorMessage = None;
                    }
                    if ui.add(buildSecondaryButton(palette, "Sao chép kết quả")).clicked() {
                        ui.ctx().copy_text(self.outputText.clone());
                    }
                });
            });
        });

        if let Some(errMsg) = &self.errorMessage {
            ui.add_space(8.0);
            ui.colored_label(palette.dangerFg, errMsg);
        }

        ui.add_space(12.0);
        if ui.available_width() >= TWO_COL_BREAKPOINT {
            ui.columns(2, |columns| {
                self.renderInputPane(&mut columns[0], palette);
                self.renderOutputPane(&mut columns[1], palette);
            });
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.renderInputPane(ui, palette);
                ui.add_space(12.0);
                self.renderOutputPane(ui, palette);
            });
        }
    }
}
