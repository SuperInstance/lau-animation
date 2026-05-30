use crate::easing::Easing;
use serde::{Deserialize, Serialize};

/// A single keyframe storing a value at a point in time with an easing function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe<T: Clone> {
    pub time: f64,
    pub value: T,
    pub easing: Easing,
}

impl<T: Clone> Keyframe<T> {
    /// Create a new keyframe.
    pub fn new(time: f64, value: T, easing: Easing) -> Self {
        Self { time, value, easing }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyframe_new() {
        let kf = Keyframe::new(0.5, 42.0, Easing::EaseInOut);
        assert!((kf.time - 0.5).abs() < 1e-12);
        assert!((kf.value - 42.0_f64).abs() < 1e-12);
        assert_eq!(kf.easing, Easing::EaseInOut);
    }

    #[test]
    fn keyframe_clone() {
        let kf = Keyframe::new(1.0, (3.0, 4.0), Easing::Linear);
        let kf2 = kf.clone();
        assert!((kf2.time - 1.0).abs() < 1e-12);
        assert_eq!(kf2.value, (3.0, 4.0));
    }
}
