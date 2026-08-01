use crate::node::{div, span, PixelNode};
use pixie_style::{Style, Color, BorderRadiusPerCorner, Sides, px, BoxShadow, FlexDirection};
use std::borrow::Cow;

pub fn card() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    s.structural.row_gap = px(8.0);
    s.paint.background = Color::rgb(255, 255, 255);
    s.paint.border_radius = BorderRadiusPerCorner::all(12.0);
    s.structural.padding = Sides::all(px(16.0));
    s.paint.box_shadow.push(BoxShadow {
        offset_x: 0.0, offset_y: 2.0, blur: 6.0, spread: 0.0,
        color: Color::rgba(0, 0, 0, 30), inset: false,
    });
    div().style(s)
}

pub fn badge(text: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.paint.background = Color::rgb(230, 230, 230);
    s.structural.padding = Sides::symmetric(px(2.0), px(8.0));
    s.paint.border_radius = BorderRadiusPerCorner::all(999.0);
    s.structural.font_size = 12.0;
    span(text).style(s)
}

pub fn chip(text: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.paint.background = Color::rgb(240, 240, 245);
    s.structural.padding = Sides::symmetric(px(4.0), px(12.0));
    s.paint.border_radius = BorderRadiusPerCorner::all(16.0);
    span(text).style(s)
}

pub fn avatar(_src: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.structural.width = px(40.0);
    s.structural.height = px(40.0);
    s.paint.border_radius = BorderRadiusPerCorner::all(20.0);
    div().style(s)
}

pub fn tooltip(text: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.paint.background = Color::rgb(30, 30, 30);
    s.paint.color = Color::rgb(255, 255, 255);
    s.structural.padding = Sides::symmetric(px(4.0), px(8.0));
    s.paint.border_radius = BorderRadiusPerCorner::all(4.0);
    s.structural.font_size = 12.0;
    span(text).style(s)
}

pub fn spinner() -> PixelNode {
    let mut s = Style::default();
    s.structural.width = px(24.0);
    s.structural.height = px(24.0);
    s.paint.border_radius = BorderRadiusPerCorner::all(12.0);
    div().style(s)
}

pub fn spacer() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_grow = 1.0;
    div().style(s)
}

pub fn divider() -> PixelNode {
    let mut s = Style::default();
    s.structural.height = px(1.0);
    s.paint.background = Color::rgb(220, 220, 220);
    div().style(s)
}

pub fn fab() -> PixelNode {
    let mut s = Style::default();
    s.structural.width = px(56.0);
    s.structural.height = px(56.0);
    s.paint.border_radius = BorderRadiusPerCorner::all(28.0);
    s.paint.background = Color::hex("#3366ff").unwrap();
    div().style(s)
}

pub fn tabs() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Row;
    div().style(s)
}

pub fn tab(text: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.structural.padding = Sides::symmetric(px(8.0), px(16.0));
    span(text).style(s)
}

pub fn tab_panel() -> PixelNode {
    div()
}

pub fn modal() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    s.structural.row_gap = px(8.0);
    s.paint.background = Color::rgb(255, 255, 255);
    s.paint.border_radius = BorderRadiusPerCorner::all(12.0);
    s.structural.padding = Sides::all(px(24.0));
    div().style(s)
}

pub fn accordion() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    div().style(s)
}

pub fn accordion_item(text: impl Into<Cow<'static, str>>) -> PixelNode {
    div().child(span(text))
}

pub fn tree() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    div().style(s)
}

pub fn tree_item(text: impl Into<Cow<'static, str>>) -> PixelNode {
    div().child(span(text))
}

pub fn dropdown() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    s.paint.background = Color::rgb(255, 255, 255);
    s.paint.box_shadow.push(BoxShadow {
        offset_x: 0.0, offset_y: 4.0, blur: 12.0, spread: 0.0,
        color: Color::rgba(0, 0, 0, 40), inset: false,
    });
    div().style(s)
}

pub fn menu() -> PixelNode {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Column;
    div().style(s)
}

pub fn menu_item(text: impl Into<Cow<'static, str>>) -> PixelNode {
    let mut s = Style::default();
    s.structural.padding = Sides::symmetric(px(8.0), px(16.0));
    span(text).style(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_is_just_a_styled_div() {
        let c = card();
        assert_eq!(c.tag, "div");
        assert_eq!(c.style.paint.border_radius, BorderRadiusPerCorner::all(12.0));
    }

    #[test]
    fn card_stacks_children_vertically_not_side_by_side() {
        assert_eq!(card().style.structural.flex_direction, FlexDirection::Column);
    }

    #[test]
    fn badge_carries_its_text() {
        let b = badge("New");
        assert_eq!(b.text.as_deref(), Some("New"));
    }
}
