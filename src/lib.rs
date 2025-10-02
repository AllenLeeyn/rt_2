pub mod core;
pub mod material;
pub mod objects;
pub mod pixels;
pub mod scene;

use std::ops::Mul;
pub fn sq<T: Mul<Output = T> + Copy>(v: T) -> T {
    v * v
}
