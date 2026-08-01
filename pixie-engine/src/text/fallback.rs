use cosmic_text::{Attrs, Family, FontSystem};

pub fn attrs_for_family(family_name: &'static str) -> Attrs<'static> {
    Attrs::new().family(Family::Name(family_name))
}

pub fn list_available_family_names(font_system: &mut FontSystem) -> Vec<String> {
    font_system
        .db()
        .faces()
        .filter_map(|face| face.families.first().map(|(name, _)| name.clone()))
        .collect()
}
