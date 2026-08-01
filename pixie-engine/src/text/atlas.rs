use std::collections::HashMap;
use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct GlyphAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    entries: HashMap<CacheKey, AtlasRect>,
    shelf_x: u32,
    shelf_y: u32,
    shelf_height: u32,
}

impl GlyphAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        GlyphAtlas {
            width, height,
            pixels: vec![0u8; (width * height) as usize],
            entries: HashMap::new(),
            shelf_x: 0, shelf_y: 0, shelf_height: 0,
        }
    }

    pub fn get_or_insert(
        &mut self,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        key: CacheKey,
    ) -> Option<AtlasRect> {
        if let Some(&rect) = self.entries.get(&key) {
            return Some(rect);
        }

        let image = swash_cache.get_image(font_system, key).as_ref()?;
        let w = image.placement.width;
        let h = image.placement.height;

        if w == 0 || h == 0 {
            let rect = AtlasRect { x: 0, y: 0, width: 0, height: 0 };
            self.entries.insert(key, rect);
            return Some(rect);
        }

        if self.shelf_x + w > self.width {
            self.shelf_x = 0;
            self.shelf_y += self.shelf_height;
            self.shelf_height = 0;
        }
        if self.shelf_y + h > self.height {
            return None;
        }

        let rect = AtlasRect { x: self.shelf_x, y: self.shelf_y, width: w, height: h };

        if image.content == SwashContent::Mask {
            let data = image.data.clone();
            self.blit(&data, rect);
        }

        self.shelf_x += w;
        self.shelf_height = self.shelf_height.max(h);
        self.entries.insert(key, rect);
        Some(rect)
    }

    fn blit(&mut self, src: &[u8], rect: AtlasRect) {
        for row in 0..rect.height {
            for col in 0..rect.width {
                let src_idx = (row * rect.width + col) as usize;
                let dst_x = rect.x + col;
                let dst_y = rect.y + row;
                let dst_idx = (dst_y * self.width + dst_x) as usize;
                if let (Some(&byte), true) = (src.get(src_idx), dst_idx < self.pixels.len()) {
                    self.pixels[dst_idx] = byte;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::shaper::{new_font_system, shape_line};

    #[test]
    fn shapes_a_simple_ascii_string() {
        let mut font_system = new_font_system();
        let glyphs = shape_line(&mut font_system, "Hi", 16.0, 20.0);
        assert_eq!(glyphs.len(), 2);
    }

    #[test]
    fn atlas_reuses_the_same_cache_key_on_a_second_request() {
        let mut font_system = new_font_system();
        let mut swash_cache = SwashCache::new();
        let glyphs = shape_line(&mut font_system, "A", 16.0, 20.0);
        let mut atlas = GlyphAtlas::new(256, 256);
        let rect_first = atlas.get_or_insert(&mut font_system, &mut swash_cache, glyphs[0].cache_key);
        let rect_second = atlas.get_or_insert(&mut font_system, &mut swash_cache, glyphs[0].cache_key);
        assert!(rect_first.is_some());
        assert_eq!(rect_first, rect_second);
    }

    #[test]
    fn distinct_glyphs_get_distinct_atlas_rects() {
        let mut font_system = new_font_system();
        let mut swash_cache = SwashCache::new();
        let glyphs = shape_line(&mut font_system, "AB", 16.0, 20.0);
        let mut atlas = GlyphAtlas::new(256, 256);
        let rect_a = atlas.get_or_insert(&mut font_system, &mut swash_cache, glyphs[0].cache_key);
        let rect_b = atlas.get_or_insert(&mut font_system, &mut swash_cache, glyphs[1].cache_key);
        assert!(rect_a.is_some());
        assert!(rect_b.is_some());
        assert_ne!(rect_a, rect_b);
    }
}
