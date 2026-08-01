#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    Background,
    BorderColor,
    Opacity,
    BorderRadius,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ease {
    Linear,
    EaseInOut,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transition {
    pub property: Property,
    pub duration_ms: u32,
    pub ease: Ease,
}
