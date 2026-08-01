pub fn snap_to_physical(logical: f32, scale_factor: f32) -> f32 {
    (logical * scale_factor).round() / scale_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snaps_cleanly_at_150_percent_scale() {
        let snapped = snap_to_physical(10.3333, 1.5);
        let physical = snapped * 1.5;
        assert!((physical - physical.round()).abs() < 0.001);
    }
}
