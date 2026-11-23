use eframe::egui;

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{DEFAULT_DIFF_LEFT_SAMPLE, DEFAULT_DIFF_RIGHT_SAMPLE};
use crate::models::diff_models::{DiffOptions, DiffViewMode};
use crate::services::diff_service::{computeTextDiff, DiffAnalysisResult};
use crate::tools::diff_checker_view::renderDiffCheckerUi;

// Công cụ so sánh sự khác biệt giữa hai văn bản hoặc mã nguồn (Diff Checker)
pub struct DiffCheckerTool {
    pub leftText: String,
    pub rightText: String,
    pub options: DiffOptions,
    pub viewMode: DiffViewMode,
    pub diffResult: DiffAnalysisResult,
    pub hasComputed: bool,
}

impl DiffCheckerTool {
    // Khởi tạo công cụ với cấu hình mặc định và nạp sẵn mẫu code để xem ngay
    pub fn new() -> Self {
        let mut tool = Self {
            leftText: DEFAULT_DIFF_LEFT_SAMPLE.to_string(),
            rightText: DEFAULT_DIFF_RIGHT_SAMPLE.to_string(),
            options: DiffOptions::default(),
            viewMode: DiffViewMode::Split,
            diffResult: DiffAnalysisResult::default(),
            hasComputed: false,
        };
        tool.executeDiff();
        tool
    }

    // Thực hiện so sánh hai văn bản
    pub fn executeDiff(&mut self) {
        self.diffResult = computeTextDiff(&self.leftText, &self.rightText, &self.options);
        self.hasComputed = true;
    }

    // Hoán đổi vị trí giữa văn bản gốc và văn bản sửa đổi
    pub fn swapInputs(&mut self) {
        std::mem::swap(&mut self.leftText, &mut self.rightText);
        self.executeDiff();
    }

    // Xóa trắng toàn bộ nội dung cả hai ô nhập
    pub fn clearAll(&mut self) {
        self.leftText.clear();
        self.rightText.clear();
        self.diffResult = DiffAnalysisResult::default();
        self.hasComputed = false;
    }

    // Nạp lại dữ liệu văn bản mẫu
    pub fn loadSample(&mut self) {
        self.leftText = DEFAULT_DIFF_LEFT_SAMPLE.to_string();
        self.rightText = DEFAULT_DIFF_RIGHT_SAMPLE.to_string();
        self.executeDiff();
    }
}

impl ToolModule for DiffCheckerTool {
    fn toolId(&self) -> &'static str {
        "diff_checker"
    }

    fn toolTitle(&self) -> &'static str {
        "Diff Checker"
    }

    fn toolShortTag(&self) -> &'static str {
        "So sánh Code/Text"
    }

    fn toolDescription(&self) -> &'static str {
        "So sánh và trực quan hóa sự khác biệt giữa hai đoạn văn bản hoặc mã nguồn"
    }

    fn searchKeywords(&self) -> &'static [&'static str] {
        &[
            "diff",
            "checker",
            "so sánh",
            "compare",
            "text diff",
            "code diff",
            "khác biệt",
        ]
    }

    fn renderToolUi(&mut self, ui: &mut egui::Ui, _eguiCtx: &egui::Context, palette: &ThemePalette) {
        renderDiffCheckerUi(self, ui, palette);
    }
}
