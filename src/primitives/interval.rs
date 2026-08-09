use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    pub fn new_from_intervals(a: &Interval, b: &Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals
        Interval {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Interval {
        rhs + self
    }
}
