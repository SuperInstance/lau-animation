use crate::clip::AnimationClip;
use crate::PropertyValue;
use std::collections::HashMap;

/// Plays (or pauses/loops/scales) an `AnimationClip`.
#[derive(Debug, Clone)]
pub struct AnimationPlayer {
    pub clip: Option<AnimationClip>,
    pub time: f64,
    pub speed: f64,
    pub playing: bool,
}

impl AnimationPlayer {
    /// Create a new stopped player with no clip loaded.
    pub fn new() -> Self {
        Self {
            clip: None,
            time: 0.0,
            speed: 1.0,
            playing: false,
        }
    }

    /// Load and start playing a clip from time 0.
    pub fn play(&mut self, clip: AnimationClip) {
        self.clip = Some(clip);
        self.time = 0.0;
        self.playing = true;
    }

    /// Stop playback and reset time.
    pub fn stop(&mut self) {
        self.playing = false;
        self.time = 0.0;
    }

    /// Pause playback (keep position).
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Resume playback from current position.
    pub fn resume(&mut self) {
        self.playing = true;
    }

    /// Advance time by `dt` seconds (respects speed and looping).
    pub fn tick(&mut self, dt: f64) {
        if !self.playing || self.clip.is_none() {
            return;
        }
        self.time += dt * self.speed;

        let clip = self.clip.as_ref().unwrap();
        if clip.looping && clip.duration > 0.0 {
            self.time %= clip.duration;
            if self.time < 0.0 {
                self.time += clip.duration; // negative wrap
            }
        } else if self.time >= clip.duration {
            self.time = clip.duration;
            self.playing = false;
        } else if self.time < 0.0 {
            self.time = 0.0;
            self.playing = false;
        }
    }

    /// Sample the current clip at the current time.
    pub fn current_values(&self) -> HashMap<String, PropertyValue> {
        match &self.clip {
            Some(clip) => clip.sample(self.time),
            None => HashMap::new(),
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time.max(0.0);
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Progress as a fraction `[0, 1]` of the current clip.
    pub fn progress(&self) -> f64 {
        match &self.clip {
            Some(clip) if clip.duration > 0.0 => (self.time / clip.duration).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new()
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

    fn make_test_clip() -> AnimationClip {
        let mut t = AnimationTrack::new();
        t.add_keyframe(Keyframe::new(0.0, 0.0, Easing::Linear));
        t.add_keyframe(Keyframe::new(1.0, 10.0, Easing::Linear));
        let mut tracks = HashMap::new();
        tracks.insert("x".into(), AnimationTrackValue::Float(t));
        AnimationClip {
            name: "test".into(),
            tracks,
            duration: 1.0,
            looping: false,
        }
    }

    #[test]
    fn player_new() {
        let p = AnimationPlayer::new();
        assert!(!p.is_playing());
        assert!(p.clip.is_none());
        assert!((p.progress() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn player_play() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        assert!(p.is_playing());
        assert!((p.time - 0.0).abs() < 1e-12);
    }

    #[test]
    fn player_tick_advances() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(0.5);
        assert!((p.time - 0.5).abs() < 1e-12);
    }

    #[test]
    fn player_tick_nonlooping_stops_at_end() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(2.0);
        assert!(!p.is_playing());
        assert!((p.time - 1.0).abs() < 1e-12);
    }

    #[test]
    fn player_tick_looping_wraps() {
        let mut clip = make_test_clip();
        clip.looping = true;
        let mut p = AnimationPlayer::new();
        p.play(clip);
        p.tick(1.5);
        assert!(p.is_playing());
        assert!((p.time - 0.5).abs() < 1e-12);
    }

    #[test]
    fn player_stop() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(0.3);
        p.stop();
        assert!(!p.is_playing());
        assert!((p.time - 0.0).abs() < 1e-12);
    }

    #[test]
    fn player_pause_resume() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(0.3);
        p.pause();
        assert!(!p.is_playing());
        let time_before = p.time;
        p.tick(1.0);
        assert!((p.time - time_before).abs() < 1e-12);
        p.resume();
        assert!(p.is_playing());
    }

    #[test]
    fn player_set_speed() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.set_speed(2.0);
        p.tick(0.5);
        assert!((p.time - 1.0).abs() < 1e-12);
    }

    #[test]
    fn player_set_time() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.set_time(0.7);
        assert!((p.time - 0.7).abs() < 1e-12);
    }

    #[test]
    fn player_progress() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(0.25);
        assert!((p.progress() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn player_progress_no_clip() {
        let p = AnimationPlayer::new();
        assert!((p.progress() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn player_current_values() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.tick(0.5);
        let vals = p.current_values();
        if let Some(PropertyValue::Float(v)) = vals.get("x") {
            assert!((v - 5.0).abs() < 1e-12);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn player_current_values_no_clip() {
        let p = AnimationPlayer::new();
        let vals = p.current_values();
        assert!(vals.is_empty());
    }

    #[test]
    fn player_play_prebuilt() {
        let mut p = AnimationPlayer::new();
        p.play(prebuilt::idle());
        assert!(p.is_playing());
        let vals = p.current_values();
        assert!(vals.contains_key("scale_x"));
        assert!(vals.contains_key("scale_y"));
    }

    #[test]
    fn player_tick_negative_speed() {
        let mut p = AnimationPlayer::new();
        p.play(make_test_clip());
        p.set_speed(-1.0);
        p.tick(0.3);
        // should snap to 0 since non-lopping and time < 0
        assert!((p.time - 0.0).abs() < 1e-12);
        assert!(!p.is_playing());
    }

    #[test]
    fn player_tick_negative_speed_looping() {
        let mut clip = make_test_clip();
        clip.looping = true;
        let mut p = AnimationPlayer::new();
        p.play(clip);
        p.set_time(0.5);
        p.set_speed(-1.0);
        p.tick(0.3);
        // 0.5 - 0.3 = 0.2 for looping
        assert!((p.time - 0.2).abs() < 1e-12);
    }
}
