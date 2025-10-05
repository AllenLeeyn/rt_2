pub mod core;
pub mod material;
pub mod scene;
pub mod pixels;
pub mod objects;

use std::ops::Mul;
pub fn sq<T: Mul<Output = T> + Copy>(v: T) -> T {
    v * v
}

use rand::Rng;
pub fn random_double() -> f32 {
    rand::rng().random::<f32>()
}
