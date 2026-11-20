#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

use dev_utility_hub::app::utility_app::DevUtilityApp;
use dev_utility_hub::config::app_constants::{
    APP_TITLE, WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH, WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH,
};
use dev_utility_hub::config::env_config::AppEnvConfig;
use dev_utility_hub::utils::font_loader::configureUnicodeFonts;

// Điểm khởi chạy chính của ứng dụng DevUtility Hub
fn main() -> eframe::Result<()> {
    let envConfig = AppEnvConfig::loadFromEnv();

    let nativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        nativeOptions,
        Box::new(move |creationCtx| {
            configureUnicodeFonts(&creationCtx.egui_ctx);
            Ok(Box::new(DevUtilityApp::new(envConfig)))
        }),
    )
}
