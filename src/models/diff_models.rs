// Các định nghĩa kiểu dữ liệu dùng cho chức năng So sánh văn bản & Code (Diff Checker)

// Loại thao tác so sánh trên từng dòng
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffOp {
    Equal,
    Insert,
    Delete,
}

// Thông tin chi tiết một dòng sau khi so sánh diff
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffLine {
    pub op: DiffOp,
    pub text: String,
    pub oldLineNum: Option<usize>,
    pub newLineNum: Option<usize>,
}

// Chế độ hiển thị kết quả diff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffViewMode {
    Split,
    Unified,
}

// Thống kê tổng hợp số dòng thêm, xóa, không đổi
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DiffSummary {
    pub additions: usize,
    pub deletions: usize,
    pub unchanged: usize,
}

// Cấu hình các tùy chọn so sánh
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiffOptions {
    pub ignoreWhitespace: bool,
    pub caseSensitive: bool,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            ignoreWhitespace: false,
            caseSensitive: true,
        }
    }
}
