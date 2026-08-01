#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn hex(input: &str) -> Result<Color, String> {
        let s = input.trim_start_matches('#');
        if s.len() != 6 && s.len() != 8 {
            return Err(format!("expected 6 or 8 hex chars, got {}", s.len()));
        }
        if !s.is_ascii() {
            return Err("expected valid ASCII hex characters".to_string());
        }
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
        let a = if s.len() == 8 {
            u8::from_str_radix(&s[6..8], 16).map_err(|e| e.to_string())?
        } else {
            255
        };
        Ok(Color { r, g, b, a })
    }
}
