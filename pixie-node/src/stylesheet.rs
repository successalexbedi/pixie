use crate::node::PixelNode;
use pixie_style::{Style, Color, BorderRadiusPerCorner, Sides, px};

#[derive(Debug, Clone, Default)]
pub struct RuleBuilder {
    target: Option<&'static str>,
    background: Option<Color>,
    color: Option<Color>,
    font_size: Option<f32>,
    radius: Option<f32>,
    padding: Option<f32>,
}

impl RuleBuilder {
    pub fn background(mut self, color: Color) -> Self { self.background = Some(color); self }
    pub fn color(mut self, color: Color) -> Self { self.color = Some(color); self }
    pub fn font_size(mut self, size: f32) -> Self { self.font_size = Some(size); self }
    pub fn radius(mut self, r: f32) -> Self { self.radius = Some(r); self }
    /// Uniform padding on all four sides — per-side padding via
    /// stylesheet rules is real future work, .style() already
    /// supports it directly in the meantime.
    pub fn padding(mut self, p: f32) -> Self { self.padding = Some(p); self }
}

#[derive(Debug, Clone)]
pub struct Rule {
    target: Option<&'static str>,
    background: Option<Color>,
    color: Option<Color>,
    font_size: Option<f32>,
    radius: Option<f32>,
    padding: Option<f32>,
}

impl From<RuleBuilder> for Rule {
    fn from(rb: RuleBuilder) -> Rule {
        Rule {
            target: rb.target,
            background: rb.background,
            color: rb.color,
            font_size: rb.font_size,
            radius: rb.radius,
            padding: rb.padding,
        }
    }
}

fn apply_rule(style: &mut Style, rule: &Rule) {
    if let Some(c) = rule.background { style.paint.background = c; }
    if let Some(c) = rule.color { style.paint.color = c; }
    if let Some(fs) = rule.font_size { style.structural.font_size = fs; }
    if let Some(r) = rule.radius { style.paint.border_radius = BorderRadiusPerCorner::all(r); }
    if let Some(p) = rule.padding { style.structural.padding = Sides::all(px(p)); }
}

pub fn background(color: Color) -> RuleBuilder {
    RuleBuilder::default().background(color)
}

pub fn target(id: &'static str) -> RuleBuilder {
    RuleBuilder { target: Some(id), ..Default::default() }
}

#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

/// Extended up to 12 rules — 6 was fine for a proof-of-concept, too
/// small for a real screen with a header, hero, several cards, and
/// actions all themed at once.
macro_rules! impl_tuple_into_stylesheet {
    ($($name:ident)+) => {
        impl<$($name),+> From<($($name,)+)> for StyleSheet
        where $($name: Into<Rule>),+
        {
            #[allow(non_snake_case)]
            fn from(tuple: ($($name,)+)) -> StyleSheet {
                let ($($name,)+) = tuple;
                StyleSheet { rules: vec![$($name.into()),+] }
            }
        }
    };
}

impl_tuple_into_stylesheet!(A);
impl_tuple_into_stylesheet!(A B);
impl_tuple_into_stylesheet!(A B C);
impl_tuple_into_stylesheet!(A B C D);
impl_tuple_into_stylesheet!(A B C D E);
impl_tuple_into_stylesheet!(A B C D E F);
impl_tuple_into_stylesheet!(A B C D E F G);
impl_tuple_into_stylesheet!(A B C D E F G H);
impl_tuple_into_stylesheet!(A B C D E F G H I);
impl_tuple_into_stylesheet!(A B C D E F G H I J);
impl_tuple_into_stylesheet!(A B C D E F G H I J K);
impl_tuple_into_stylesheet!(A B C D E F G H I J K L);

impl PixelNode {
    pub fn stylesheet(mut self, sheet: impl Into<StyleSheet>) -> Self {
        let sheet = sheet.into();
        resolve_into(&mut self, &sheet);
        self
    }
}

fn resolve_into(node: &mut PixelNode, sheet: &StyleSheet) {
    for rule in &sheet.rules {
        match rule.target {
            None => apply_rule(&mut node.style, rule),
            Some(id) if node.id.as_deref() == Some(id) => apply_rule(&mut node.style, rule),
            Some(_) => {}
        }
    }
    for child in &mut node.children {
        resolve_into(child, sheet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{div, h1, button, p};

    #[test]
    fn global_rule_reaches_every_node() {
        let tree = div()
            .child(h1("Welcome").ref_id("title"))
            .child(p("Body").ref_id("description"))
            .stylesheet((background(Color::rgb(255, 255, 255)),));
        assert_eq!(tree.style.paint.background, Color::rgb(255, 255, 255));
        assert_eq!(tree.children[0].style.paint.background, Color::rgb(255, 255, 255));
    }

    #[test]
    fn target_rule_only_touches_the_matching_node() {
        let tree = div()
            .child(h1("Welcome").ref_id("title"))
            .child(p("Body").ref_id("description"))
            .stylesheet((target("title").font_size(32.0).color(Color::rgb(0, 0, 255)),));
        assert_eq!(tree.children[0].style.structural.font_size, 32.0);
        assert_eq!(tree.children[1].style.structural.font_size, 16.0);
    }

    #[test]
    fn twelve_rule_sheet_compiles_and_resolves() {
        let tree = div()
            .child(button("Save").ref_id("save"))
            .stylesheet((
                background(Color::rgb(1, 1, 1)),
                target("a"), target("b"), target("c"), target("d"),
                target("e"), target("f"), target("g"), target("h"),
                target("i"),
                target("save").padding(12.0).radius(8.0),
            ));
        assert_eq!(tree.children[0].style.structural.padding.top, px(12.0));
    }
}
