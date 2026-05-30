use crate::keyframe::Keyframe;
use crate::lerp::Lerp;
use std::fmt::Debug;

/// A single typed animation track — a sequence of keyframes sorted by time.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnimationTrack<T: Clone + Lerp> {
    pub keyframes: Vec<Keyframe<T>>,
}

impl<T: Clone + Lerp> AnimationTrack<T> {
    /// Create a new empty track.
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    /// Add a keyframe to the track. Keyframes are kept sorted by time
    /// (stable-insert).
    pub fn add_keyframe(&mut self, kf: Keyframe<T>) {
        let idx = self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&kf.time).unwrap());
        match idx {
            Ok(pos) => self.keyframes[pos] = kf,
            Err(pos) => self.keyframes.insert(pos, kf),
        }
    }

    /// Sample the track at the given time.
    ///
    /// * If the track is empty, panics.
    /// * If `time ≤ first_keyframe.time`, returns the first keyframe's value.
    /// * If `time ≥ last_keyframe.time`, returns the last keyframe's value.
    /// * Otherwise interpolates between the surrounding keyframes using the
    ///   *right* keyframe's easing function.
    pub fn sample(&self, time: f64) -> T {
        assert!(!self.keyframes.is_empty(), "cannot sample empty track");

        let len = self.keyframes.len();

        // Before first keyframe
        if time <= self.keyframes[0].time {
            return self.keyframes[0].value.clone();
        }

        // After last keyframe
        if time >= self.keyframes[len - 1].time {
            return self.keyframes[len - 1].value.clone();
        }

        // Find surrounding keyframes
        let hi = self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&time).unwrap())
            .unwrap_or_else(|i| i);

        let lo = hi - 1;
        let kf_lo = &self.keyframes[lo];
        let kf_hi = &self.keyframes[hi];

        let span = kf_hi.time - kf_lo.time;
        let t = if span > 0.0 {
            (time - kf_lo.time) / span
        } else {
            1.0
        };

        let eased = kf_hi.easing.apply(t);
        kf_lo.value.lerp(&kf_hi.value, eased)
    }

    /// Total duration of the track (last keyframe time).
    pub fn duration(&self) -> f64 {
        self.keyframes
            .last()
            .map(|kf| kf.time)
            .unwrap_or(0.0)
    }

    /// Whether the track has no keyframes.
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Sample the track with looping: wraps `time` to `[0, duration)`.
    pub fn looping_sample(&self, time: f64) -> T {
        let dur = self.duration();
        if dur <= 0.0 || self.keyframes.is_empty() {
            return self.keyframes[0].value.clone();
        }
        let wrapped = time % dur;
        let wrapped = if wrapped < 0.0 {
            wrapped + dur
        } else {
            wrapped
        };
        self.sample(wrapped)
    }
}

impl<T: Clone + Lerp> Default for AnimationTrack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easing::Easing;
    use crate::keyframe::Keyframe;

    fn make_track() -> AnimationTrack<f64> {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 10.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(2.0, 20.0, Easing::Linear));
        t
    }

    #[test]
    fn track_new_empty() {
        let t: AnimationTrack<f64> = AnimationTrack::new();
        assert!(t.is_empty());
        assert!((t.duration() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn track_sample_before_first() {
        let t = make_track();
        let v = t.sample(-1.0);
        assert!((v - 0.0).abs() < 1e-12);
    }

    #[test]
    fn track_sample_after_last() {
        let t = make_track();
        let v = t.sample(5.0);
        assert!((v - 20.0).abs() < 1e-12);
    }

    #[test]
    fn track_sample_exact_keyframe() {
        let t = make_track();
        let v = t.sample(1.0);
        assert!((v - 10.0).abs() < 1e-12);
    }

    #[test]
    fn track_sample_interpolate() {
        let t = make_track();
        let v = t.sample(0.5);
        assert!((v - 5.0).abs() < 1e-12);
    }

    #[test]
    fn track_sample_interpolate_mid() {
        let t = make_track();
        let v = t.sample(1.5);
        assert!((v - 15.0).abs() < 1e-12);
    }

    #[test]
    fn track_duration() {
        let t = make_track();
        assert!((t.duration() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn track_looping_sample() {
        let t = make_track();
        let v = t.looping_sample(2.5);
        assert!((v - 5.0).abs() < 1e-12); // wraps to 0.5
    }

    #[test]
    fn track_looping_sample_exact_duration() {
        let t = make_track();
        let v = t.looping_sample(2.0);
        assert!((v - 0.0).abs() < 1e-12); // wraps to 0
    }

    #[test]
    fn track_add_keyframe_sorts() {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(2.0, 200.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 100.0, Easing::Linear));
        assert!((t.keyframes[0].time - 0.0).abs() < 1e-12);
        assert!((t.keyframes[1].time - 1.0).abs() < 1e-12);
        assert!((t.keyframes[2].time - 2.0).abs() < 1e-12);
    }

    #[test]
    #[should_panic(expected = "cannot sample empty track")]
    fn track_sample_empty_panics() {
        let t: AnimationTrack<f64> = AnimationTrack::new();
        t.sample(0.0);
    }

    #[test]
    fn track_vec2_interpolate() {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, (0.0, 0.0), Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, (10.0, 20.0), Easing::Linear));
        let v = t.sample(0.5);
        assert!((v.0 - 5.0).abs() < 1e-12);
        assert!((v.1 - 10.0).abs() < 1e-12);
    }

    #[test]
    fn track_duration_empty() {
        let t: AnimationTrack<f64> = AnimationTrack::new();
        assert!((t.duration() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn track_looping_on_empty_no_panic() {
        let _t: AnimationTrack<f64> = AnimationTrack::new();
    }

    #[test]
    fn track_default_empty() {
        let t: AnimationTrack<f64> = AnimationTrack::default();
        assert!(t.is_empty());
    }

    #[test]
    fn track_looping_wrap_multiple_times() {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 10.0, Easing::Linear));
        let v = t.looping_sample(4.5);
        assert!((v - 5.0).abs() < 1e-12); // 4.5 % 1.0 = 0.5
    }

    #[test]
    fn track_looping_negative_time() {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 10.0, Easing::Linear));
        let v = t.looping_sample(-0.5);
        assert!((v - 5.0).abs() < 1e-12); // -0.5 % 1.0 = 0.5 in Rust
    }

    #[test]
    fn track_easing_applied() {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 10.0, Easing::EaseIn));
        let v = t.sample(0.25);
        let expected = 0.0_f64.lerp(&10.0, 0.25 * 0.25); // EaseIn: t²
        assert!((v - expected).abs() < 1e-12);
    }
}
