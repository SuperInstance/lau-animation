use crate::AnimationTrackValue;
use crate::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named collection of animation tracks with a total duration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    pub name: String,
    pub tracks: HashMap<String, AnimationTrackValue>,
    pub duration: f64,
    pub looping: bool,
}

impl AnimationClip {
    /// Sample this clip at the given time, returning a map of property names
    /// to their sampled values.
    ///
    /// If `looping` is true, time wraps around the clip duration.
    /// If not looping, time is clamped to `[0, duration]`.
    pub fn sample(&self, time: f64) -> HashMap<String, PropertyValue> {
        let sample_time = if self.duration > 0.0 {
            if self.looping {
                let t = time % self.duration;
                if t < 0.0 {
                    t + self.duration
                } else {
                    t
                }
            } else {
                time.clamp(0.0, self.duration)
            }
        } else {
            time
        };

        self.tracks
            .iter()
            .map(|(name, track)| {
                let value = match track {
                    AnimationTrackValue::Float(t) => {
                        PropertyValue::Float(t.sample(sample_time))
                    }
                    AnimationTrackValue::Vec2(t) => {
                        PropertyValue::Vec2(t.sample(sample_time))
                    }
                    AnimationTrackValue::Color(t) => {
                        PropertyValue::Color(t.sample(sample_time))
                    }
                };
                (name.clone(), value)
            })
            .collect()
    }
}

/// Create a set of pre-built animation clips.
pub mod prebuilt {
    use crate::clip::AnimationClip;
    use crate::color::Color;
    use crate::easing::Easing;
    use crate::keyframe::Keyframe;
    use crate::track::AnimationTrack;
    use crate::AnimationTrackValue;
    use std::collections::HashMap;

    /// Idle: subtle breathing (scale oscillation).
    ///
    /// duration ≈ 2.0s, looping.
    pub fn idle() -> AnimationClip {
        let mut scale_x = AnimationTrack::new();
        let mut scale_y = AnimationTrack::new();

        // Subtle breathe: scale goes 1.0 → 1.02 → 1.0 over 2 seconds
        scale_x.add_keyframe(Keyframe::new(0.0, 1.0, Easing::EaseInOut));
        scale_x.add_keyframe(Keyframe::new(1.0, 1.02, Easing::EaseInOut));
        scale_x.add_keyframe(Keyframe::new(2.0, 1.0, Easing::EaseInOut));

        scale_y.add_keyframe(Keyframe::new(0.0, 1.0, Easing::EaseInOut));
        scale_y.add_keyframe(Keyframe::new(1.0, 1.02, Easing::EaseInOut));
        scale_y.add_keyframe(Keyframe::new(2.0, 1.0, Easing::EaseInOut));

        let mut tracks = HashMap::new();
        tracks.insert("scale_x".into(), AnimationTrackValue::Float(scale_x));
        tracks.insert("scale_y".into(), AnimationTrackValue::Float(scale_y));

        AnimationClip {
            name: "idle".into(),
            tracks,
            duration: 2.0,
            looping: true,
        }
    }

    /// Walk: bobbing + swaying.
    ///
    /// duration ≈ 0.8s, looping.
    pub fn walk() -> AnimationClip {
        let mut pos_y = AnimationTrack::new();
        let mut rotation = AnimationTrack::new();

        // Bob up and down
        pos_y.add_keyframe(Keyframe::new(0.0, 0.0, Easing::EaseOut));
        pos_y.add_keyframe(Keyframe::new(0.4, 3.0, Easing::EaseInOut));
        pos_y.add_keyframe(Keyframe::new(0.8, 0.0, Easing::EaseIn));

        // Sway side to side (rotation in degrees, treat as f64)
        rotation.add_keyframe(Keyframe::new(0.0, -2.0, Easing::EaseInOut));
        rotation.add_keyframe(Keyframe::new(0.4, 2.0, Easing::EaseInOut));
        rotation.add_keyframe(Keyframe::new(0.8, -2.0, Easing::EaseInOut));

        let mut tracks = HashMap::new();
        tracks.insert("pos_y".into(), AnimationTrackValue::Float(pos_y));
        tracks.insert("rotation".into(), AnimationTrackValue::Float(rotation));

        AnimationClip {
            name: "walk".into(),
            tracks,
            duration: 0.8,
            looping: true,
        }
    }

    /// Jump: anticipation → launch → land.
    ///
    /// duration ≈ 0.7s, non-looping.
    pub fn jump() -> AnimationClip {
        let mut pos_y = AnimationTrack::new();
        let mut scale_y = AnimationTrack::new();

        // Anticipation (squat), launch, apex, land
        pos_y.add_keyframe(Keyframe::new(0.0, 0.0, Easing::EaseOut));
        pos_y.add_keyframe(Keyframe::new(0.15, -2.0, Easing::EaseIn)); // squat
        pos_y.add_keyframe(Keyframe::new(0.4, 20.0, Easing::EaseOut)); // apex
        pos_y.add_keyframe(Keyframe::new(0.65, 0.0, Easing::Bounce)); // land

        // Stretch during jump
        scale_y.add_keyframe(Keyframe::new(0.0, 1.0, Easing::EaseOut));
        scale_y.add_keyframe(Keyframe::new(0.15, 0.8, Easing::EaseIn)); // squash
        scale_y.add_keyframe(Keyframe::new(0.4, 1.2, Easing::EaseOut)); // stretch
        scale_y.add_keyframe(Keyframe::new(0.65, 1.0, Easing::Bounce)); // recover

        let mut tracks = HashMap::new();
        tracks.insert("pos_y".into(), AnimationTrackValue::Float(pos_y));
        tracks.insert("scale_y".into(), AnimationTrackValue::Float(scale_y));

        AnimationClip {
            name: "jump".into(),
            tracks,
            duration: 0.65,
            looping: false,
        }
    }

    /// Celebrate: bounce + color pulse.
    ///
    /// duration ≈ 1.5s, looping.
    pub fn celebrate() -> AnimationClip {
        let mut pos_y = AnimationTrack::new();
        let mut color = AnimationTrack::new();

        // Bounce
        pos_y.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Bounce));
        pos_y.add_keyframe(Keyframe::new(0.5, 15.0, Easing::Bounce));
        pos_y.add_keyframe(Keyframe::new(1.0, 0.0, Easing::Bounce));
        pos_y.add_keyframe(Keyframe::new(1.5, 0.0, Easing::Bounce));

        // Color pulse: white → gold → white
        color.add_keyframe(Keyframe::new(
            0.0,
            Color::WHITE,
            Easing::EaseInOut,
        ));
        color.add_keyframe(Keyframe::new(
            0.75,
            Color::new(1.0, 0.84, 0.0, 1.0), // gold
            Easing::EaseInOut,
        ));
        color.add_keyframe(Keyframe::new(
            1.5,
            Color::WHITE,
            Easing::EaseInOut,
        ));

        let mut tracks = HashMap::new();
        tracks.insert("pos_y".into(), AnimationTrackValue::Float(pos_y));
        tracks.insert("color".into(), AnimationTrackValue::Color(color));

        AnimationClip {
            name: "celebrate".into(),
            tracks,
            duration: 1.5,
            looping: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnimationTrackValue;

    fn make_simple_clip() -> AnimationClip {
        let mut t = crate::track::AnimationTrack::new();
        t.add_keyframe(crate::keyframe::Keyframe::new(
            0.0,
            0.0,
            crate::easing::Easing::Linear,
        ));
        t.add_keyframe(crate::keyframe::Keyframe::new(
            1.0,
            10.0,
            crate::easing::Easing::Linear,
        ));
        let mut tracks = HashMap::new();
        tracks.insert("height".into(), AnimationTrackValue::Float(t));
        AnimationClip {
            name: "test".into(),
            tracks,
            duration: 1.0,
            looping: true,
        }
    }

    #[test]
    fn clip_sample_nonlooping() {
        let mut clip = make_simple_clip();
        clip.looping = false;
        let result = clip.sample(2.0);
        if let Some(PropertyValue::Float(v)) = result.get("height") {
            assert!((v - 10.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn clip_sample_looping_wrap() {
        let clip = make_simple_clip();
        let result = clip.sample(2.5);
        if let Some(PropertyValue::Float(v)) = result.get("height") {
            assert!((v - 5.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn clip_sample_exact_start() {
        let clip = make_simple_clip();
        let result = clip.sample(0.0);
        if let Some(PropertyValue::Float(v)) = result.get("height") {
            assert!((v - 0.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn prebuilt_idle_not_empty() {
        let clip = prebuilt::idle();
        assert!(!clip.tracks.is_empty());
        assert!(clip.looping);
        assert!((clip.duration - 2.0).abs() < 1e-12);
    }

    #[test]
    fn prebuilt_walk_has_tracks() {
        let clip = prebuilt::walk();
        assert!(clip.tracks.contains_key("pos_y"));
        assert!(clip.tracks.contains_key("rotation"));
    }

    #[test]
    fn prebuilt_jump_nonlooping() {
        let clip = prebuilt::jump();
        assert!(!clip.looping);
    }

    #[test]
    fn prebuilt_celebrate_has_color() {
        let clip = prebuilt::celebrate();
        assert!(clip.tracks.contains_key("color"));
    }

    #[test]
    fn prebuilt_jump_returns_values() {
        let clip = prebuilt::jump();
        let result = clip.sample(0.0);
        assert!(result.contains_key("pos_y"));
        assert!(result.contains_key("scale_y"));
    }

    #[test]
    fn clip_multiple_tracks() {
        let mut pos = crate::track::AnimationTrack::new();
        pos.add_keyframe(crate::keyframe::Keyframe::new(
            0.0,
            (0.0, 0.0),
            crate::easing::Easing::Linear,
        ));
        pos.add_keyframe(crate::keyframe::Keyframe::new(
            1.0,
            (10.0, 20.0),
            crate::easing::Easing::Linear,
        ));

        let mut col = crate::track::AnimationTrack::new();
        col.add_keyframe(crate::keyframe::Keyframe::new(
            0.0,
            crate::Color::BLACK,
            crate::easing::Easing::Linear,
        ));
        col.add_keyframe(crate::keyframe::Keyframe::new(
            1.0,
            crate::Color::WHITE,
            crate::easing::Easing::Linear,
        ));

        let mut tracks = HashMap::new();
        tracks.insert("position".into(), AnimationTrackValue::Vec2(pos));
        tracks.insert("tint".into(), AnimationTrackValue::Color(col));

        let clip = AnimationClip {
            name: "multi".into(),
            tracks,
            duration: 1.0,
            looping: false,
        };

        let result = clip.sample(0.5);
        if let Some(PropertyValue::Vec2(v)) = result.get("position") {
            assert!((v.0 - 5.0).abs() < 1e-12);
            assert!((v.1 - 10.0).abs() < 1e-12);
        } else {
            panic!("expected Vec2");
        }
        if let Some(PropertyValue::Color(c)) = result.get("tint") {
            assert!((c.r - 0.5).abs() < 1e-12);
        } else {
            panic!("expected Color");
        }
    }
}
