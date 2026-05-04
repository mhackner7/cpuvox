use fastrand::Rng;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const DEFAULT: Interval = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
    pub const EMPTY: Interval = Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
    pub const BASE: Interval = Interval { min: 0.0, max: 1.0 };
    pub const UNIT: Interval = Interval {
        min: -1.0,
        max: 1.0,
    };

    #[inline(always)]
    pub const fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    #[inline(always)]
    pub fn rand(self, rng: &mut Rng) -> f32 {
        self.min + rng.f32() * self.size()
    }

    #[inline(always)]
    pub fn size(self) -> f32 {
        self.max - self.min
    }

    #[inline(always)]
    pub fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline(always)]
    pub fn surrounds(self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[inline(always)]
    pub fn clamp(self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}
