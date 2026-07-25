use std::rc::Rc;

use eframe::egui_wgpu::RenderState;
use eframe::{App, CreationContext, egui::CentralPanel};

use crate::components::backend_window::BackendWindow;
use crate::screens::home::HomeScreen;

pub struct EguiAppTest {
    render_state: Option<Rc<RenderState>>,
    home: Box<HomeScreen>,
}

impl EguiAppTest {
    pub fn new(creation_context: &CreationContext) -> Self {
        let render_state = creation_context
            .wgpu_render_state
            .as_ref()
            .map(|rs| Rc::new(rs.clone()));

        Self {
            render_state,
            home: Box::new(HomeScreen::new(20)),
        }
    }
}

impl App for EguiAppTest {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.render_backend_window(ui);
        CentralPanel::default().show(ui, |ui| self.home.ui(ui));
    }
}

impl EguiAppTest {
    fn render_backend_window(&mut self, ui: &mut eframe::egui::Ui) {
        let render_state = self.render_state.as_ref().map(Rc::downgrade);

        if let Some(render_state) = render_state {
            let window = BackendWindow::new(render_state);
            window.ui(ui);
        }
    }
}
