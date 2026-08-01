use pixie_style::Style;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct PixelNode {
    pub tag: &'static str,
    pub style: Style,
    pub id: Option<Cow<'static, str>>,
    pub text: Option<Cow<'static, str>>,
    pub ordered: bool,
    pub children: Vec<PixelNode>,
}

impl PixelNode {
    pub fn new(tag: &'static str) -> Self {
        PixelNode { tag, style: Style::default(), id: None, text: None, ordered: false, children: Vec::new() }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn ref_id(self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id(id)
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn ordered(mut self, ordered: bool) -> Self {
        self.ordered = ordered;
        self
    }

    /// Accepts ONE node (div().child(h1("Title"))) OR a tuple of
    /// several (div().child((h1("Title"), p("Body"), a("Link")))) —
    /// one call, no repeated .child() chaining. The double parens on
    /// the tuple form are unavoidable: Rust methods take exactly one
    /// argument, so several nodes have to arrive as a single tuple.
    pub fn child(mut self, child: impl IntoChildren) -> Self {
        self.children.extend(child.into_children());
        self
    }

    /// For dynamic lists — .children(items.iter().map(|i| ...)) —
    /// kept separate from .child() since a runtime-length iterator
    /// and a fixed-arity tuple are genuinely different shapes.
    pub fn children(mut self, children: impl IntoIterator<Item = PixelNode>) -> Self {
        self.children.extend(children);
        self
    }
}

/// Lets .child() accept either a single PixelNode or a tuple of them.
pub trait IntoChildren {
    fn into_children(self) -> Vec<PixelNode>;
}

impl IntoChildren for PixelNode {
    fn into_children(self) -> Vec<PixelNode> {
        vec![self]
    }
}

macro_rules! impl_tuple_into_children {
    ($($name:ident)+) => {
        impl<$($name),+> IntoChildren for ($($name,)+)
        where $($name: IntoChildren),+
        {
            #[allow(non_snake_case)]
            fn into_children(self) -> Vec<PixelNode> {
                let ($($name,)+) = self;
                let mut out = Vec::new();
                $(out.extend($name.into_children());)+
                out
            }
        }
    };
}

impl_tuple_into_children!(A);
impl_tuple_into_children!(A B);
impl_tuple_into_children!(A B C);
impl_tuple_into_children!(A B C D);
impl_tuple_into_children!(A B C D E);
impl_tuple_into_children!(A B C D E F);
impl_tuple_into_children!(A B C D E F G);
impl_tuple_into_children!(A B C D E F G H);
impl_tuple_into_children!(A B C D E F G H I);
impl_tuple_into_children!(A B C D E F G H I J);
impl_tuple_into_children!(A B C D E F G H I J K);
impl_tuple_into_children!(A B C D E F G H I J K L);

macro_rules! declare_container_tags {
    ($($name:ident),* $(,)?) => {
        $( pub fn $name() -> PixelNode { PixelNode::new(stringify!($name)) } )*
    };
}

macro_rules! declare_text_tags {
    ($($name:ident),* $(,)?) => {
        $( pub fn $name(text: impl Into<Cow<'static, str>>) -> PixelNode {
            let mut node = PixelNode::new(stringify!($name));
            node.text = Some(text.into());
            node
        } )*
    };
}

declare_container_tags!(
    div, section, header, footer, main_, nav, aside, article,
    input, password, email, search, textarea, checkbox, radio, switch, slider, select,
    img, icon, canvas, video, audio,
    list, item,
    table, thead, tbody, tr, th, td,
    grid, flex, scroll,
    dialog,
    hr,
);

declare_text_tags!(
    h1, h2, h3, h4, h5, h6,
    p, span, label, small, strong, em, code, pre, blockquote,
    button, a, option,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_child_still_works() {
        let tree = div().child(h1("Welcome"));
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn tuple_child_adds_all_at_once_no_repeated_calls() {
        let tree = div().child((h1("Title"), p("Body"), a("Link")));
        assert_eq!(tree.children.len(), 3);
        assert_eq!(tree.children[0].tag, "h1");
        assert_eq!(tree.children[1].tag, "p");
        assert_eq!(tree.children[2].tag, "a");
    }

    #[test]
    fn ref_id_is_readable_as_id() {
        let node = h1("Welcome").ref_id("title");
        assert_eq!(node.id.as_deref(), Some("title"));
    }
}
