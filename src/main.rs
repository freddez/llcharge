use std::process;
use std::{thread, time};
mod input;
mod sample;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let power_threshold = 0.6;
    let initial_pause = time::Duration::from_millis(2000);
    let some_seconds = time::Duration::from_millis(1000);
    let mut sample = sample::Sample::default();
    ureq::get("http://192.168.0.105/relay/0?turn=on").call()?;
    println!("Power ON");
    thread::sleep(initial_pause);
    loop {
        thread::sleep(some_seconds);
        let message: input::Message = ureq::get("http://192.168.0.105/status/")
            .call()?
            .into_json()?;
        let power = message.get_power();
        sample.insert(power);
        if sample.is_ready() {
            println!("{} avg : {}", power, sample.last_avg());
            if power < power_threshold {
                println!("Down to {} watts, Power OFF", { power_threshold });
                // ureq::get("http://192.168.0.105/relay/0?turn=off").call()?;
                process::exit(1);
            }
        } else {
            println!("{}", power);
        }
    }
}
