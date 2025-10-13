pub mod core;
pub mod material;
pub mod objects;
pub mod particle_system;
pub mod pixels;
pub mod scene;

use std::ops::Mul;
pub fn square<T: Mul<Output = T> + Copy>(value: T) -> T {
    value * value
}

use rand::Rng;
pub fn random_float() -> f32 {
    rand::rng().random::<f32>()
}
