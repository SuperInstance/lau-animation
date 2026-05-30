use serde::{Deserialize, Serialize};

/// RGBA color with components in the `[0, 1]` range.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

    /// Convert HSL to RGB. `h` is in turns (0-1), `s` and `l` in `[0,1]`.
    /// Returns an opaque Color (alpha = 1.0).
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let (r, g, b) = if h < 1.0 / 6.0 {
            (c, x, 0.0)
        } else if h < 2.0 / 6.0 {
            (x, c, 0.0)
        } else if h < 3.0 / 6.0 {
            (0.0, c, x)
        } else if h < 4.0 / 6.0 {
            (0.0, x, c)
        } else if h < 5.0 / 6.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        Color::new(r + m, g + m, b + m, 1.0)
    }
}

impl crate::lerp::Lerp for Color {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Color {
            r: self.r.lerp(&other.r, t),
            g: self.g.lerp(&other.g, t),
            b: self.b.lerp(&other.b, t),
            a: self.a.lerp(&other.a, t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lerp::Lerp;

    #[test]
    fn color_lerp_midpoint() {
        let a = Color::new(0.0, 0.0, 0.0, 0.0);
        let b = Color::new(1.0, 1.0, 1.0, 1.0);
        let r = a.lerp(&b, 0.5);
        assert!((r.r - 0.5).abs() < 1e-12);
        assert!((r.g - 0.5).abs() < 1e-12);
        assert!((r.b - 0.5).abs() < 1e-12);
        assert!((r.a - 0.5).abs() < 1e-12);
    }

    #[test]
    fn color_lerp_identity() {
        let c = Color::new(0.2, 0.4, 0.6, 0.8);
        let r = c.lerp(&c, 0.5);
        assert_eq!(r, c);
    }

    #[test]
    fn color_from_hsl_primary() {
        // Red at h=0
        let c = Color::from_hsl(0.0, 1.0, 0.5);
        assert!((c.r - 1.0).abs() < 1e-6);
        assert!((c.g - 0.0).abs() < 1e-6);
        assert!((c.b - 0.0).abs() < 1e-6);
        assert!((c.a - 1.0).abs() < 1e-12);
    }

    #[test]
    fn color_from_hsl_green() {
        let c = Color::from_hsl(1.0 / 3.0, 1.0, 0.5);
        assert!((c.r - 0.0).abs() < 1e-6);
        assert!((c.g - 1.0).abs() < 1e-6);
        assert!((c.b - 0.0).abs() < 1e-6);
    }

    #[test]
    fn color_from_hsl_blue() {
        let c = Color::from_hsl(2.0 / 3.0, 1.0, 0.5);
        assert!((c.r - 0.0).abs() < 1e-6);
        assert!((c.g - 0.0).abs() < 1e-6);
        assert!((c.b - 1.0).abs() < 1e-6);
    }

    #[test]
    fn color_constants() {
        assert_eq!(Color::BLACK, Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::WHITE, Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::TRANSPARENT, Color::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn color_from_hsl_greys() {
        let c = Color::from_hsl(0.3, 0.0, 0.5);
        assert!((c.r - 0.5).abs() < 1e-6);
        assert!((c.g - 0.5).abs() < 1e-6);
        assert!((c.b - 0.5).abs() < 1e-6);
    }
}
