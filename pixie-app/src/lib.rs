use pixie_engine::window::PixieApp;
use pixie_node::{div, h1, PixelNode};
use pixie_style::{Style, Color, Display, FlexDirection, Sides, px};
use std::panic::AssertUnwindSafe;

pub struct DemoApp;

impl PixieApp for DemoApp {
    fn build(&mut self, width: u32, height: u32) -> PixelNode {
        let mut root_style = Style::default();
        root_style.structural.display = Display::Flex;
        root_style.structural.flex_direction = FlexDirection::Column;
        root_style.structural.width = px(width as f32);
        root_style.structural.height = px(height as f32);
        root_style.structural.padding = Sides::all(px(24.0));
        root_style.structural.row_gap = px(16.0);
        root_style.paint.background = Color::rgb(20, 20, 40);

        let mut text_style = Style::default();
        text_style.structural.font_size = 32.0;
        text_style.paint.color = Color::rgb(255, 255, 255);

        let mut box_a = Style::default();
        box_a.structural.width = px(300.0);
        box_a.structural.height = px(80.0);
        box_a.paint.background = Color::hex("#3366ff").unwrap();

        div().style(root_style).child((
            h1("Hello, Pixie!").style(text_style),
            div().style(box_a),
        ))
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let crash_path = app.external_data_path().map(|p| p.join("pixie_crash.txt"));
    if let Some(path) = crash_path.clone() {
        let _ = std::fs::write(&path, "android_main started, no crash yet\n");
    }
    std::panic::set_hook(Box::new({
        let crash_path = crash_path.clone();
        move |info| {
            log::error!("PIXIE PANIC: {}", info);
            if let Some(path) = &crash_path {
                let _ = std::fs::write(path, format!("PIXIE PANIC: {}\n", info));
            }
        }
    }));

    log::info!("PIXIE: android_main starting");

    // The real fix: a panic crossing an un-caught FFI boundary is
    // undefined behavior, not just "the app crashes." catch_unwind
    // guarantees the panic dies HERE, inside safe Rust, never
    // touching the C caller's stack at all.
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        pixie_engine::window::run_android(DemoApp, app);
    }));

    if let Err(e) = result {
        log::error!("PIXIE: run_android panicked, caught safely: {:?}", e);
    }
}
