pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            min: f32::MIN,
            max: f32::MAX,
        }
    }

    pub fn with(_min: f32, _max: f32) -> Interval {
        Interval {
            min: _min,
            max: _max,
        }
    }

    pub fn contains(&self, val: f32) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f32) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        return x;
    }
}

pub static EMPTY: Interval = Interval {
    min: f32::MAX,
    max: f32::MIN,
};

pub static UNIVERSE: Interval = Interval {
    min: f32::MIN,
    max: f32::MAX,
};
