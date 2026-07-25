pub mod app;
pub mod array;
pub mod components;
pub mod memory;
pub mod screens;

pub mod prelude {
    use eframe::{NativeOptions, run_native};

    use crate::app::EguiAppTest;

    pub fn run_app() -> eframe::Result {
        run_native(
            "EguiAppTest",
            NativeOptions::default(),
            Box::new(|cc| Ok(Box::new(EguiAppTest::new(cc)))),
        )
    }
}
