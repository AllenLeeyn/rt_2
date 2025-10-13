use crate::core::{Hittable, Point3};

pub struct ParticleSys<F>
where
    F: Fn(usize, Point3) -> Box<dyn Hittable>,
{
    pub min: Point3,
    pub max: Point3,
    pub count: usize,
    pub pattern: F,
    pub min_dist: f32, // minimum allowed distance between particles
}

impl<F> ParticleSys<F>
where
    F: Fn(usize, Point3) -> Box<dyn Hittable>,
{
    pub fn new(min: Point3, max: Point3, count: usize, pattern: F, min_dist: f32) -> Self {
        Self {
            min,
            max,
            count,
            pattern,
            min_dist,
        }
    }

    /// Generate all particle objects ensuring no overlapping within min_dist
    pub fn generate(&self) -> Vec<Box<dyn Hittable>> {
        use rand::Rng;
        let mut rng = rand::rng();

        let mut particles = Vec::with_capacity(self.count);
        let mut points: Vec<Point3> = Vec::with_capacity(self.count);

        let max_attempts = 100;

        while points.len() < self.count {
            let mut attempts = 0;
            let mut candidate = Point3::new(
                rng.random_range(self.min.x()..self.max.x()),
                rng.random_range(self.min.y()..self.max.y()),
                rng.random_range(self.min.z()..self.max.z()),
            );

            // Retry until candidate is far enough from all others, or max_attempts reached
            while points.iter().any(|p| p.distance(candidate) < self.min_dist)
                && attempts < max_attempts
            {
                candidate = Point3::new(
                    rng.random_range(self.min.x()..self.max.x()),
                    rng.random_range(self.min.y()..self.max.y()),
                    rng.random_range(self.min.z()..self.max.z()),
                );
                attempts += 1;
            }

            if attempts == max_attempts {
                eprintln!(
                    "Warning: max attempts reached, could not place more particles without overlap"
                );
                break;
            }

            let index = points.len();
            points.push(candidate);
            particles.push((self.pattern)(index, candidate));
        }

        particles
    }
}
