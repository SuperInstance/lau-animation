# lau-animation

> Sprite/property animation system — keyframe interpolation, skeletal hints, procedural animation from vibe

## What This Does

Sprite/property animation system — keyframe interpolation, skeletal hints, procedural animation from vibe. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-animation
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_animation::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub enum Easing 
    pub fn apply(&self, t: f64) -> f64 
pub struct AnimationClip 
    pub fn sample(&self, time: f64) -> HashMap<String, PropertyValue> 
    pub fn idle() -> AnimationClip 
    pub fn walk() -> AnimationClip 
    pub fn jump() -> AnimationClip 
    pub fn celebrate() -> AnimationClip 
pub struct AnimationTrack<T: Clone + Lerp> 
    pub fn new() -> Self 
    pub fn add_keyframe(&mut self, kf: Keyframe<T>) 
    pub fn sample(&self, time: f64) -> T 
    pub fn duration(&self) -> f64 
    pub fn is_empty(&self) -> bool 
    pub fn looping_sample(&self, time: f64) -> T 
pub trait Lerp: Clone 
pub struct AnimationBlender;
    pub fn blend(
pub struct Keyframe<T: Clone> 
    pub fn new(time: f64, value: T, easing: Easing) -> Self 
pub struct VibeAnimator;
    pub fn breathe(vibe: f64, time: f64) -> (f64, f64) 
    pub fn sway(vibe: f64, time: f64) -> f64 
    pub fn color_pulse(vibe: f64, time: f64) -> Color 
    pub fn bounce_height(vibe: f64, time: f64) -> f64 
pub struct AnimationPlayer 
    pub fn new() -> Self 
    pub fn play(&mut self, clip: AnimationClip) 
    pub fn stop(&mut self) 
    pub fn pause(&mut self) 
    pub fn resume(&mut self) 
    pub fn tick(&mut self, dt: f64) 
    pub fn current_values(&self) -> HashMap<String, PropertyValue> 
    pub fn set_speed(&mut self, speed: f64) 
    pub fn set_time(&mut self, time: f64) 
    pub fn is_playing(&self) -> bool 
    pub fn progress(&self) -> f64 
pub enum PropertyValue 
pub enum AnimationTrackValue 
pub struct Color 
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**90 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
