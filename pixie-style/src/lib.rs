pub mod breakpoint;
pub mod color;
pub mod state_variants;
pub mod style;
pub mod theme;
pub mod transition;
pub mod units;

pub use breakpoint::Breakpoint;
pub use color::Color;
pub use state_variants::{InteractionState, StateVariants};
pub use style::{
    AlignItems, BorderRadiusPerCorner, BoxShadow, BoxSizing, Cursor, Display, FlexDirection,
    FlexWrap, FontStyle, FontWeight, JustifyContent, Overflow, PaintStyle, Position,
    StructuralStyle, Style, TextAlign, TextDecoration, Transform,
};
pub use theme::{ColorValue, Theme};
pub use transition::{Ease, Property, Transition};
pub use units::{fr, pct, px, LengthValue, Sides};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn px_and_pct_are_distinct_variants() {
        assert_eq!(px(16.0), LengthValue::Px(16.0));
        assert_eq!(pct(50.0), LengthValue::Pct(50.0));
        assert_ne!(px(16.0), pct(16.0));
    }

    #[test]
    fn hex_parses_correctly() {
        let c = Color::hex("#ff3366").unwrap();
        assert_eq!(c, Color::rgba(0xff, 0x33, 0x66, 255));
    }

    #[test]
    fn hex_handles_invalid_non_ascii_safely() {
        assert!(Color::hex("#ff🦀66").is_err());
    }

    #[test]
    fn style_defaults_are_sane() {
        let s = Style::default();
        assert_eq!(s.structural.display, Display::Flex);
        assert_eq!(s.paint.opacity, 1.0);
    }

    #[test]
    fn breakpoint_thresholds() {
        assert_eq!(Breakpoint::from_width(400.0), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_width(800.0), Breakpoint::Md);
        assert_eq!(Breakpoint::from_width(1200.0), Breakpoint::Lg);
    }

    #[test]
    fn state_variant_resolves_hover_override() {
        let base = PaintStyle::default();
        let mut hover = base.clone();
        hover.opacity = 0.8;
        let sv = StateVariants {
            hover: Some(hover),
            ..Default::default()
        };
        let resolved = sv.resolve(&base, InteractionState::Hover);
        assert_eq!(resolved.opacity, 0.8);
    }

    #[test]
    fn box_shadow_stacks_like_css() {
        let mut s = Style::default();
        s.paint.box_shadow.push(BoxShadow {
            offset_x: 0.0,
            offset_y: 4.0,
            blur: 8.0,
            spread: 0.0,
            color: Color::rgba(0, 0, 0, 60),
            inset: false,
        });
        assert_eq!(s.paint.box_shadow.len(), 1);
    }

    #[test]
    fn grid_tracks_use_fr() {
        let mut s = Style::default();
        s.structural.display = Display::Grid;
        s.structural.grid_template_columns = vec![fr(1.0), fr(2.0), px(100.0)];
        assert_eq!(s.structural.grid_template_columns.len(), 3);
    }
}
