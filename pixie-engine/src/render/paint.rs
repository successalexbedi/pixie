use pixie_node::PixelNode;
use pixie_style::Color;
use taffy::prelude::NodeId;
use cosmic_text::{FontSystem, SwashCache};
use crate::layout::TreeBridge;
use crate::text::{GlyphAtlas, shape_line};
use super::quad::{QuadBatch, PixelRect};
use super::text_pass::push_glyph;

fn color_to_f32(c: Color) -> [f32; 4] {
    [c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0]
}

#[allow(clippy::too_many_arguments)]
pub fn paint_tree(
    node: &PixelNode,
    bridge: &TreeBridge,
    node_id: NodeId,
    offset_x: f32,
    offset_y: f32,
    batch: &mut QuadBatch,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    atlas: &mut GlyphAtlas,
) {
    let layout = bridge.layout(node_id);
    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;

    if node.style.paint.background.a > 0 {
        batch.push_rect(
            PixelRect { x: abs_x, y: abs_y, width: layout.size.width, height: layout.size.height },
            color_to_f32(node.style.paint.background),
        );
    }

    if let Some(text) = &node.text {
        let line_height_px = node.style.structural.font_size * node.style.structural.line_height;
        let glyphs = shape_line(font_system, text, node.style.structural.font_size, line_height_px);
        let color = color_to_f32(node.style.paint.color);

        for glyph in glyphs {
            if let Some(rect) = atlas.get_or_insert(font_system, swash_cache, glyph.cache_key) {
                push_glyph(batch, abs_x + glyph.x, abs_y + glyph.y, rect, atlas.width, atlas.height, color);
            }
        }
    }

    let taffy_children = bridge.tree.children(node_id).unwrap_or_default();
    for (child_node, &child_id) in node.children.iter().zip(taffy_children.iter()) {
        paint_tree(child_node, bridge, child_id, abs_x, abs_y, batch, font_system, swash_cache, atlas);
    }
}
