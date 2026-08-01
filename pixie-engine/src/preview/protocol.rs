use serde::Serialize;
use taffy::prelude::*;
use pixie_node::PixelNode;
use pixie_style::Color;
use crate::layout::TreeBridge;

/// A flattened snapshot of one node's REAL computed position (from
/// taffy) and paint data — not the browser re-deriving layout from
/// CSS, but pixie's own layout math, just drawn as absolutely
/// positioned divs. What you see in the browser is what the actual
/// engine computed.
#[derive(Debug, Clone, Serialize)]
pub struct BoxSnapshot {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background: [u8; 4],
    pub text: Option<String>,
    pub font_size: f32,
    pub color: [u8; 4],
    pub children: Vec<BoxSnapshot>,
}

fn color_to_rgba(c: Color) -> [u8; 4] {
    [c.r, c.g, c.b, c.a]
}

/// Walks a PixelNode tree alongside its already-computed taffy
/// layout, accumulating absolute (not parent-relative) positions —
/// taffy gives each box's location relative to its own parent, so
/// this threads the running offset down as it recurses.
pub fn snapshot_tree(node: &PixelNode, bridge: &TreeBridge, node_id: NodeId, offset_x: f32, offset_y: f32) -> BoxSnapshot {
    let layout = bridge.layout(node_id);
    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;

    let taffy_children = bridge.tree.children(node_id).unwrap_or_default();
    let children = node.children.iter().zip(taffy_children.iter())
        .map(|(child_node, &child_id)| snapshot_tree(child_node, bridge, child_id, abs_x, abs_y))
        .collect();

    BoxSnapshot {
        x: abs_x,
        y: abs_y,
        width: layout.size.width,
        height: layout.size.height,
        background: color_to_rgba(node.style.paint.background),
        text: node.text.as_ref().map(|t| t.to_string()),
        font_size: node.style.structural.font_size,
        color: color_to_rgba(node.style.paint.color),
        children,
    }
}
