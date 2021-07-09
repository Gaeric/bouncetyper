use bevy::prelude::*;

pub trait Interpolation {
    fn lerp(self, begin: Self, end: Self) -> Self;
}

pub trait Damp {
    fn damp(self, target: Self, speed: f32, delta_seconds: f32) -> Self;
}

impl Interpolation for f32 {
    fn lerp(self, begin: Self, end: Self) -> Self {
        begin * (1.0 - self) + end * self
    }
}

impl Damp for f32 {
    fn damp(self, target: Self, speed: f32, delta_seconds: f32) -> Self {
        (1.0 - (-speed * delta_seconds).exp()).lerp(self, target)
    }
}

impl Damp for Vec2 {
    fn damp(self, target: Self, speed: f32, delta_seconds: f32) -> Self {
        Vec2::new(
            self.x.damp(target.x, speed, delta_seconds),
            self.y.damp(target.y, speed, delta_seconds),
        )
    }
}

pub fn merge_result<T, E>(first: Result<T, E>, second: Result<T, E>) -> Result<(T, T), E> {
    Ok((first?, second?))
}
