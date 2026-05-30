use serde::{Deserialize, Serialize};

/// Easing curve function — maps `t ∈ [0,1]` to an eased value `∈ [0,1]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Step,
}

impl Easing {
    /// Apply this easing function to `t` (clamped to `[0, 1]`).
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => ease_in_quad(t),
            Easing::EaseOut => ease_out_quad(t),
            Easing::EaseInOut => ease_in_out_quad(t),
            Easing::Bounce => ease_bounce(t),
            Easing::Elastic => ease_elastic(t),
            Easing::Step => {
                if t < 0.5 {
                    0.0
                } else {
                    1.0
                }
            }
        }
    }
}

fn ease_in_quad(t: f64) -> f64 {
    t * t
}

fn ease_out_quad(t: f64) -> f64 {
    t * (2.0 - t)
}

fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

fn ease_bounce(t: f64) -> f64 {
    const N1: f64 = 7.5625;
    const D1: f64 = 2.75;
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984_375
    }
}

fn ease_elastic(t: f64) -> f64 {
    const C4: f64 = std::f64::consts::PI * 2.0 / 3.0;
    if t == 0.0 || t == 1.0 {
        return t;
    }
    -f64::powf(2.0, 10.0 * t - 10.0) * f64::sin((t * 10.0 - 10.75) * C4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_identity() {
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((Easing::Linear.apply(t) - t).abs() < 1e-12);
        }
    }

    #[test]
    fn ease_in_starts_slow() {
        let v = Easing::EaseIn.apply(0.25);
        assert!(v < 0.25);
        assert!(v > 0.0);
    }

    #[test]
    fn ease_out_ends_slow() {
        let v = Easing::EaseOut.apply(0.75);
        assert!(v > 0.75);
        assert!(v < 1.0);
    }

    #[test]
    fn ease_in_out_midpoint() {
        let v = Easing::EaseInOut.apply(0.5);
        assert!((v - 0.5).abs() < 1e-12);
    }

    #[test]
    fn bounce_range() {
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let v = Easing::Bounce.apply(t);
            assert!(v >= 0.0, "bounce at {t} = {v} < 0");
            assert!(v <= 1.0, "bounce at {t} = {v} > 1");
        }
    }

    #[test]
    fn elastic_extremes() {
        // elastic can overshoot both below 0 and above 1
        let _v = Easing::Elastic.apply(0.5);
        // but should start at 0 and end at 1
        assert!((Easing::Elastic.apply(0.0) - 0.0).abs() < 1e-12);
        assert!((Easing::Elastic.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn step_discrete() {
        assert!((Easing::Step.apply(0.0) - 0.0).abs() < 1e-12);
        assert!((Easing::Step.apply(0.49) - 0.0).abs() < 1e-12);
        assert!((Easing::Step.apply(0.5) - 1.0).abs() < 1e-12);
        assert!((Easing::Step.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn all_easings_at_zero() {
        for e in &[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::Bounce,
            Easing::Elastic,
            Easing::Step,
        ] {
            assert!((e.apply(0.0) - 0.0).abs() < 1e-12, "{:?} not 0 at 0", e);
        }
    }

    #[test]
    fn all_easings_at_one() {
        for e in &[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::Bounce,
            Easing::Elastic,
            Easing::Step,
        ] {
            assert!((e.apply(1.0) - 1.0).abs() < 1e-12, "{:?} not 1 at 1", e);
        }
    }

    #[test]
    fn clamp_below_zero() {
        assert!((Easing::EaseIn.apply(-0.5) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn clamp_above_one() {
        assert!((Easing::EaseOut.apply(1.5) - 1.0).abs() < 1e-12);
    }
}
