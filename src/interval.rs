use derive_more::Constructor;

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn surrounds(self, value: f32) -> bool {
        self.min < value && value < self.max
    }

    pub fn clamp(self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }
}
