use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct QuadBatch {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl QuadBatch {
    pub fn new() -> Self { Self::default() }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn push_rect(&mut self, rect: PixelRect, color: [f32; 4]) {
        let base = self.vertices.len() as u32;
        let (x, y, w, h) = (rect.x, rect.y, rect.width, rect.height);
        self.vertices.push(Vertex { position: [x, y], uv: [-1.0, -1.0], color });
        self.vertices.push(Vertex { position: [x + w, y], uv: [-1.0, -1.0], color });
        self.vertices.push(Vertex { position: [x + w, y + h], uv: [-1.0, -1.0], color });
        self.vertices.push(Vertex { position: [x, y + h], uv: [-1.0, -1.0], color });
        self.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    pub fn push_textured_rect(&mut self, rect: PixelRect, uv_min: [f32; 2], uv_max: [f32; 2], color: [f32; 4]) {
        let base = self.vertices.len() as u32;
        let (x, y, w, h) = (rect.x, rect.y, rect.width, rect.height);
        self.vertices.push(Vertex { position: [x, y], uv: [uv_min[0], uv_min[1]], color });
        self.vertices.push(Vertex { position: [x + w, y], uv: [uv_max[0], uv_min[1]], color });
        self.vertices.push(Vertex { position: [x + w, y + h], uv: [uv_max[0], uv_max[1]], color });
        self.vertices.push(Vertex { position: [x, y + h], uv: [uv_min[0], uv_max[1]], color });
        self.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_rect_produces_four_verts_six_indices() {
        let mut batch = QuadBatch::new();
        batch.push_rect(PixelRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 }, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(batch.vertices.len(), 4);
        assert_eq!(batch.indices.len(), 6);
    }

    #[test]
    fn clear_drops_contents_but_keeps_capacity() {
        let mut batch = QuadBatch::new();
        for _ in 0..50 {
            batch.push_rect(PixelRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, [0.0, 0.0, 0.0, 1.0]);
        }
        let cap_before = batch.vertices.capacity();
        batch.clear();
        assert_eq!(batch.vertices.len(), 0);
        assert_eq!(batch.vertices.capacity(), cap_before);
    }

    #[test]
    fn second_rect_indices_offset_correctly() {
        let mut batch = QuadBatch::new();
        batch.push_rect(PixelRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, [0.0, 0.0, 0.0, 1.0]);
        batch.push_rect(PixelRect { x: 1.0, y: 0.0, width: 1.0, height: 1.0 }, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(&batch.indices[6..12], &[4, 5, 6, 4, 6, 7]);
    }
}
