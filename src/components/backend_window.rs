use std::rc::Weak;

use eframe::{
    egui::{self, Align2, Window, vec2},
    egui_wgpu::RenderState,
};
use re_format::format_bytes;

pub struct BackendWindow {
    render_state: Weak<RenderState>,
}
impl BackendWindow {
    pub fn new(render_state: Weak<RenderState>) -> Self {
        Self { render_state }
    }

    pub fn ui(self, ui: &mut egui::Ui) {
        let backend_window = Window::new("Backend");
        let render_info = self.render_state.upgrade();
        let adapter_info = render_info.map(|render_info| render_info.adapter.get_info());
        let mem_use = re_memory::MemoryUse::capture();

        if let Some(adapter_info) = adapter_info {
            backend_window
                .default_open(false)
                .anchor(Align2::RIGHT_TOP, vec2(5.0, 5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Name: {}", adapter_info.name));
                        ui.label(format!("Vendor: {}", adapter_info.vendor));
                        ui.label(format!("Type: {:?}", adapter_info.device_type));
                        ui.label(format!("Driver: {}", adapter_info.driver));
                        ui.label(format!("Backend: {:?}", adapter_info.backend));
                        ui.separator();
                        if let Some(bytes) = mem_use.counted {
                            ui.label(format!("Heap usage: {}", format_bytes(bytes as f64)));
                        }
                        if let Some(bytes) = mem_use.resident {
                            ui.label(format!("RSS usage: {}", format_bytes(bytes as f64)));
                        }
                    });
                });
        }
    }
}
