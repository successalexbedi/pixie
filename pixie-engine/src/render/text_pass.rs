use crate::text::AtlasRect;
use super::quad::{QuadBatch, PixelRect};

pub fn push_glyph(
    batch: &mut QuadBatch,
    screen_x: f32,
    screen_y: f32,
    atlas_rect: AtlasRect,
    atlas_width: u32,
    atlas_height: u32,
    color: [f32; 4],
) {
    if atlas_rect.width == 0 || atlas_rect.height == 0 {
        return;
    }
    let rect = PixelRect {
        x: screen_x, y: screen_y,
        width: atlas_rect.width as f32, height: atlas_rect.height as f32,
    };
    let uv_min = [atlas_rect.x as f32 / atlas_width as f32, atlas_rect.y as f32 / atlas_height as f32];
    let uv_max = [
        (atlas_rect.x + atlas_rect.width) as f32 / atlas_width as f32,
        (atlas_rect.y + atlas_rect.height) as f32 / atlas_height as f32,
    ];
    batch.push_textured_rect(rect, uv_min, uv_max, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_glyph_pushes_nothing() {
        let mut batch = QuadBatch::new();
        push_glyph(&mut batch, 0.0, 0.0, AtlasRect { x: 0, y: 0, width: 0, height: 0 }, 256, 256, [0.0; 4]);
        assert_eq!(batch.vertices.len(), 0);
    }

    #[test]
    fn real_glyph_computes_correct_uv_rect() {
        let mut batch = QuadBatch::new();
        push_glyph(&mut batch, 5.0, 5.0, AtlasRect { x: 64, y: 0, width: 64, height: 64 }, 256, 256, [1.0; 4]);
        assert_eq!(batch.vertices.len(), 4);
        assert!((batch.vertices[0].uv[0] - 0.25).abs() < 0.0001);
    }
}
