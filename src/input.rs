use serde::Deserialize;

#[derive(Deserialize)]
pub struct Meter {
    power: f32,
}

#[derive(Deserialize)]
pub struct Message {
    meters: Vec<Meter>,
}

impl Message {
    pub fn get_power(&self) -> f32 {
        self.meters[0].power
    }
}
