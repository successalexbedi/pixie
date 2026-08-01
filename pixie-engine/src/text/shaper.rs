use cosmic_text::{Attrs, Buffer, CacheKey, FontSystem, Metrics, Shaping};

#[derive(Debug, Clone, Copy)]
pub struct PositionedGlyph {
    pub cache_key: CacheKey,
    pub x: f32,
    pub y: f32,
}

pub fn new_font_system() -> FontSystem {
    let mut font_system = FontSystem::new();
    if font_system.db().faces().next().is_none() {
        font_system.db_mut().load_fonts_dir("/system/fonts");
    }
    font_system
}

/// Returns (width, height) of a shaped line at this font size. Taffy
/// has no idea what a glyph is — a text leaf needs its size computed
/// HERE, before it's handed to the layout bridge as an explicit
/// width/height, or it silently collapses to 0x0.
pub fn measure_line(font_system: &mut FontSystem, text: &str, font_size: f32, line_height: f32) -> (f32, f32) {
    let metrics = Metrics::new(font_size, line_height);
    let mut buffer = Buffer::new(font_system, metrics);
    let mut buffer = buffer.borrow_with(font_system);
    buffer.set_size(Some(10_000.0), Some(line_height * 4.0));
    let attrs = Attrs::new();
    buffer.set_text(text, attrs, Shaping::Advanced);
    buffer.shape_until_scroll(true);

    let mut max_width: f32 = 0.0;
    let mut line_count = 0;
    for run in buffer.layout_runs() {
        max_width = max_width.max(run.line_w);
        line_count += 1;
    }
    let height = if line_count == 0 { line_height } else { line_height * line_count as f32 };
    (max_width, height)
}

pub fn shape_line(font_system: &mut FontSystem, text: &str, font_size: f32, line_height: f32) -> Vec<PositionedGlyph> {
    let metrics = Metrics::new(font_size, line_height);
    let mut buffer = Buffer::new(font_system, metrics);
    let mut buffer = buffer.borrow_with(font_system);

    buffer.set_size(Some(10_000.0), Some(line_height * 4.0));
    let attrs = Attrs::new();
    buffer.set_text(text, attrs, Shaping::Advanced);
    buffer.shape_until_scroll(true);

    let mut glyphs = Vec::new();
    for run in buffer.layout_runs() {
        for glyph in run.glyphs.iter() {
            let physical = glyph.physical((0.0, 0.0), 1.0);
            glyphs.push(PositionedGlyph {
                cache_key: physical.cache_key,
                x: physical.x as f32,
                y: run.line_y + physical.y as f32,
            });
        }
    }
    glyphs
}
