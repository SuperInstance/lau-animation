use crate::clip::AnimationClip;
use crate::PropertyValue;

/// AnimationBlender blends two clips at a given blend factor.
pub struct AnimationBlender;

impl AnimationBlender {
    /// Blend two animation clips.
    ///
    /// Both clips are sampled at `time`, then their values are linearly
    /// interpolated using `blend_factor` (`0.0` = pure A, `1.0` = pure B).
    ///
    /// Only properties present in *both* clips are blended. Properties unique
    /// to one clip are taken as-is.
    pub fn blend(
        clip_a: &AnimationClip,
        clip_b: &AnimationClip,
        time: f64,
        blend_factor: f64,
    ) -> std::collections::HashMap<String, PropertyValue> {
        let a = clip_a.sample(time);
        let b = clip_b.sample(time);
        let bf = blend_factor.clamp(0.0, 1.0);

        let mut result = std::collections::HashMap::new();

        // Properties in both: blend
        for (key, val_a) in &a {
            if let Some(val_b) = b.get(key) {
                result.insert(key.clone(), blend_values(val_a, val_b, bf));
            }
        }

        // Properties only in B
        for (key, val_b) in &b {
            if !result.contains_key(key) && !a.contains_key(key) {
                result.insert(key.clone(), val_b.clone());
            }
        }

        // Properties only in A
        for (key, val_a) in &a {
            if !result.contains_key(key) {
                result.insert(key.clone(), val_a.clone());
            }
        }

        result
    }
}

fn blend_values(a: &PropertyValue, b: &PropertyValue, t: f64) -> PropertyValue {
    use crate::lerp::Lerp;
    match (a, b) {
        (PropertyValue::Float(af), PropertyValue::Float(bf)) => {
            PropertyValue::Float(af.lerp(bf, t))
        }
        (PropertyValue::Vec2(av), PropertyValue::Vec2(bv)) => {
            PropertyValue::Vec2(av.lerp(bv, t))
        }
        (PropertyValue::Color(ac), PropertyValue::Color(bc)) => {
            PropertyValue::Color(ac.lerp(bc, t))
        }
        _ => a.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clip::prebuilt;
    use crate::easing::Easing;
    use crate::keyframe::Keyframe;
    use crate::track::AnimationTrack;
    use crate::AnimationTrackValue;
    use std::collections::HashMap;

    fn make_clip(name: &str, start: f64, end: f64) -> AnimationClip {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, start, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, end, Easing::Linear));
        let mut tracks = HashMap::new();
        tracks.insert("value".into(), AnimationTrackValue::Float(t));
        AnimationClip {
            name: name.into(),
            tracks,
            duration: 1.0,
            looping: true,
        }
    }

    #[test]
    fn blend_pure_a() {
        let a = make_clip("a", 0.0, 10.0);
        let b = make_clip("b", 100.0, 200.0);
        let r = AnimationBlender::blend(&a, &b, 0.5, 0.0);
        if let Some(PropertyValue::Float(v)) = r.get("value") {
            assert!((v - 5.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn blend_pure_b() {
        let a = make_clip("a", 0.0, 10.0);
        let b = make_clip("b", 100.0, 200.0);
        let r = AnimationBlender::blend(&a, &b, 0.5, 1.0);
        if let Some(PropertyValue::Float(v)) = r.get("value") {
            assert!((v - 150.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn blend_midpoint() {
        let a = make_clip("a", 0.0, 10.0);
        let b = make_clip("b", 100.0, 200.0);
        let r = AnimationBlender::blend(&a, &b, 0.5, 0.5);
        if let Some(PropertyValue::Float(v)) = r.get("value") {
            assert!((v - 77.5).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn blend_clamp_factor() {
        let a = make_clip("a", 0.0, 10.0);
        let b = make_clip("b", 100.0, 200.0);
        let r = AnimationBlender::blend(&a, &b, 0.0, 1.5);
        if let Some(PropertyValue::Float(v)) = r.get("value") {
            assert!((v - 100.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn blend_unique_props_from_a() {
        let mut t_a = AnimationTrack::new();
        t_a.add_keyframe(Keyframe::new(0.0, 1.0, Easing::Linear));
        let mut t_b = AnimationTrack::new();
        t_b.add_keyframe(Keyframe::new(0.0, 2.0, Easing::Linear));

        let mut tracks_a = HashMap::new();
        tracks_a.insert("a_only".into(), AnimationTrackValue::Float(t_a));
        let mut tracks_b = HashMap::new();
        tracks_b.insert("b_only".into(), AnimationTrackValue::Float(t_b));

        let a = AnimationClip {
            name: "a".into(),
            tracks: tracks_a,
            duration: 1.0,
            looping: true,
        };
        let b = AnimationClip {
            name: "b".into(),
            tracks: tracks_b,
            duration: 1.0,
            looping: true,
        };

        let r = AnimationBlender::blend(&a, &b, 0.0, 0.5);
        assert!(r.contains_key("a_only"));
        assert!(r.contains_key("b_only"));
    }

    #[test]
    fn blend_with_prebuilt_clips() {
        let idle = prebuilt::idle();
        let walk = prebuilt::walk();
        let r = AnimationBlender::blend(&idle, &walk, 0.0, 0.5);
        // Should have combined keys from both
        assert!(r.contains_key("scale_x"));
        assert!(r.contains_key("pos_y"));
    }
}
