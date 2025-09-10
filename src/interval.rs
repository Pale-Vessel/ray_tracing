use derive_more::Constructor;

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    pub fn enclose(&self, interval_two: Self) -> Self {
        let minimum = f32::min(self.min, interval_two.min);
        let maximum = f32::max(self.max, interval_two.max);
        Interval::new(minimum, maximum)
    }
}
