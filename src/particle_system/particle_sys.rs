use crate::core::{Hittable, Point3};

pub struct ParticleSys<F>
where
    F: Fn(Point3) -> Box<dyn Hittable>,
{
    pub min: Point3,
    pub max: Point3,
    pub count: usize,
    pub pattern: F,
}
impl<F> ParticleSys<F>
where
    F: Fn(Point3) -> Box<dyn Hittable>,
{
    pub fn new(min: Point3, max: Point3, count: usize, pattern: F) -> Self {
        Self {
            min,
            max,
            count,
            pattern,
        }
    }

    /// Generate all particle objects
    pub fn generate(&self) -> Vec<Box<dyn Hittable>> {
        use rand::Rng;
        let mut rng = rand::rng();

        let mut particles = Vec::with_capacity(self.count);

        for _ in 0..self.count {
            let p = Point3::new(
                rng.random_range(self.min.x()..self.max.x()),
                rng.random_range(self.min.y()..self.max.y()),
                rng.random_range(self.min.z()..self.max.z()),
            );

            particles.push((self.pattern)(p));
        }

        particles
    }
}
