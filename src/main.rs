use actix_web::{get, App, HttpServer};
use once_cell::sync::Lazy;
use std::process;
use std::sync::Mutex;
use std::{thread, time};
mod cfg;
mod input;
mod sample;

static SAMPLE: Lazy<Mutex<sample::Sample>> = Lazy::new(|| Mutex::new(sample::Sample::default()));

#[get("/")]
async fn index() -> String {
    let last_avg = SAMPLE.lock().unwrap().last_avg();
    format!("average of {last_avg}W")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(|| {
        let cfg: cfg::MyConfig = match confy::load("bat-plug") {
            Ok(c) => c,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let power_threshold = 0.6;

        let initial_pause = time::Duration::from_millis(2000);
        let some_seconds = time::Duration::from_millis(1000);
        match ureq::get(&cfg.power_on_url).call() {
            Ok(a) => a,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        println!("Power ON");
        thread::sleep(initial_pause);
        loop {
            thread::sleep(some_seconds);
            let message: input::Message = match match ureq::get(&cfg.status_url).call() {
                Ok(a) => a,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            }
            .into_json()
            {
                Ok(a) => a,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            let power = message.get_power();
            SAMPLE.lock().unwrap().insert(power);
            if SAMPLE.lock().unwrap().is_ready() {
                println!("{} avg : {}", power, SAMPLE.lock().unwrap().last_avg());
                if power < power_threshold {
                    println!("Down to {} watts, Power OFF", { power_threshold });
                    // ureq::get(&cfg.power_off_url).call()?;
                    process::exit(1);
                }
            } else {
                println!("{}", power);
            }
        }
    });

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
