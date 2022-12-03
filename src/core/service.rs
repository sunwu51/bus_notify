use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use serde_json::Value;

use crate::utils::http_client;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusInfo {
    stations: i32,    // 还有几站
    distance: String, // 零头距离是距离最近那一站的距离，而非距离安慧东里的总距离
}

impl BusInfo {
    pub fn to_string(&self) -> String {
        format!("距离{}站+{}米", self.stations, self.distance)
    }
}

pub async fn fetch_and_notify() -> Result<Vec<BusInfo>, Box<dyn std::error::Error>> {
    // 获取即将到站的车辆信息
    let bus_info = fetch_bus_info().await?;

    if bus_info.len() > 0 {
        let mut msg = String::from("");

        for ele in bus_info.iter() {
            let time = Utc::now().with_timezone(&Shanghai);
            let time = time.format("%H:%M");
            msg += &time.to_string();
            msg += " ";
            msg += &(ele.to_string() + "\n");
        }

        let slack_res = http_client::send_message(&msg).await?;
        info!("{:#?}", slack_res);
    }

    Ok(bus_info)
}

async fn fetch_bus_info() -> Result<Vec<BusInfo>, Box<dyn std::error::Error>> {
    let info_str = http_client::get_bus_info().await?;
    // -- json反序列化成businfo结构体 --
    let v: Value = serde_json::from_str(&info_str)?;

    // 1 从json里先找到安慧北里的序号
    let station_no = get_station_no("安慧北里", &v)?;

    // 2 newbus字段是个数组，主要是order和distanceToSc俩属性，代表了距离序号是order的站点还有distanceToSc这么多米
    // 记录还未到达安慧北里 且距离在4站以内的车辆 并返回
    let newbus = v["newbus"].as_array().unwrap();
    let mut arr: Vec<BusInfo> = vec![];
    for ele in newbus {
        let order = ele["order"].as_i64().unwrap();
        let diff = station_no - order as i32;
        if diff < 0 || diff > 5 {
            continue;
        }
        let distance = ele["distanceToSc"].as_str().unwrap();
        arr.push(BusInfo {
            distance: distance.to_string(),
            stations: diff,
        });
    }
    info!("请求到的busInfos: {:#?}", arr);
    Ok(arr)
}

fn get_station_no(station_name: &str, v: &Value) -> Result<i32, Box<dyn std::error::Error>> {
    // json中stations字段是所有的公交站列表，找到安慧北里的序号
    let mut station_no = 1;
    for station in v["stations"].as_array().unwrap() {
        match station["sn"].as_str() {
            Some(name) => {
                if name == station_name {
                    break
                }
                station_no += 1
            },
            _ => station_no += 1,
        }
    }
    Ok(station_no)
}
