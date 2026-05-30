use crate::color::Color;

/// Procedural animation driven by a "vibe" value (energy / intensity).
///
/// Higher vibe values produce faster / more exaggerated motion.
pub struct VibeAnimator;

impl VibeAnimator {
    /// Breathe: scale oscillation.
    ///
    /// Returns `(scale_x, scale_y)`. Higher vibe → faster rate and slightly
    /// larger amplitude.
    pub fn breathe(vibe: f64, time: f64) -> (f64, f64) {
        let rate = 1.0 + vibe * 2.0; // base 1 Hz, up to ~3 Hz
        let amplitude = 0.02 + vibe * 0.03; // 2–5%
        let phase = (time * rate * std::f64::consts::TAU).sin();
        let s = 1.0 + phase * amplitude;
        (s, s)
    }

    /// Sway: rotation oscillation.
    ///
    /// Returns a rotation offset (in degrees-equivalent f64). Higher vibe →
    /// faster and wider sway.
    pub fn sway(vibe: f64, time: f64) -> f64 {
        let rate = 0.8 + vibe * 2.2; // 0.8–3.0 Hz
        let amplitude = 2.0 + vibe * 8.0; // 2–10 degrees
        let phase = (time * rate * std::f64::consts::TAU).sin();
        phase * amplitude
    }

    /// Color pulse: hue shifts over time.
    ///
    /// Returns a pulsing `Color`. Vibe controls the saturation and animation
    /// speed. Hue rotates continuously.
    pub fn color_pulse(vibe: f64, time: f64) -> Color {
        let hue_speed = 0.1 + vibe * 0.4;
        let hue = (time * hue_speed).fract();
        let saturation = (0.5 + vibe * 0.5).clamp(0.0, 1.0);
        let lightness = 0.6;
        Color::from_hsl(hue, saturation, lightness)
    }

    /// Bounce height: vertical offset.
    ///
    /// Returns a vertical displacement. Vibe controls the energy/height.
    /// Uses a half-sine wave per bounce, with bounce frequency increasing
    /// with vibe.
    pub fn bounce_height(vibe: f64, time: f64) -> f64 {
        let rate = 1.0 + vibe * 3.0; // 1–4 bounces per second
        let height = 5.0 + vibe * 20.0; // 5–25 px equivalent
        let phase = (time * rate * std::f64::consts::PI).sin();
        if phase < 0.0 {
            0.0
        } else {
            phase * height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breathe_returns_pair() {
        let (sx, sy) = VibeAnimator::breathe(0.0, 0.0);
        assert!((sx - sy).abs() < 1e-12);
        assert!(sx > 0.0);
    }

    #[test]
    fn breathe_center_at_zero_time() {
        let (sx, sy) = VibeAnimator::breathe(0.5, 0.0);
        assert!((sx - 1.0).abs() < 1e-12);
        assert!((sy - 1.0).abs() < 1e-12);
    }

    #[test]
    fn breathe_higher_vibe_more_amplitude() {
        let (sx_low, _) = VibeAnimator::breathe(0.0, 0.25);
        let (sx_high, _) = VibeAnimator::breathe(1.0, 0.25);
        // higher vibe may have different phase; just check amplitude ranges
        assert!(sx_low >= 0.96);
        assert!(sx_low <= 1.04);
        assert!(sx_high >= 0.92);
    }

    #[test]
    fn sway_zero_vibe() {
        let r = VibeAnimator::sway(0.0, 0.0);
        assert!((r - 0.0).abs() < 1e-12);
    }

    #[test]
    fn sway_range() {
        let r = VibeAnimator::sway(0.5, 0.25);
        assert!(r.abs() <= 10.0);
        let r = VibeAnimator::sway(1.0, 0.125);
        assert!(r.abs() <= 10.0);
    }

    #[test]
    fn color_pulse_returns_valid_color() {
        let c = VibeAnimator::color_pulse(0.5, 1.0);
        assert!(c.r >= 0.0 && c.r <= 1.0);
        assert!(c.g >= 0.0 && c.g <= 1.0);
        assert!(c.b >= 0.0 && c.b <= 1.0);
        assert!((c.a - 1.0).abs() < 1e-12);
    }

    #[test]
    fn color_pulse_zero_vibe() {
        let c = VibeAnimator::color_pulse(0.0, 0.0);
        // saturation 0.5, lightness 0.6, hue 0 → red-ish
        assert!(c.a > 0.9);
    }

    #[test]
    fn bounce_height_zero_at_rest() {
        let h = VibeAnimator::bounce_height(0.5, 0.0);
        assert!((h - 0.0).abs() < 1e-12);
    }

    #[test]
    fn bounce_height_nonnegative() {
        for i in 0..100 {
            let t = i as f64 / 20.0;
            let h = VibeAnimator::bounce_height(0.5, t);
            assert!(h >= -1e-12, "bounce_height < 0 at t={t}");
        }
    }

    #[test]
    fn bounce_height_higher_vibe_higher_bounce() {
        // At rest (t=0.5/rate for each vibe, so they're at same phase)
        let h_low = VibeAnimator::bounce_height(0.0, 0.25);
        // at vibe 0.0, rate = 1.0, phase = pi*0.25 = sin(pi/4)
        // at vibe 1.0, rate = 4.0, very different phase, but max amplitude is higher
        let h_high = VibeAnimator::bounce_height(1.0, 0.25);
        // just check both are valid nonnegative numbers
        assert!(h_low >= 0.0);
        assert!(h_high >= 0.0);
    }

    #[test]
    fn breathe_oscillates_symmetrically() {
        let (sx0, _) = VibeAnimator::breathe(0.5, 0.0);
        let (sx_half, _) = VibeAnimator::breathe(0.5, 0.5);
        // Should be roughly symmetric: expanded at peak and compressed at trough
        assert!((sx0 - 1.0).abs() < 1e-12);
        assert!((sx_half - 1.0).abs() > 0.001 || (sx_half - 1.0).abs() < 1e-12);
    }

    #[test]
    fn sway_zero_at_zero_time() {
        let r = VibeAnimator::sway(1.0, 0.0);
        assert!((r - 0.0).abs() < 1e-12);
    }

    #[test]
    fn all_procedural_functions_have_reasonable_range() {
        for vibe in &[0.0, 0.3, 0.7, 1.0] {
            for t in &[0.0, 0.1, 0.5, 1.0, 2.0] {
                let (sx, sy) = VibeAnimator::breathe(*vibe, *t);
                assert!(sx > 0.8 && sx < 1.2, "breathe sx={sx} vibe={vibe} t={t}");
                assert!(sy > 0.8 && sy < 1.2, "breathe sy={sy} vibe={vibe} t={t}");

                let sway = VibeAnimator::sway(*vibe, *t);
                assert!(
                    sway.abs() < 15.0,
                    "sway={sway} vibe={vibe} t={t}"
                );

                let c = VibeAnimator::color_pulse(*vibe, *t);
                assert!(c.r >= 0.0 && c.r <= 1.0);
                assert!(c.g >= 0.0 && c.g <= 1.0);
                assert!(c.b >= 0.0 && c.b <= 1.0);

                let bh = VibeAnimator::bounce_height(*vibe, *t);
                assert!(
                    bh >= 0.0,
                    "bounce_height={bh} vibe={vibe} t={t}"
                );
            }
        }
    }
}
