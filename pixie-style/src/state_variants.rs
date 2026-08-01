use crate::style::PaintStyle;

#[derive(Debug, Clone, Default)]
pub struct StateVariants {
    pub hover: Option<PaintStyle>,
    pub active: Option<PaintStyle>,
    pub focused: Option<PaintStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionState {
    Normal,
    Hover,
    Active,
    Focused,
}

impl StateVariants {
    pub fn resolve<'a>(&'a self, base: &'a PaintStyle, state: InteractionState) -> &'a PaintStyle {
        match state {
            InteractionState::Hover => self.hover.as_ref().unwrap_or(base),
            InteractionState::Active => self.active.as_ref().unwrap_or(base),
            InteractionState::Focused => self.focused.as_ref().unwrap_or(base),
            InteractionState::Normal => base,
        }
    }
}
