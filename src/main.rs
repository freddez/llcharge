use actix_web::{get, web, App, HttpServer, Responder, Result};
use actix_web_static_files::ResourceFiles;
use once_cell::sync::Lazy;
use std::process;
use std::sync::Mutex;
use std::{thread, time};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod cfg;
mod sample;

static SAMPLE: Lazy<Mutex<sample::Sample>> = Lazy::new(|| Mutex::new(sample::Sample::default()));

#[get("/api/range/")]
async fn range() -> Result<impl Responder> {
    let range = SAMPLE.lock().unwrap().range();
    Ok(web::Json(range))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(|| -> ! {
        let cfg: cfg::MyConfig = match confy::load("bat-plug") {
            Ok(c) => c,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut device_set = cfg::RunningDevices::default();
        device_set.init(&cfg);

        let initial_pause = time::Duration::from_millis(2000);
        let some_seconds = time::Duration::from_millis(1000);
        let mut last_avg: f32;
        cfg::power_on(&cfg);
        thread::sleep(initial_pause);
        loop {
            thread::sleep(some_seconds);
            let message = cfg::get_message(&cfg);
            let power = message.get_power();
            {
                let mut sample = SAMPLE.lock().unwrap();
                sample.insert(power);
                if sample.is_ready() {
                    last_avg = sample.last_avg();
                    println!("{} avg : {}", power, last_avg);
                    if device_set.threshold_reached(&cfg, last_avg) {
                        println!("Power below {} watts", { device_set.threshold });
                        if cfg.poweroff_under_threshold {
                            cfg::power_off(&cfg);
                            process::exit(1);
                        }
                    }
                } else {
                    if cfg.verbose {
                        println!("{}", power);
                    }
                }
            }
        }
    });

    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(range)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 7000))?
    .run()
    .await
}
