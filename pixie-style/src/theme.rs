use crate::color::Color;
use std::collections::HashMap;

/// A value that's either a literal, or a named reference into the
/// current Theme — the CSS-custom-properties equivalent. Style fields
/// that want theming (background, text color, border color) use this
/// instead of a bare Color.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorValue {
    Literal(Color),
    Token(&'static str),
}

impl ColorValue {
    pub fn literal(c: Color) -> Self {
        ColorValue::Literal(c)
    }
    pub fn token(name: &'static str) -> Self {
        ColorValue::Token(name)
    }
}

impl Default for ColorValue {
    fn default() -> Self {
        ColorValue::Literal(Color::TRANSPARENT)
    }
}

/// Named color registry — define "brand-primary" once, reference it
/// from every Style that wants it. Swapping the whole Theme (e.g. for
/// dark mode) updates every element referencing a token, with no need
/// to touch the elements themselves.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    colors: HashMap<&'static str, Color>,
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_color(mut self, name: &'static str, color: Color) -> Self {
        self.colors.insert(name, color);
        self
    }

    /// Resolves a ColorValue against this theme. Unknown tokens fall
    /// back to transparent rather than panicking — a missing token is
    /// a real bug to catch in dev tooling later, not a crash.
    pub fn resolve(&self, value: &ColorValue) -> Color {
        match value {
            ColorValue::Literal(c) => *c,
            ColorValue::Token(name) => self.colors.get(name).copied().unwrap_or(Color::TRANSPARENT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_resolves_to_itself() {
        let theme = Theme::new();
        let v = ColorValue::literal(Color::rgb(10, 20, 30));
        assert_eq!(theme.resolve(&v), Color::rgb(10, 20, 30));
    }

    #[test]
    fn token_resolves_against_theme() {
        let theme = Theme::new().set_color("brand-primary", Color::hex("#3366ff").unwrap());
        let v = ColorValue::token("brand-primary");
        assert_eq!(theme.resolve(&v), Color::hex("#3366ff").unwrap());
    }

    #[test]
    fn changing_theme_changes_every_reference() {
        let light = Theme::new().set_color("bg", Color::rgb(255, 255, 255));
        let dark = Theme::new().set_color("bg", Color::rgb(20, 20, 20));
        let v = ColorValue::token("bg");
        assert_ne!(light.resolve(&v), dark.resolve(&v));
    }

    #[test]
    fn unknown_token_falls_back_to_transparent_not_a_panic() {
        let theme = Theme::new();
        let v = ColorValue::token("does-not-exist");
        assert_eq!(theme.resolve(&v), Color::TRANSPARENT);
    }
}
