use actix_web::{get, web, App, HttpServer, Responder, Result};
use actix_web_static_files::ResourceFiles;
use clap::Parser;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::{env, thread, time};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Configuration path
    #[clap(short = 'c', parse(from_os_str), value_name = "config_path", value_hint = clap::ValueHint::DirPath)]
    config_path: Option<std::path::PathBuf>,
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod cfg;
mod sample;

static SAMPLE: Lazy<Mutex<sample::Sample>> = Lazy::new(|| Mutex::new(sample::Sample::default()));

fn run(cfg: &cfg::MyConfig) {
    let mut device_set = cfg::RunningDevices::default();
    device_set.init(&cfg);

    let some_seconds = time::Duration::from_millis(5000);
    let mut last_avg: f32;
    loop {
        thread::sleep(some_seconds);
        let message = cfg::get_message(&cfg);
        let power = message.get_power();
        {
            let mut sample = SAMPLE.lock().unwrap();
            sample.insert(power);
            if sample.is_ready() {
                last_avg = sample.last_avg();
                println!("{}W avg : {}W", power, last_avg);
                if device_set.threshold_reached(&cfg, last_avg) {
                    println!("Power below {}W", { device_set.threshold });
                    if cfg.power_off_under_threshold {
                        cfg::power_off(&cfg);
                        sample.running = false;
                        return;
                    }
                } else if device_set.num_candidates() == 0 {
                    println!("Unknown device");
                    cfg::power_off(&cfg);
                    sample.running = false;
                    return;
                }
            } else {
                if cfg.verbose {
                    println!("{}W", power);
                }
            }
        }
    }
}

#[get("/api/range/")]
async fn range() -> Result<impl Responder> {
    let range = SAMPLE.lock().unwrap().range();
    Ok(web::Json(range))
}

#[get("/api/activate/")]
async fn activate() -> Result<impl Responder> {
    let mut sample = SAMPLE.lock().unwrap();
    let initial_pause = time::Duration::from_millis(10000);
    thread::sleep(initial_pause);
    sample.start();
    Ok(web::Json("OK"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cfg: cfg::MyConfig = if let Some(config_path) = args.config_path.as_deref() {
        match confy::load_path(config_path) {
            Ok(c) => c,
            Err(error) => panic!("Problem opening {} {:?}", config_path.display(), error),
        }
    } else {
        match confy::load("llcharge") {
            Ok(c) => c,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    };
    let cfg2 = cfg.clone();
    thread::spawn(move || {
        if cfg2.power_on_on_startup {
            cfg::power_on(&cfg2);
        }
        let initial_pause = time::Duration::from_millis(10000);
        loop {
            thread::sleep(initial_pause);
            if SAMPLE.lock().unwrap().running {
                run(&cfg2);
            }
            println!("waiting...");
        }
    });
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(activate)
            .service(range)
            .service(ResourceFiles::new("/", generated))
    })
    .bind((cfg.ws_listen_addr, cfg.ws_port))?
    .run()
    .await
}
