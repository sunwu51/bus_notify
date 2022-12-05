pub mod core;
pub mod utils;
use log::LevelFilter;
use reqwest::StatusCode;
use serde_json::json;
use warp::reply::*;
use warp::Filter;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    // env_logger::init();
    use env_logger::{Builder, Target};
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
    let port : i32 = env::var("port").unwrap_or("10000".to_string()).parse().unwrap();

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

    warp::serve(bus_end_point.or(health_end_point))
        .run(([0, 0, 0, 0], port))
        .await;
}
