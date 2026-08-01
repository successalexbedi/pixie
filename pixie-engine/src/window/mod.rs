pub mod event_loop;
pub mod desktop;
pub mod android;
pub mod multi_window;
pub mod tray_menu;

pub use event_loop::{App, PixieApp, run_desktop};

#[cfg(target_os = "android")]
pub use event_loop::run_android;
