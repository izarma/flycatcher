use eframe::run_native;
use gui::chat_app::ChatApp;
use tokio::runtime::Runtime;

pub mod gui;

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    let tokio_runtime = Runtime::new().unwrap();
    run_native(
        "FlyCatcher",
        native_options,
        Box::new(|cc| Ok(Box::new(ChatApp::new(cc)))),
    )
    .unwrap();
}
