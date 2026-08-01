use crate::color::Color;
use crate::units::{px, LengthValue, Sides};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Display {
    Flex,
    Grid,
    Block,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    Bold,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
    LineThrough,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
    Grab,
    NotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub inset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotate_deg: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate_deg: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRadiusPerCorner {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadiusPerCorner {
    pub fn all(v: f32) -> Self {
        BorderRadiusPerCorner {
            top_left: v,
            top_right: v,
            bottom_right: v,
            bottom_left: v,
        }
    }
}

impl Default for BorderRadiusPerCorner {
    fn default() -> Self {
        BorderRadiusPerCorner::all(0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuralStyle {
    pub display: Display,
    pub position: Position,
    pub top: LengthValue,
    pub right: LengthValue,
    pub bottom: LengthValue,
    pub left: LengthValue,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: LengthValue,
    pub grid_template_columns: Vec<LengthValue>,
    pub grid_template_rows: Vec<LengthValue>,
    pub grid_column_start: Option<u16>,
    pub grid_column_end: Option<u16>,
    pub grid_row_start: Option<u16>,
    pub grid_row_end: Option<u16>,
    pub width: LengthValue,
    pub height: LengthValue,
    pub min_width: LengthValue,
    pub min_height: LengthValue,
    pub max_width: LengthValue,
    pub max_height: LengthValue,
    pub padding: Sides,
    pub margin: Sides,
    pub border_width: Sides,
    pub row_gap: LengthValue,
    pub column_gap: LengthValue,
    pub overflow: Overflow,
    pub box_sizing: BoxSizing,
    pub z_index: i32,
    pub font_size: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
}

impl Default for StructuralStyle {
    fn default() -> Self {
        StructuralStyle {
            display: Display::Flex,
            position: Position::Relative,
            top: LengthValue::Auto,
            right: LengthValue::Auto,
            bottom: LengthValue::Auto,
            left: LengthValue::Auto,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: LengthValue::Auto,
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_column_start: None,
            grid_column_end: None,
            grid_row_start: None,
            grid_row_end: None,
            width: LengthValue::Auto,
            height: LengthValue::Auto,
            min_width: LengthValue::Auto,
            min_height: LengthValue::Auto,
            max_width: LengthValue::Auto,
            max_height: LengthValue::Auto,
            padding: Sides::zero(),
            margin: Sides::zero(),
            border_width: Sides::zero(),
            row_gap: px(0.0),
            column_gap: px(0.0),
            overflow: Overflow::Visible,
            box_sizing: BoxSizing::BorderBox,
            z_index: 0,
            font_size: 16.0,
            line_height: 1.4,
            letter_spacing: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaintStyle {
    pub background: Color,
    pub border_color: Color,
    pub border_radius: BorderRadiusPerCorner,
    pub box_shadow: Vec<BoxShadow>,
    pub opacity: f32,
    pub transform: Transform,
    pub color: Color,
    pub font_family: Cow<'static, str>,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub text_align: TextAlign,
    pub text_decoration: TextDecoration,
    pub cursor: Cursor,
}

impl Default for PaintStyle {
    fn default() -> Self {
        PaintStyle {
            background: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            border_radius: BorderRadiusPerCorner::default(),
            box_shadow: Vec::new(),
            opacity: 1.0,
            transform: Transform::default(),
            color: Color::rgb(0, 0, 0),
            font_family: Cow::Borrowed("system-ui"),
            font_weight: FontWeight::Regular,
            font_style: FontStyle::Normal,
            text_align: TextAlign::Left,
            text_decoration: TextDecoration::None,
            cursor: Cursor::Default,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    pub structural: StructuralStyle,
    pub paint: PaintStyle,
}
