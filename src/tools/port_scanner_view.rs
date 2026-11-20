use eframe::egui;
use eframe::egui::RichText;
use egui_extras::{Column, TableBuilder};

use crate::app::theme::ThemePalette;
use crate::config::app_constants::{
    HOST_INPUT_WIDTH, MAX_CONCURRENCY, MAX_TIMEOUT_MS, MIN_CONCURRENCY, MIN_TIMEOUT_MS,
    PORT_SPEC_INPUT_WIDTH, PROGRESS_BAR_HEIGHT, SEARCH_INPUT_WIDTH, TABLE_HEADER_HEIGHT,
    TABLE_ROW_HEIGHT,
};
use crate::models::port_models::ScanPresetMode;
use crate::tools::port_scanner_tool::PortScannerTool;
use crate::utils::ui_dialogs::renderStatusBanner;
use crate::utils::ui_helpers::{
    buildDangerButton, buildPrimaryButton, buildSecondaryButton, renderBadgePill,
    renderCardContainer, renderEmptyStateBox, renderToolHeader,
};

// Vẽ giao diện điều khiển và bảng kết quả của Port Scanner
pub fn renderPortScannerUi(
    state: &mut PortScannerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
) {
    let openCount = state.scanResults.iter().filter(|item| item.isOpen).count();
    let countBadge = format!("{} cổng đang mở", openCount);

    renderToolHeader(
        ui,
        palette,
        "Quét Port đa luồng (Port Scanner)",
        "Kiểm tra nhanh các cổng dịch vụ Developer hoặc dải cổng tùy chỉnh không làm đứng giao diện.",
        Some(&countBadge),
    );

    renderCardContainer(ui, palette, |ui| {
        renderScanConfigForm(state, ui, palette);
        renderProgressBar(state, ui);
    });
    ui.add_space(12.0);

    renderStatusBanner(ui, palette, &mut state.statusMessage);

    let filteredIndices = state.buildFilteredIndices();
    renderCardContainer(ui, palette, |ui| {
        renderResultFilterBar(state, ui, palette, filteredIndices.len());
        ui.add_space(10.0);
        if filteredIndices.is_empty() {
            renderEmptyStateBox(
                ui,
                palette,
                "Chưa có kết quả phù hợp",
                "Nhấn 'Bắt đầu quét' để kiểm tra cổng hoặc điều chỉnh bộ lọc phía trên.",
            );
        } else {
            renderResultsTable(state, ui, palette, &filteredIndices);
        }
    });
}

// Vẽ biểu mẫu nhập Host, chế độ cổng, Timeout và Số luồng
fn renderScanConfigForm(
    state: &mut PortScannerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Host / IP:").color(palette.textSecondary));
        ui.add_enabled(
            !state.isScanning,
            egui::TextEdit::singleline(&mut state.hostInput)
                .desired_width(HOST_INPUT_WIDTH)
                .hint_text("127.0.0.1 hoặc localhost"),
        );

        ui.separator();
        ui.add_enabled_ui(!state.isScanning, |ui| {
            ui.radio_value(
                &mut state.presetMode,
                ScanPresetMode::CommonDevPorts,
                "Cổng Dev phổ biến",
            );
            ui.radio_value(
                &mut state.presetMode,
                ScanPresetMode::CustomRange,
                "Dải cổng tùy chỉnh",
            );
        });
    });

    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        let isCustomMode = state.presetMode == ScanPresetMode::CustomRange && !state.isScanning;
        ui.label(RichText::new("Dải cổng:").color(palette.textSecondary));
        ui.add_enabled(
            isCustomMode,
            egui::TextEdit::singleline(&mut state.customPortSpec)
                .desired_width(PORT_SPEC_INPUT_WIDTH)
                .hint_text("Ví dụ: 80, 443, 3000-3010"),
        );

        ui.label(RichText::new("Timeout (ms):").color(palette.textSecondary));
        ui.add_enabled(
            !state.isScanning,
            egui::DragValue::new(&mut state.timeoutMsInput)
                .range(MIN_TIMEOUT_MS..=MAX_TIMEOUT_MS)
                .speed(10.0),
        );

        ui.label(RichText::new("Số luồng:").color(palette.textSecondary));
        ui.add_enabled(
            !state.isScanning,
            egui::DragValue::new(&mut state.concurrencyInput)
                .range(MIN_CONCURRENCY..=MAX_CONCURRENCY)
                .speed(1.0),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !state.isScanning {
                if ui.add(buildPrimaryButton(palette, "Bắt đầu quét")).clicked() {
                    state.startScan();
                }
            } else if ui.add(buildDangerButton(palette, "Dừng quét")).clicked() {
                state.stopScan();
            }
        });
    });
}

// Vẽ thanh tiến trình quét lớn dễ theo dõi
fn renderProgressBar(state: &PortScannerTool, ui: &mut egui::Ui) {
    if state.totalPlannedPorts == 0 {
        return;
    }
    ui.add_space(10.0);
    let progress = (state.scannedCount as f32) / (state.totalPlannedPorts as f32);
    let openCount = state.scanResults.iter().filter(|item| item.isOpen).count();
    let progressLabel = format!(
        "Tiến độ: {}/{} cổng — Mở: {} cổng",
        state.scannedCount, state.totalPlannedPorts, openCount
    );
    ui.add(
        egui::ProgressBar::new(progress)
            .desired_height(PROGRESS_BAR_HEIGHT)
            .text(progressLabel),
    );
}

// Vẽ thanh lọc kết quả và nút sao chép các cổng đang mở
fn renderResultFilterBar(
    state: &mut PortScannerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    visibleCount: usize,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Lọc kết quả:").color(palette.textSecondary));
        ui.add(
            egui::TextEdit::singleline(&mut state.resultFilterText)
                .desired_width(SEARCH_INPUT_WIDTH)
                .hint_text("Số cổng hoặc tên dịch vụ..."),
        );
        ui.checkbox(&mut state.openOnlyFilter, "Chỉ hiện cổng đang mở");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(buildSecondaryButton(palette, "Sao chép cổng mở"))
                .clicked()
            {
                let openPortsText: Vec<String> = state
                    .scanResults
                    .iter()
                    .filter(|item| item.isOpen)
                    .map(|item| item.port.to_string())
                    .collect();
                ui.ctx().copy_text(openPortsText.join(", "));
            }
            ui.label(
                RichText::new(format!("Hiển thị: {} mục", visibleCount))
                    .color(palette.textSecondary),
            );
        });
    });
}

// Vẽ bảng ảo hóa kết quả quét cổng theo chỉ số đã lọc
fn renderResultsTable(
    state: &PortScannerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    filteredIndices: &[usize],
) {
    TableBuilder::new(ui)
        .id_salt("port_scanner_table")
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(110.0).at_least(85.0))
        .column(Column::initial(135.0).at_least(110.0))
        .column(Column::initial(120.0).at_least(95.0))
        .column(Column::remainder().at_least(180.0))
        .header(TABLE_HEADER_HEIGHT, |mut header| {
            header.col(|ui| {
                ui.strong("Cổng");
            });
            header.col(|ui| {
                ui.strong("Trạng thái");
            });
            header.col(|ui| {
                ui.strong("Độ trễ");
            });
            header.col(|ui| {
                ui.strong("Dịch vụ nhận diện");
            });
        })
        .body(|body| {
            body.rows(TABLE_ROW_HEIGHT, filteredIndices.len(), |mut row| {
                let item = &state.scanResults[filteredIndices[row.index()]];
                row.col(|ui| {
                    ui.label(RichText::new(item.port.to_string()).monospace().strong());
                });
                row.col(|ui| {
                    if item.isOpen {
                        renderBadgePill(ui, "ĐANG MỞ", palette.successFg, palette.successSoftBg);
                    } else {
                        renderBadgePill(ui, "Đóng", palette.textSecondary, palette.controlBg);
                    }
                });
                row.col(|ui| {
                    let latencyText = item
                        .latencyMs
                        .map(|ms| format!("{} ms", ms))
                        .unwrap_or_else(|| "-".to_string());
                    ui.monospace(latencyText);
                });
                row.col(|ui| {
                    ui.label(&item.serviceName);
                });
            });
        });
}
