use crate::models::diff_models::{DiffLine, DiffOp, DiffOptions, DiffSummary};

// Dòng hiển thị song song cho chế độ xem Split View
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitDiffRow {
    pub left: Option<(usize, String, DiffOp)>,
    pub right: Option<(usize, String, DiffOp)>,
}

// Kết quả phân tích diff hoàn chỉnh gồm danh sách tuần tự, hàng split và tóm tắt
#[derive(Debug, Clone, Default)]
pub struct DiffAnalysisResult {
    pub unifiedLines: Vec<DiffLine>,
    pub splitRows: Vec<SplitDiffRow>,
    pub summary: DiffSummary,
}

// Chuẩn hóa một dòng văn bản phục vụ so sánh theo tùy chọn cấu hình
fn normalizeForCompare(line: &str, opts: &DiffOptions) -> String {
    let mut normalized = if opts.ignoreWhitespace {
        line.trim().to_string()
    } else {
        line.to_string()
    };

    if !opts.caseSensitive {
        normalized = normalized.to_lowercase();
    }
    normalized
}

// Thuật toán so sánh diff dòng theo dòng sử dụng quy hoạch động LCS (Longest Common Subsequence)
pub fn computeTextDiff(left: &str, right: &str, opts: &DiffOptions) -> DiffAnalysisResult {
    if left.is_empty() && right.is_empty() {
        return DiffAnalysisResult::default();
    }

    let leftRaw: Vec<&str> = if left.is_empty() {
        Vec::new()
    } else {
        left.lines().collect()
    };

    let rightRaw: Vec<&str> = if right.is_empty() {
        Vec::new()
    } else {
        right.lines().collect()
    };

    let leftNorm: Vec<String> = leftRaw
        .iter()
        .map(|&l| normalizeForCompare(l, opts))
        .collect();
    let rightNorm: Vec<String> = rightRaw
        .iter()
        .map(|&r| normalizeForCompare(r, opts))
        .collect();

    let n = leftNorm.len();
    let m = rightNorm.len();

    // Giới hạn an toàn chống tràn bộ nhớ ma trận DP cho văn bản cực lớn
    const MAX_MATRIX_CELLS: usize = 4_000_000;
    if (n + 1).saturating_mul(m + 1) > MAX_MATRIX_CELLS {
        return computeFallbackDiff(&leftRaw, &rightRaw);
    }

    let cols = m + 1;
    let mut dp = vec![0u32; (n + 1) * cols];

    for i in 0..n {
        for j in 0..m {
            let idxCurrent = (i + 1) * cols + (j + 1);
            if leftNorm[i] == rightNorm[j] {
                dp[idxCurrent] = dp[i * cols + j] + 1;
            } else {
                let fromTop = dp[i * cols + (j + 1)];
                let fromLeft = dp[(i + 1) * cols + j];
                dp[idxCurrent] = fromTop.max(fromLeft);
            }
        }
    }

    // Truy vết ngược ma trận để xây dựng danh sách thao tác diff
    let mut rawOps: Vec<(DiffOp, String)> = Vec::with_capacity(n + m);
    let mut i = n;
    let mut j = m;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && leftNorm[i - 1] == rightNorm[j - 1] {
            rawOps.push((DiffOp::Equal, leftRaw[i - 1].to_string()));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i * cols + (j - 1)] >= dp[(i - 1) * cols + j]) {
            rawOps.push((DiffOp::Insert, rightRaw[j - 1].to_string()));
            j -= 1;
        } else if i > 0 {
            rawOps.push((DiffOp::Delete, leftRaw[i - 1].to_string()));
            i -= 1;
        }
    }

    rawOps.reverse();

    // Gán số dòng gốc và số dòng mới
    let mut unifiedLines = Vec::with_capacity(rawOps.len());
    let mut oldLine = 1usize;
    let mut newLine = 1usize;
    let mut summary = DiffSummary::default();

    for (op, text) in rawOps {
        match op {
            DiffOp::Equal => {
                unifiedLines.push(DiffLine {
                    op,
                    text,
                    oldLineNum: Some(oldLine),
                    newLineNum: Some(newLine),
                });
                oldLine += 1;
                newLine += 1;
                summary.unchanged += 1;
            }
            DiffOp::Delete => {
                unifiedLines.push(DiffLine {
                    op,
                    text,
                    oldLineNum: Some(oldLine),
                    newLineNum: None,
                });
                oldLine += 1;
                summary.deletions += 1;
            }
            DiffOp::Insert => {
                unifiedLines.push(DiffLine {
                    op,
                    text,
                    oldLineNum: None,
                    newLineNum: Some(newLine),
                });
                newLine += 1;
                summary.additions += 1;
            }
        }
    }

    let splitRows = buildSplitRows(&unifiedLines);

    DiffAnalysisResult {
        unifiedLines,
        splitRows,
        summary,
    }
}

// Xây dựng các hàng hiển thị song song cho chế độ Split View
fn buildSplitRows(lines: &[DiffLine]) -> Vec<SplitDiffRow> {
    let mut rows = Vec::new();
    let mut idx = 0;

    while idx < lines.len() {
        let item = &lines[idx];
        match item.op {
            DiffOp::Equal => {
                rows.push(SplitDiffRow {
                    left: Some((item.oldLineNum.unwrap_or(0), item.text.clone(), DiffOp::Equal)),
                    right: Some((item.newLineNum.unwrap_or(0), item.text.clone(), DiffOp::Equal)),
                });
                idx += 1;
            }
            DiffOp::Delete => {
                // Kiểm tra xem dòng kế tiếp có phải là Insert để ghép chung thành 1 hàng không
                if idx + 1 < lines.len() && lines[idx + 1].op == DiffOp::Insert {
                    let nextItem = &lines[idx + 1];
                    rows.push(SplitDiffRow {
                        left: Some((item.oldLineNum.unwrap_or(0), item.text.clone(), DiffOp::Delete)),
                        right: Some((
                            nextItem.newLineNum.unwrap_or(0),
                            nextItem.text.clone(),
                            DiffOp::Insert,
                        )),
                    });
                    idx += 2;
                } else {
                    rows.push(SplitDiffRow {
                        left: Some((item.oldLineNum.unwrap_or(0), item.text.clone(), DiffOp::Delete)),
                        right: None,
                    });
                    idx += 1;
                }
            }
            DiffOp::Insert => {
                rows.push(SplitDiffRow {
                    left: None,
                    right: Some((item.newLineNum.unwrap_or(0), item.text.clone(), DiffOp::Insert)),
                });
                idx += 1;
            }
        }
    }

    rows
}

// Cơ chế dự phòng khi kích thước dữ liệu vượt quá khả năng ma trận bộ nhớ
fn computeFallbackDiff(leftRaw: &[&str], rightRaw: &[&str]) -> DiffAnalysisResult {
    let mut unifiedLines = Vec::new();
    let mut summary = DiffSummary::default();

    for (i, line) in leftRaw.iter().enumerate() {
        unifiedLines.push(DiffLine {
            op: DiffOp::Delete,
            text: line.to_string(),
            oldLineNum: Some(i + 1),
            newLineNum: None,
        });
        summary.deletions += 1;
    }

    for (j, line) in rightRaw.iter().enumerate() {
        unifiedLines.push(DiffLine {
            op: DiffOp::Insert,
            text: line.to_string(),
            oldLineNum: None,
            newLineNum: Some(j + 1),
        });
        summary.additions += 1;
    }

    let splitRows = buildSplitRows(&unifiedLines);
    DiffAnalysisResult {
        unifiedLines,
        splitRows,
        summary,
    }
}
