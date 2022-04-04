const DATA_SIZE: usize = 12 * 60 * 60;
const SAMPLING_RANGE: usize = 5;
const RANGE: usize = 32;

pub struct Sample {
    data: [f32; DATA_SIZE],
    index: usize,
    pub running: bool,
    ready: bool,
}
impl Default for Sample {
    fn default() -> Self {
        Sample {
            data: [0.0; DATA_SIZE],
            index: 0,
            running: true,
            ready: false,
        }
    }
}
impl Sample {
    pub fn insert(&mut self, value: f32) {
        self.data[self.index] = value;
        self.index += 1;
        if !self.ready && self.index > SAMPLING_RANGE {
            self.ready = true;
        }
        if self.index >= DATA_SIZE {
            self.index = 0;
        }
    }

    pub fn last_avg(&self) -> f32 {
        let mut i = self.index - 1;
        let mut sum = 0.0;
        let range = SAMPLING_RANGE;
        for n in 0..range {
            if i - n == 0 {
                i = DATA_SIZE;
            }
            sum += self.data[i - n];
        }
        sum / range as f32
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn range(&self) -> [f32; RANGE] {
        let mut r = [0.0; RANGE];
        let mut i = self.index - 1;
        for n in 0..RANGE {
            if i - n == 0 {
                i = DATA_SIZE;
            }
            r[n] = self.data[i - n];
        }
        r
    }
}
