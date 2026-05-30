/// Linear interpolation trait.
///
/// Implementations should produce `self + (other - self) * t` for `t` in `[0, 1]`.
pub trait Lerp: Clone {
    fn lerp(&self, other: &Self, t: f64) -> Self;
}

// ---------------------------------------------------------------------------
// f64
// ---------------------------------------------------------------------------
impl Lerp for f64 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        self + (other - self) * t
    }
}

// ---------------------------------------------------------------------------
// (f64, f64)
// ---------------------------------------------------------------------------
impl Lerp for (f64, f64) {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_f64_midpoint() {
        assert!((0.5_f64.lerp(&1.5, 0.5) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_f64_zero() {
        assert!((10.0_f64.lerp(&20.0, 0.0) - 10.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_f64_one() {
        assert!((10.0_f64.lerp(&20.0, 1.0) - 20.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_f64_clamp_extrapolate() {
        // lerp works outside [0,1] — intentional extrapolation
        let v = 0.0_f64.lerp(&10.0, 2.0);
        assert!((v - 20.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_f64_negative_t() {
        let v = 10.0_f64.lerp(&20.0, -1.0);
        assert!((v - 0.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_vec2_midpoint() {
        let a = (0.0, 0.0);
        let b = (10.0, 20.0);
        let r = a.lerp(&b, 0.5);
        assert!((r.0 - 5.0).abs() < 1e-12);
        assert!((r.1 - 10.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_vec2_zero() {
        let a = (5.0, 5.0);
        let b = (10.0, 10.0);
        let r = a.lerp(&b, 0.0);
        assert!((r.0 - 5.0).abs() < 1e-12);
        assert!((r.1 - 5.0).abs() < 1e-12);
    }

    #[test]
    fn lerp_vec2_one() {
        let a = (5.0, 5.0);
        let b = (10.0, 10.0);
        let r = a.lerp(&b, 1.0);
        assert!((r.0 - 10.0).abs() < 1e-12);
        assert!((r.1 - 10.0).abs() < 1e-12);
    }
}
