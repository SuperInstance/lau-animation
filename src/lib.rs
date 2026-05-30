//! `lau-animation` — sprite/property animation system for the game "Glitchmere".
//!
//! Keyframe interpolation, skeletal hints, procedural animation from vibe.
//!
//! # Features
//!
//! - **Keyframes & Tracks** — typed keyframes with easing, sampled at any time
//! - **Animation Clips** — named collections of tracks, with looping support
//! - **Animation Player** — playback (play/pause/stop/speed/time) with tick
//! - **Animation Blender** — blend two clips at a given blend factor
//! - **Vibe Animator** — procedural animation from a vibe value (breathe, sway,
//!   color-pulse, bounce)
//! - **Pre-built clips** — idle, walk, jump, celebrate

pub mod blender;
pub mod clip;
pub mod color;
pub mod easing;
pub mod keyframe;
pub mod lerp;
pub mod player;
pub mod track;
pub mod vibe;

pub use blender::AnimationBlender;
pub use clip::AnimationClip;
pub use color::Color;
pub use easing::Easing;
pub use keyframe::Keyframe;
pub use lerp::Lerp;
pub use player::AnimationPlayer;
pub use track::AnimationTrack;
pub use vibe::VibeAnimator;

/// Serialized property value sampled from a track.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Float(f64),
    Vec2((f64, f64)),
    Color(Color),
}

/// A value in an animation track, holding one of several typed track types.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AnimationTrackValue {
    Float(AnimationTrack<f64>),
    Vec2(AnimationTrack<(f64, f64)>),
    Color(AnimationTrack<Color>),
}
