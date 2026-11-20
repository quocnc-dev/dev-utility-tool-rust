use eframe::egui;
use eframe::egui::RichText;
use egui_extras::{Column, TableBuilder};

use crate::app::theme::ThemePalette;
use crate::app::tool_trait::ToolModule;
use crate::config::app_constants::{
    SEARCH_INPUT_WIDTH, TABLE_HEADER_HEIGHT, TABLE_ROW_HEIGHT,
};
use crate::models::port_models::{
    KillConfirmTarget, PortEntry, PortSortColumn, ProtocolFilter, TransportProtocol,
};
use crate::tools::port_manager_tool::PortManagerTool;
use crate::utils::ui_dialogs::{
    renderKillConfirmDialog, renderStatusBanner, KillDialogDecision,
};
use crate::utils::ui_helpers::{
    buildDangerButton, buildPrimaryButton, buildSecondaryButton, renderBadgePill,
    renderCardContainer, renderEmptyStateBox, renderToolHeader,
};

// Vẽ toàn bộ giao diện công cụ Quản lý Port & Kill Port
pub fn renderPortManagerUi(
    state: &mut PortManagerTool,
    ui: &mut egui::Ui,
    eguiCtx: &egui::Context,
    palette: &ThemePalette,
) {
    let visibleIndices = state.buildVisibleIndices();
    let countBadge = format!("{} / {} cổng", visibleIndices.len(), state.portEntries.len());

    renderToolHeader(
        ui,
        palette,
        "Quản lý Port & Giải phóng tiến trình",
        state.toolDescription(),
        Some(&countBadge),
    );

    renderCardContainer(ui, palette, |ui| {
        renderFilterToolbar(state, ui, palette);
    });
    ui.add_space(12.0);

    renderStatusBanner(ui, palette, &mut state.statusMessage);

    renderCardContainer(ui, palette, |ui| {
        if visibleIndices.is_empty() {
            renderEmptyStateBox(
                ui,
                palette,
                "Không có cổng nào khớp bộ lọc",
                "Thử xóa từ khóa tìm kiếm hoặc bỏ chọn bộ lọc 'Chỉ cổng LISTEN'.",
            );
        } else {
            renderPortsTable(state, ui, palette, &visibleIndices);
        }
    });

    match renderKillConfirmDialog(eguiCtx, palette, &state.pendingKillTarget) {
        KillDialogDecision::Confirmed => state.executePendingKill(),
        KillDialogDecision::Cancelled => state.pendingKillTarget = None,
        KillDialogDecision::None => {}
    }
}

// Vẽ thanh tìm kiếm, bộ lọc giao thức và các nút làm mới thoáng rộng
fn renderFilterToolbar(
    state: &mut PortManagerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Tìm kiếm:").color(palette.textSecondary));
        ui.add(
            egui::TextEdit::singleline(&mut state.searchQuery)
                .desired_width(SEARCH_INPUT_WIDTH)
                .hint_text("Port, tên process, PID, IP..."),
        );

        ui.separator();
        ui.selectable_value(&mut state.protocolFilter, ProtocolFilter::All, "Tất cả");
        ui.selectable_value(&mut state.protocolFilter, ProtocolFilter::TcpOnly, "TCP");
        ui.selectable_value(&mut state.protocolFilter, ProtocolFilter::UdpOnly, "UDP");
    });

    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.checkbox(&mut state.listeningOnly, "Chỉ cổng LISTEN");
        ui.add_space(8.0);
        ui.checkbox(
            &mut state.autoRefresh,
            format!("Tự động làm mới ({}s)", state.refreshIntervalSecs),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(buildPrimaryButton(palette, "Làm mới ngay")).clicked() {
                state.refreshPorts();
            }
            if !state.searchQuery.is_empty()
                && ui.add(buildSecondaryButton(palette, "Xóa lọc")).clicked()
            {
                state.searchQuery.clear();
            }
        });
    });
}

// Vẽ bảng ảo hóa danh sách cổng đang hoạt động theo chỉ số đã lọc
fn renderPortsTable(
    state: &mut PortManagerTool,
    ui: &mut egui::Ui,
    palette: &ThemePalette,
    visibleIndices: &[usize],
) {
    let mut sortTarget: Option<PortSortColumn> = None;
    let mut killCandidate: Option<KillConfirmTarget> = None;

    TableBuilder::new(ui)
        .id_salt("port_manager_table")
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(84.0).at_least(70.0))
        .column(Column::initial(170.0).at_least(120.0))
        .column(Column::initial(90.0).at_least(75.0))
        .column(Column::initial(175.0).at_least(110.0))
        .column(Column::initial(130.0).at_least(100.0))
        .column(Column::initial(85.0).at_least(70.0))
        .column(Column::remainder().at_least(150.0))
        .column(Column::initial(175.0).at_least(165.0))
        .header(TABLE_HEADER_HEIGHT, |mut header| {
            renderSortableHeader(&mut header, "Loại", PortSortColumn::Protocol, state, &mut sortTarget);
            header.col(|ui| {
                ui.strong("Địa chỉ cục bộ");
            });
            renderSortableHeader(&mut header, "Cổng", PortSortColumn::Port, state, &mut sortTarget);
            header.col(|ui| {
                ui.strong("Địa chỉ đích");
            });
            renderSortableHeader(&mut header, "Trạng thái", PortSortColumn::State, state, &mut sortTarget);
            renderSortableHeader(&mut header, "PID", PortSortColumn::Pid, state, &mut sortTarget);
            renderSortableHeader(&mut header, "Tiến trình", PortSortColumn::ProcessName, state, &mut sortTarget);
            header.col(|ui| {
                ui.strong("Thao tác");
            });
        })
        .body(|body| {
            body.rows(TABLE_ROW_HEIGHT, visibleIndices.len(), |mut row| {
                let entry = &state.portEntries[visibleIndices[row.index()]];
                renderPortRow(&mut row, palette, entry, &mut killCandidate);
            });
        });

    if let Some(col) = sortTarget {
        state.toggleSort(col);
    }
    if let Some(target) = killCandidate {
        state.pendingKillTarget = Some(target);
    }
}

// Vẽ tiêu đề cột có hỗ trợ nhấn để sắp xếp
fn renderSortableHeader(
    header: &mut egui_extras::TableRow<'_, '_>,
    label: &str,
    column: PortSortColumn,
    state: &PortManagerTool,
    sortTarget: &mut Option<PortSortColumn>,
) {
    header.col(|ui| {
        let indicator = if state.sortColumn == column {
            if state.sortAscending { " ▲" } else { " ▼" }
        } else {
            ""
        };
        if ui.button(format!("{}{}", label, indicator)).clicked() {
            *sortTarget = Some(column);
        }
    });
}

// Vẽ dữ liệu một dòng trong bảng cổng với nhãn trạng thái và nút lớn dễ bấm
fn renderPortRow(
    row: &mut egui_extras::TableRow<'_, '_>,
    palette: &ThemePalette,
    entry: &PortEntry,
    killCandidate: &mut Option<KillConfirmTarget>,
) {
    row.col(|ui| {
        let (fgColor, bgColor) = match entry.protocol {
            TransportProtocol::Tcp => (palette.infoFg, palette.infoSoftBg),
            TransportProtocol::Udp => (palette.warningFg, palette.warningSoftBg),
        };
        renderBadgePill(ui, entry.protocol.asLabel(), fgColor, bgColor);
    });
    row.col(|ui| {
        ui.monospace(&entry.localAddr);
    });
    row.col(|ui| {
        ui.label(RichText::new(entry.localPort.to_string()).monospace().strong());
    });
    row.col(|ui| {
        ui.label(RichText::new(&entry.remoteAddr).monospace().color(palette.textSecondary));
    });
    row.col(|ui| {
        if entry.isListening {
            renderBadgePill(ui, &entry.stateText, palette.successFg, palette.successSoftBg);
        } else {
            ui.label(RichText::new(&entry.stateText).color(palette.textSecondary));
        }
    });
    row.col(|ui| {
        let pidLabel = entry
            .pid
            .map(|pid| pid.to_string())
            .unwrap_or_else(|| "-".to_string());
        ui.monospace(pidLabel);
    });
    row.col(|ui| {
        let resp = ui.label(RichText::new(&entry.processName).strong());
        if !entry.exePath.is_empty() {
            resp.on_hover_text(&entry.exePath);
        }
    });
    row.col(|ui| {
        ui.horizontal(|ui| {
            if ui.add(buildSecondaryButton(palette, "Sao chép")).clicked() {
                ui.ctx().copy_text(entry.localPort.to_string());
            }
            if let Some(pidValue) = entry.pid {
                if ui.add(buildDangerButton(palette, "Kill")).clicked() {
                    *killCandidate = Some(KillConfirmTarget {
                        pid: pidValue,
                        processName: entry.processName.clone(),
                        portNumber: Some(entry.localPort),
                    });
                }
            }
        });
    });
}
