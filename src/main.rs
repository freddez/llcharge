use actix_web::{get, web, App, HttpServer, Responder, Result};
use actix_web_static_files::ResourceFiles;
use once_cell::sync::Lazy;
use std::process;
use std::sync::Mutex;
use std::{thread, time};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod cfg;
mod input;
mod sample;

static SAMPLE: Lazy<Mutex<sample::Sample>> = Lazy::new(|| Mutex::new(sample::Sample::default()));

#[get("/api")]
async fn index() -> Result<impl Responder> {
    let range = SAMPLE.lock().unwrap().range();
    Ok(web::Json(range))
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

    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(index)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 7000))?
    .run()
    .await
}
