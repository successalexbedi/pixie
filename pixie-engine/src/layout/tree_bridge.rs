use taffy::prelude::*;
use pixie_style::{
    Style as PixelStyle, StructuralStyle, LengthValue,
    Display as PixelDisplay, Position as PixelPosition,
    FlexDirection as PixelFlexDirection, FlexWrap as PixelFlexWrap,
    JustifyContent as PixelJustifyContent, AlignItems as PixelAlignItems,
    Overflow as PixelOverflow, BoxSizing as PixelBoxSizing,
};
use cosmic_text::FontSystem;
use crate::text::shaper::measure_line;

pub trait LayoutNode {
    fn style(&self) -> &PixelStyle;
    fn children(&self) -> &[Self] where Self: Sized;
    /// Default None — only nodes that actually carry text (like
    /// PixelNode) need to override this for their size to be
    /// measurable at all.
    fn text(&self) -> Option<&str> { None }
}

fn to_dimension(v: LengthValue) -> Dimension {
    match v {
        LengthValue::Px(px) => length(px),
        LengthValue::Pct(p) => percent(p / 100.0),
        LengthValue::Fr(_) => auto(),
        LengthValue::Auto => auto(),
    }
}

fn to_length_percentage(v: LengthValue) -> LengthPercentage {
    match v {
        LengthValue::Px(px) => length(px),
        LengthValue::Pct(p) => percent(p / 100.0),
        LengthValue::Fr(_) | LengthValue::Auto => length(0.0),
    }
}

fn to_length_percentage_auto(v: LengthValue) -> LengthPercentageAuto {
    match v {
        LengthValue::Px(px) => length(px),
        LengthValue::Pct(p) => percent(p / 100.0),
        LengthValue::Fr(_) => auto(),
        LengthValue::Auto => auto(),
    }
}

fn to_display(d: PixelDisplay) -> taffy::style::Display {
    match d {
        PixelDisplay::Flex => taffy::style::Display::Flex,
        PixelDisplay::Grid => taffy::style::Display::Grid,
        PixelDisplay::Block => taffy::style::Display::Block,
        PixelDisplay::None => taffy::style::Display::None,
    }
}

fn to_position(p: PixelPosition) -> taffy::style::Position {
    match p {
        PixelPosition::Relative => taffy::style::Position::Relative,
        PixelPosition::Absolute | PixelPosition::Fixed => taffy::style::Position::Absolute,
    }
}

fn to_flex_direction(f: PixelFlexDirection) -> taffy::style::FlexDirection {
    match f {
        PixelFlexDirection::Row => taffy::style::FlexDirection::Row,
        PixelFlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
        PixelFlexDirection::Column => taffy::style::FlexDirection::Column,
        PixelFlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
    }
}

fn to_flex_wrap(f: PixelFlexWrap) -> taffy::style::FlexWrap {
    match f {
        PixelFlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
        PixelFlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
        PixelFlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
    }
}

fn to_justify_content(j: PixelJustifyContent) -> taffy::style::JustifyContent {
    match j {
        PixelJustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
        PixelJustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
        PixelJustifyContent::Center => taffy::style::JustifyContent::Center,
        PixelJustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
        PixelJustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
        PixelJustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
    }
}

fn to_align_items(a: PixelAlignItems) -> taffy::style::AlignItems {
    match a {
        PixelAlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
        PixelAlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
        PixelAlignItems::Center => taffy::style::AlignItems::Center,
        PixelAlignItems::Stretch => taffy::style::AlignItems::Stretch,
        PixelAlignItems::Baseline => taffy::style::AlignItems::Baseline,
    }
}

fn to_overflow(o: PixelOverflow) -> taffy::style::Overflow {
    match o {
        PixelOverflow::Visible => taffy::style::Overflow::Visible,
        PixelOverflow::Hidden => taffy::style::Overflow::Hidden,
        PixelOverflow::Scroll => taffy::style::Overflow::Scroll,
    }
}

fn to_box_sizing(b: PixelBoxSizing) -> taffy::style::BoxSizing {
    match b {
        PixelBoxSizing::ContentBox => taffy::style::BoxSizing::ContentBox,
        PixelBoxSizing::BorderBox => taffy::style::BoxSizing::BorderBox,
    }
}

pub fn to_taffy_style(s: &StructuralStyle) -> taffy::style::Style {
    let overflow_xy = to_overflow(s.overflow);

    taffy::style::Style {
        display: to_display(s.display),
        position: to_position(s.position),
        overflow: taffy::geometry::Point { x: overflow_xy, y: overflow_xy },
        box_sizing: to_box_sizing(s.box_sizing),
        inset: taffy::geometry::Rect {
            top: to_length_percentage_auto(s.top),
            right: to_length_percentage_auto(s.right),
            bottom: to_length_percentage_auto(s.bottom),
            left: to_length_percentage_auto(s.left),
        },
        size: taffy::geometry::Size { width: to_dimension(s.width), height: to_dimension(s.height) },
        min_size: taffy::geometry::Size { width: to_dimension(s.min_width), height: to_dimension(s.min_height) },
        max_size: taffy::geometry::Size { width: to_dimension(s.max_width), height: to_dimension(s.max_height) },
        margin: taffy::geometry::Rect {
            top: to_length_percentage_auto(s.margin.top),
            right: to_length_percentage_auto(s.margin.right),
            bottom: to_length_percentage_auto(s.margin.bottom),
            left: to_length_percentage_auto(s.margin.left),
        },
        padding: taffy::geometry::Rect {
            top: to_length_percentage(s.padding.top),
            right: to_length_percentage(s.padding.right),
            bottom: to_length_percentage(s.padding.bottom),
            left: to_length_percentage(s.padding.left),
        },
        border: taffy::geometry::Rect {
            top: to_length_percentage(s.border_width.top),
            right: to_length_percentage(s.border_width.right),
            bottom: to_length_percentage(s.border_width.bottom),
            left: to_length_percentage(s.border_width.left),
        },
        gap: taffy::geometry::Size {
            width: to_length_percentage(s.column_gap),
            height: to_length_percentage(s.row_gap),
        },
        flex_direction: to_flex_direction(s.flex_direction),
        flex_wrap: to_flex_wrap(s.flex_wrap),
        flex_grow: s.flex_grow,
        flex_shrink: s.flex_shrink,
        flex_basis: to_dimension(s.flex_basis),
        align_items: Some(to_align_items(s.align_items)),
        justify_content: Some(to_justify_content(s.justify_content)),
        ..Default::default()
    }
}

pub struct TreeBridge {
    pub tree: TaffyTree<()>,
}

impl TreeBridge {
    pub fn new() -> Self {
        TreeBridge { tree: TaffyTree::new() }
    }

    /// font_system is required now — text leaves get measured before
    /// their taffy style is built, so a font shaper has to be
    /// available at build time, not just at paint time.
    pub fn build<N: LayoutNode>(&mut self, node: &N, font_system: &mut FontSystem) -> NodeId {
        let mut structural = node.style().structural.clone();

        if let Some(text) = node.text() {
            if structural.width == LengthValue::Auto || structural.height == LengthValue::Auto {
                let line_height_px = structural.font_size * structural.line_height;
                let (w, h) = measure_line(font_system, text, structural.font_size, line_height_px);
                if structural.width == LengthValue::Auto {
                    structural.width = LengthValue::Px(w);
                }
                if structural.height == LengthValue::Auto {
                    structural.height = LengthValue::Px(h);
                }
            }
        }

        let taffy_style = to_taffy_style(&structural);
        let kids = node.children();
        let mut child_ids: Vec<NodeId> = Vec::with_capacity(kids.len());
        for c in kids {
            child_ids.push(self.build(c, font_system));
        }
        if child_ids.is_empty() {
            self.tree.new_leaf(taffy_style).expect("failed to create taffy leaf")
        } else {
            self.tree.new_with_children(taffy_style, &child_ids).expect("failed to create taffy node")
        }
    }

    pub fn compute(&mut self, root: NodeId, available: Size<AvailableSpace>) {
        self.tree.compute_layout(root, available).expect("layout computation failed");
    }

    pub fn layout(&self, node: NodeId) -> &Layout {
        self.tree.layout(node).expect("no layout computed for node")
    }
}

impl Default for TreeBridge {
    fn default() -> Self { Self::new() }
}

impl LayoutNode for pixie_node::PixelNode {
    fn style(&self) -> &PixelStyle { &self.style }
    fn children(&self) -> &[Self] { &self.children }
    fn text(&self) -> Option<&str> { self.text.as_deref() }
}
