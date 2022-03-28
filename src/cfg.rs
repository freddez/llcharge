use serde::{Deserialize, Serialize};
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
