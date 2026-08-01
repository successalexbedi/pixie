use pixie_style::{Ease, Transition};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct ActiveTween {
    pub start: f32,
    pub target: f32,
    pub elapsed_ms: f32,
    pub duration_ms: f32,
    pub ease: Ease,
}

impl ActiveTween {
    pub fn new(start: f32, target: f32, transition: Transition) -> Self {
        ActiveTween { start, target, elapsed_ms: 0.0, duration_ms: transition.duration_ms as f32, ease: transition.ease }
    }

    fn eased_fraction(&self) -> f32 {
        let frac = if self.duration_ms <= 0.0 { 1.0 } else { (self.elapsed_ms / self.duration_ms).min(1.0) };
        match self.ease {
            Ease::Linear => frac,
            Ease::EaseInOut => frac * frac * (3.0 - 2.0 * frac),
        }
    }

    pub fn step(&mut self, dt_ms: f32) -> f32 {
        self.elapsed_ms = (self.elapsed_ms + dt_ms).min(self.duration_ms);
        self.start + (self.target - self.start) * self.eased_fraction()
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed_ms >= self.duration_ms
    }
}

#[derive(Default)]
pub struct TweenRuntime {
    active: HashMap<u64, ActiveTween>,
}

impl TweenRuntime {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self, node_key: u64, start: f32, target: f32, transition: Transition) {
        self.active.insert(node_key, ActiveTween::new(start, target, transition));
    }

    pub fn advance_all(&mut self, dt_ms: f32) {
        self.active.retain(|_, tween| {
            tween.step(dt_ms);
            !tween.is_finished()
        });
    }

    pub fn current(&self, node_key: u64) -> Option<f32> {
        self.active.get(&node_key).map(|t| t.start + (t.target - t.start) * t.eased_fraction())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixie_style::Property;

    #[test]
    fn tween_reaches_target_at_full_duration() {
        let transition = Transition { property: Property::Opacity, duration_ms: 200, ease: Ease::Linear };
        let mut tween = ActiveTween::new(0.0, 1.0, transition);
        let v = tween.step(200.0);
        assert!((v - 1.0).abs() < 0.0001);
        assert!(tween.is_finished());
    }

    #[test]
    fn tween_is_halfway_at_half_duration_linear() {
        let transition = Transition { property: Property::Opacity, duration_ms: 200, ease: Ease::Linear };
        let mut tween = ActiveTween::new(0.0, 10.0, transition);
        let v = tween.step(100.0);
        assert!((v - 5.0).abs() < 0.0001);
    }

    #[test]
    fn runtime_drops_finished_tweens_after_advance() {
        let mut runtime = TweenRuntime::new();
        let transition = Transition { property: Property::Opacity, duration_ms: 100, ease: Ease::Linear };
        runtime.start(1, 0.0, 1.0, transition);
        runtime.advance_all(200.0);
        assert!(runtime.current(1).is_none());
    }
}
