use pixie_engine::window::PixieApp;
use pixie_node::{div, header, h1, h3, p, button, badge, card, PixelNode};
use pixie_node::stylesheet::{background, target};
use pixie_style::{Style, Color, Display, FlexDirection, AlignItems, Sides, px};
use std::panic::AssertUnwindSafe;

pub struct DemoApp;

fn page_style(width: f32, height: f32) -> Style {
    let mut s = Style::default();
    s.structural.display = Display::Flex;
    s.structural.flex_direction = FlexDirection::Column;
    s.structural.width = px(width);
    s.structural.height = px(height);
    s.structural.padding = Sides::all(px(24.0));
    s.structural.row_gap = px(20.0);
    s
}

fn row_style(gap: f32) -> Style {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Row;
    s.structural.align_items = AlignItems::Center;
    s.structural.column_gap = px(gap);
    s
}

impl PixieApp for DemoApp {
    fn build(&mut self, width: u32, height: u32) -> PixelNode {
        div()
            .style(page_style(width as f32, height as f32))
            .child((
                header().ref_id("header").child(h1("Pixie Design Playground").ref_id("title")),
                div().ref_id("hero").child(p("Edit this file, save, and see it update — this is your canvas now.").ref_id("hero-text")),
                div().style(row_style(16.0)).child((
                    card().ref_id("card-1").child((h3("Fast").ref_id("card-1-title"), p("Real taffy layout, real cosmic-text shaping.").ref_id("card-1-body"))),
                    card().ref_id("card-2").child((h3("Typed").ref_id("card-2-title"), p("Styles are Rust structs, not strings.").ref_id("card-2-body"))),
                )),
                div().style(row_style(12.0)).child((
                    button("Get Started").ref_id("cta"),
                    badge("v0.1").ref_id("version-badge"),
                )),
            ))
            .stylesheet((
                background(Color::rgb(245, 246, 250)),
                target("header").background(Color::rgb(255, 255, 255)).padding(16.0),
                target("title").font_size(28.0).color(Color::hex("#1a1a2e").unwrap()),
                target("hero-text").color(Color::rgb(90, 90, 100)).font_size(16.0),
                target("card-1").background(Color::rgb(255, 255, 255)).radius(12.0).padding(16.0),
                target("card-2").background(Color::rgb(255, 255, 255)).radius(12.0).padding(16.0),
                target("cta").background(Color::hex("#3366ff").unwrap()).color(Color::rgb(255, 255, 255)).radius(8.0).padding(12.0),
                target("version-badge").background(Color::rgb(230, 230, 235)).color(Color::rgb(100, 100, 110)),
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

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        pixie_engine::window::run_android(DemoApp, app);
    }));

    if let Err(e) = result {
        log::error!("PIXIE: run_android panicked, caught safely: {:?}", e);
    }
}
