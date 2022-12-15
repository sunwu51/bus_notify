pub mod core;
pub mod utils;
use chrono::Timelike;
use job_scheduler::Job;
use chrono_tz::Asia::Shanghai;
use chrono::Utc;
use job_scheduler::JobScheduler;
use log::LevelFilter;
use reqwest::StatusCode;
use serde_json::json;
use warp::reply::*;
use warp::Filter;
use std::env;
use std::time::Duration;
use std::thread;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;


lazy_static!{
    static ref PORT : u16 = env::var("port").unwrap_or("10000".to_string()).parse().unwrap();
}

#[tokio::main]
async fn main() {
    // env_logger::init();
    use env_logger::{Builder, Target};
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
    

    let bus_end_point = warp::path!("bus").and(warp::get()).then(|| async move {
        let bus_info = core::service::fetch_and_notify().await;
        if bus_info.is_ok() {
            return with_status(json(&bus_info.unwrap()), StatusCode::OK);
        } else {
            return with_status(
                json(&json!({ "msg": format!("{:?}", bus_info.unwrap_err()) })),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    });

    let health_end_point = warp::path!("healthz").map(|| "ok");

    tokio::spawn(cron());

    warp::serve(bus_end_point.or(health_end_point))
        .run(([0, 0, 0, 0], *PORT))
        .await;
}


async fn cron() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("0 * * * * *".parse().unwrap(), || {
        let t = Utc::now().with_timezone(&Shanghai);
        if t.hour() == 8 {
            let f = core::service::fetch_and_notify();
            tokio::spawn(async move{
                let res = f.await.unwrap();
            });
        }
    }));

    loop {
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }
}