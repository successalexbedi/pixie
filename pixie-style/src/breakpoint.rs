#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    Sm,
    Md,
    Lg,
}

impl Breakpoint {
    pub fn from_width(width_px: f32) -> Breakpoint {
        if width_px < 640.0 {
            Breakpoint::Sm
        } else if width_px < 1024.0 {
            Breakpoint::Md
        } else {
            Breakpoint::Lg
        }
    }
}
