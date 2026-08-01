use taffy::prelude::*;
use cosmic_text::FontSystem;
use super::tree_bridge::{LayoutNode, TreeBridge, to_taffy_style};

pub struct Reconciler;

impl Reconciler {
    pub fn reconcile<N: LayoutNode>(
        bridge: &mut TreeBridge,
        existing_id: NodeId,
        new_node: &N,
        font_system: &mut FontSystem,
    ) -> NodeId {
        let new_style = to_taffy_style(&new_node.style().structural);
        let styles_differ = bridge.tree.style(existing_id).expect("node missing from tree") != &new_style;
        if styles_differ {
            bridge.tree.set_style(existing_id, new_style).expect("failed to set style");
        }

        let existing_children = bridge.tree.children(existing_id).expect("failed to get children");
        let new_children = new_node.children();
        let mut result_children: Vec<NodeId> = Vec::with_capacity(new_children.len());

        for (i, new_child) in new_children.iter().enumerate() {
            if let Some(&old_child_id) = existing_children.get(i) {
                result_children.push(Self::reconcile(bridge, old_child_id, new_child, font_system));
            } else {
                result_children.push(bridge.build(new_child, font_system));
            }
        }

        if result_children.len() != existing_children.len() {
            bridge.tree.set_children(existing_id, &result_children).expect("failed to set children");
        }

        existing_id
    }
}
