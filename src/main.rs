use warp::Filter;
use serde_json::{Value};
pub mod bus;
pub mod db;
pub mod slack;

async fn get_bus(bus_no: &str) -> Result<impl warp::Reply, warp::Rejection> {
    let str = bus::fetch_bus_info(bus_no, 0).await.unwrap();
    let v: Value = serde_json::from_str(&str).unwrap();
    Ok(format!("{:#?}", v["newbus"]))
}

#[tokio::main]
async fn main() {
    let entry_point = warp::path!("a"/String)
            .and(warp::get())
            .and_then(|bus_no: String| async move {
                get_bus(&bus_no).await
            });

    warp::serve(entry_point)
        .run(([127, 0, 0, 1], 3030))
        .await;

}
