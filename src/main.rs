use warp::Filter;
use crate::bus::BusSvc;
use crate::slack::SlackSvc;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod bus;
pub mod db;
pub mod slack;


async fn run(bus_svc: BusSvc, slack_service: SlackSvc) -> Result<impl warp::Reply, warp::Rejection> {
    //"newbus":[{"order":2,"distanceToSc":"414","toppx":50},{"order":13,"distanceToSc":"185","toppx":710},{"order":23,"distanceToSc":"523","toppx":1310}
    let bus_fut = bus_svc.fetch_bus_info();
    let arr = bus_fut.await.unwrap();
    slack_service.notify_slack(&arr).await;
    Ok(warp::reply::json(&arr))
}



#[tokio::main]
async fn main() {
    let bus_end_point = warp::path!("bus")
            .and(warp::get())
            .and_then(|| async move {
                let bus_svc = BusSvc::new();
                let slack_service = SlackSvc::new();
                let result = run(bus_svc, slack_service).await;
                result
            });

    warp::serve(bus_end_point)
        .run(([127, 0, 0, 1], 10000))
        .await;

}
