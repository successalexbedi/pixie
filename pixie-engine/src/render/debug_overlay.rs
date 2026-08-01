use super::quad::{QuadBatch, PixelRect};

#[derive(Default)]
pub struct DebugOverlay {
    pub enabled: bool,
}

impl DebugOverlay {
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn push_box(&self, batch: &mut QuadBatch, rect: PixelRect) {
        if !self.enabled { return; }
        batch.push_rect(rect, [1.0, 0.0, 1.0, 0.15]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_overlay_pushes_nothing() {
        let overlay = DebugOverlay::default();
        let mut batch = QuadBatch::new();
        overlay.push_box(&mut batch, PixelRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        assert_eq!(batch.vertices.len(), 0);
    }

    #[test]
    fn enabled_overlay_pushes_a_quad() {
        let mut overlay = DebugOverlay::default();
        overlay.toggle();
        let mut batch = QuadBatch::new();
        overlay.push_box(&mut batch, PixelRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        assert_eq!(batch.vertices.len(), 4);
    }
}
