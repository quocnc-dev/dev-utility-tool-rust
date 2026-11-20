use eframe::egui;

use crate::app::theme::ThemePalette;
use crate::config::env_config::AppEnvConfig;

// Giao diện chung bắt buộc mọi module công cụ trong DevUtility Hub phải triển khai
pub trait ToolModule {
    // Mã định danh duy nhất của công cụ
    fn toolId(&self) -> &'static str;

    // Tên hiển thị trên thanh điều hướng bên trái
    fn toolTitle(&self) -> &'static str;

    // Nhãn phụ ngắn hiển thị dưới tên công cụ trên thanh điều hướng
    fn toolShortTag(&self) -> &'static str;

    // Mô tả chi tiết chức năng chính của công cụ
    fn toolDescription(&self) -> &'static str;

    // Danh sách từ khóa phục vụ tìm kiếm nhanh trên Sidebar
    fn searchKeywords(&self) -> &'static [&'static str];

    // Đồng bộ lại cấu hình mặc định khi người dùng thay đổi trong trang Cài đặt
    fn applyUpdatedConfig(&mut self, _config: &AppEnvConfig) {}

    // Vẽ giao diện tương tác của công cụ với bảng màu đang áp dụng
    fn renderToolUi(
        &mut self,
        ui: &mut egui::Ui,
        eguiCtx: &egui::Context,
        palette: &ThemePalette,
    );
}
