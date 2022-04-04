use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub struct RunningDevices {
    devices: HashSet<usize>,
    pub threshold: f32,
}
impl ::std::default::Default for RunningDevices {
    fn default() -> Self {
        Self {
            devices: HashSet::new(),
            threshold: -1.0,
        }
    }
}
impl RunningDevices {
    pub fn init(&mut self, config: &MyConfig) {
        let mut i: usize = 0;
        let n = config.devices.len();
        while i < n {
            self.devices.insert(i);
            i = i + 1
        }
    }

    pub fn threshold_reached(&mut self, config: &MyConfig, power: f32) -> bool {
        if self.threshold == -1.0 {
            self.devices.retain(|i| {
                let device = &config.devices[*i];
                power < device.max_power && power > device.min_power
            });
            let n = self.devices.len();
            if n == 1 {
                for i in self.devices.iter() {
                    let device = &config.devices[*i];
                    println!("Device {} identified", device.name);
                    if config.verbose {
                        println!(
                            "{}<{} {}>{}",
                            power, device.max_power, power, device.min_power
                        );
                    }
                    self.threshold = device.power_threshold;
                }
            } else if n == 0 {
                panic!("No device matching current power");
            }
        }
        self.threshold != -1.0 && power < self.threshold
    }
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub max_power: f32,
    pub min_power: f32,
    pub power_threshold: f32,
}
#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub status_url: String,
    pub power_on_url: String,
    pub power_off_url: String,
    pub poweroff_under_threshold: bool,
    pub verbose: bool,
    pub devices: Vec<Device>,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            status_url: "http://192.168.0.105/status/".into(),
            power_on_url: "http://192.168.0.105/relay/0?turn=on".into(),
            power_off_url: "http://192.168.0.105/relay/0?turn=off".into(),
            poweroff_under_threshold: false.into(),
            verbose: true.into(),
            devices: vec![
                Device {
                    name: "Super Soco CPX".into(),
                    max_power: 1100.0,
                    min_power: 650.0,
                    power_threshold: 700.0,
                },
                Device {
                    name: "OnePlus 5".into(),
                    max_power: 12.0,
                    min_power: 3.0,
                    power_threshold: 8.0,
                },
            ],
        }
    }
}

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
pub fn power_on(config: &MyConfig) {
    match ureq::get(&config.power_on_url).call() {
        Ok(a) => a,
        Err(error) => panic!("Problem activating plug: {:?}", error),
    };
    println!("Power ON");
}
pub fn power_off(config: &MyConfig) {
    match ureq::get(&config.power_off_url).call() {
        Ok(a) => a,
        Err(error) => panic!("Power-off plug error: {:?}", error),
    };
    println!("Power OFF");
}

pub fn get_message(config: &MyConfig) -> Message {
    match match ureq::get(&config.status_url).call() {
        Ok(a) => a,
        Err(error) => panic!("Problem calling plug: {:?}", error),
    }
    .into_json()
    {
        Ok(a) => a,
        Err(error) => panic!("Problem parsing plug response: {:?}", error),
    }
}
