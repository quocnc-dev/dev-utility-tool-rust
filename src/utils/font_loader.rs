use std::fs;
use std::sync::Arc;

use eframe::egui;

use crate::config::app_constants::{
    LINUX_FONT_PATHS, LINUX_MONO_FONT_PATHS, MACOS_FONT_PATHS, MACOS_MONO_FONT_PATHS,
    WINDOWS_FONT_PATHS, WINDOWS_MONO_FONT_PATHS,
};

// Nạp font Unicode từ hệ điều hành để hiển thị đầy đủ tiếng Việt có dấu trên Windows/macOS/Linux
pub fn configureUnicodeFonts(eguiCtx: &egui::Context) {
    let (proportionalPaths, monospacePaths) = getPlatformFontCandidates();
    let mut fontDefs = egui::FontDefinitions::default();

    if let Some(sansBytes) = loadFirstAvailableFont(proportionalPaths) {
        let fontKey = "system_unicode_sans".to_owned();
        fontDefs
            .font_data
            .insert(fontKey.clone(), Arc::new(egui::FontData::from_owned(sansBytes)));

        fontDefs
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, fontKey.clone());

        fontDefs
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push(fontKey);
    }

    if let Some(monoBytes) = loadFirstAvailableFont(monospacePaths) {
        let monoKey = "system_unicode_mono".to_owned();
        fontDefs
            .font_data
            .insert(monoKey.clone(), Arc::new(egui::FontData::from_owned(monoBytes)));

        fontDefs
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, monoKey);
    }

    eguiCtx.set_fonts(fontDefs);
}

// Chọn danh sách đường dẫn font tương ứng với hệ điều hành đang chạy
fn getPlatformFontCandidates() -> (&'static [&'static str], &'static [&'static str]) {
    if cfg!(target_os = "windows") {
        (WINDOWS_FONT_PATHS, WINDOWS_MONO_FONT_PATHS)
    } else if cfg!(target_os = "macos") {
        (MACOS_FONT_PATHS, MACOS_MONO_FONT_PATHS)
    } else {
        (LINUX_FONT_PATHS, LINUX_MONO_FONT_PATHS)
    }
}

// Đọc dữ liệu file font đầu tiên tồn tại và mở được trên hệ thống
fn loadFirstAvailableFont(candidatePaths: &[&str]) -> Option<Vec<u8>> {
    for fontPath in candidatePaths {
        if let Ok(fontBytes) = fs::read(fontPath) {
            return Some(fontBytes);
        }
    }
    None
}
