#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthValue {
    Px(f32),
    Pct(f32),
    Fr(f32),
    Auto,
}

pub fn px(value: f32) -> LengthValue {
    LengthValue::Px(value)
}
pub fn pct(value: f32) -> LengthValue {
    LengthValue::Pct(value)
}
pub fn fr(value: f32) -> LengthValue {
    LengthValue::Fr(value)
}

impl Default for LengthValue {
    fn default() -> Self {
        LengthValue::Auto
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sides {
    pub top: LengthValue,
    pub right: LengthValue,
    pub bottom: LengthValue,
    pub left: LengthValue,
}

impl Sides {
    pub fn all(v: LengthValue) -> Sides {
        Sides {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
    pub fn symmetric(vertical: LengthValue, horizontal: LengthValue) -> Sides {
        Sides {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
    pub fn zero() -> Sides {
        Sides::all(LengthValue::Px(0.0))
    }
}

impl Default for Sides {
    fn default() -> Self {
        Sides::zero()
    }
}
