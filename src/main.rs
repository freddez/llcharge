use serde::Deserialize;
use std::process;
use std::{thread, time};

#[derive(Deserialize)]
struct Meter {
    power: f32,
}
#[derive(Deserialize)]
struct Message {
    meters: Vec<Meter>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let power_threshold = 0.6;
    let initial_pause = time::Duration::from_millis(8000);
    let some_seconds = time::Duration::from_millis(1000);
    ureq::get("http://192.168.0.105/relay/0?turn=on").call()?;
    println!("Power ON");
    thread::sleep(initial_pause);
    loop {
        thread::sleep(some_seconds);
        let message: Message = ureq::get("http://192.168.0.105/status/")
            .call()?
            .into_json()?;
        let power = message.meters[0].power;
        println!("{}", power);
        if power < power_threshold {
            println!("Down to {} watts, Power OFF", { power_threshold });
            ureq::get("http://192.168.0.105/relay/0?turn=off").call()?;
            process::exit(1);
        }
    }
}
