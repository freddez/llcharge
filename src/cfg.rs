use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub max_power: f32,
    pub min_power: f32,
    pub power_threshold: f32,
}

// pub struct RunningDevices {
//     pub devices: HashMap<String, &Device>,
//     pub threshold: f32,
// }
// impl ::std::default::Default for RunningDevices {
//     fn default() -> Self {
//         Self {
//             devices: HashMap::new(),
//             threshold: -1.0,
//         }
//     }
// }
// impl RunningDevices {
//     pub fn device_identified(&self) -> bool {
//         self.threshold != -1.0
//     }
//     pub fn filter_devices(&self, power: f32) {}
// }

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub status_url: String,
    pub power_on_url: String,
    pub power_off_url: String,
    pub devices: Vec<Device>,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            status_url: "http://192.168.0.105/status/".into(),
            power_on_url: "http://192.168.0.105/relay/0?turn=on".into(),
            power_off_url: "http://192.168.0.105/relay/0?turn=off".into(),
            devices: vec![
                Device {
                    name: "Super Soco".into(),
                    max_power: 1100.0,
                    min_power: 650.0,
                    power_threshold: 700.0,
                },
                Device {
                    name: "OnePlus 5".into(),
                    max_power: 12.0,
                    min_power: 2.0,
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
