use rand::{Rng, rng, seq::SliceRandom};

use crate::vector::{Point3, Vec3};

#[cfg(false)]
#[derive(Clone, Debug)]
pub struct Perlin {
    random_vectors: Vec<Vec<f64>>
}

#[cfg(false)]
impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let mut rng = rng();
        let mut random_numbers = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            random_numbers.push(Vec3::rand_unit_vector());
        }
        let (cached_x, cached_y, cached_z) = (
            Self::generate_cache(point_count),
            Self::generate_cache(point_count),
            Self::generate_cache(point_count),
        );
        Self {
            cached_random: random_numbers,
            cached_x,
            cached_y,
            cached_z,
        }
    }

    pub fn get_noise(&self, point: Point3) -> f64 {
        let point = point.abs();

        let hermite_cubic = |val: f64| val * val * (3. - 2. * val);

        let u = hermite_cubic(point.x - point.x.floor());
        let v = hermite_cubic(point.y - point.y.floor());
        let w = hermite_cubic(point.z - point.z.floor());

        let i = (point.x).floor() as usize & 255;
        let j = (point.y).floor() as usize & 255;
        let k = (point.z).floor() as usize & 255;

        let mut total = 0.;

        for delta_i in 0..2 {
            for delta_j in 0..2 {
                for delta_k in 0..2 {
                    let value = self.cached_random[self.cached_x
                        [(i + delta_i) & 255]
                        ^ self.cached_y[(j + delta_j) & 255]
                        ^ self.cached_z[(k + delta_k) & 255]];

                    let (i, j, k) =
                        (delta_i as f64, delta_j as f64, delta_k as f64);
                    total += (i * u + (1. - i) * (1. - u))
                        * (j * v + (1. - j) * (1. - v))
                        * (k * w + (1. - k) * (1. - w))
                        * value;
                }
            }
        }

        total
    }

    fn generate_cache(point_count: usize) -> Vec<usize> {
        let mut numbers = (0..point_count).collect::<Vec<_>>();
        let mut rng = rng();
        numbers.shuffle(&mut rng);
        numbers
    }
}
