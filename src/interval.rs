use derive_more::Constructor;

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    #[cfg(false)]
    pub const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    #[cfg(false)]
    pub fn contains(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    #[cfg(false)]
    pub fn expand(&self, delta: f32) -> Self {
        #[cfg(false)]
        let padding = delta / 2.;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn enclose(&self, interval_two: Self) -> Self {
        let minimum = self.min.min(interval_two.min);
        let maximum = self.max.max(interval_two.max);
        Interval::new(minimum, maximum)
    }
}
