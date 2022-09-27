use serde_json::{Value, Error};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusInfo {
    stations: i64,  // 还有几站
    distance: String,  // 零头距离是距离最近那一站的距离，而非距离安慧东里的总距离
}

impl BusInfo {
    pub fn to_string(&self) -> String {
        format!("距离{}站+{}米", self.stations, self.distance)
    }
}

pub struct BusSvc {
    client: Client,
}

impl BusSvc {
    pub fn new() -> BusSvc {
        BusSvc { client: reqwest::Client::new()}
    }

    // 拉取公交的实时状态
    pub async fn fetch_bus_info(&self) -> Result<Vec<BusInfo>, Error> {
        // 从笑园公交小程序的接口拉取928路公交的实时信息。
        let url = "https://job.dwmm136.cn/z_busapi/bus_shishi.php?do=luxian&op=line_view";
        let line_id = "110100015127"; // 928的内部lineId
    
        let params = [
            ("login_openid", "oKZtK5J6tn_3ouHEoaBB_y5O3ogU"),
            ("cityid", "027"),
            ("lineId", line_id),
            ("lineNo", "110000"),
            ("direction", "1"),
        ];
        // todo 把client提出去
        let client = self.client.clone();

        let res = client
            .post(url)
            .header(
                "Referer",
                "https://servicewechat.com/wxf5fe85d673cb8da2/72/page-frame.html",
            )
            .form(&params)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        // 请求接口拿到json格式的数据
        let v: Value = serde_json::from_str(&res).unwrap();
    
        // stations字段是所有的公交站列表，找到安慧北里的序号
        let mut station_no = 1;
        for station in v["stations"].as_array().unwrap() {
            match station["sn"].as_str() {
                Some("安慧北里") => break,
                _ => station_no += 1,
            }
        }
        println!("station_no: {}", station_no); // 7
    
        // newbus字段是个数组，主要是order和distanceToSc俩属性，代表了距离序号是order的站点还有distanceToSc这么多米
        let newbus = v["newbus"].as_array();
    
        // 记录还未到达安慧北里 且距离在4站以内的车辆 并返回
        let mut arr: Vec<BusInfo> = vec![];
        for ele in newbus.expect("newbus field is empty") {
            let order = ele["order"].as_i64().unwrap();
            let diff = station_no - order;
            if diff < 0 || diff > 4 {
                continue;
            }
           
            let distance = ele["distanceToSc"].as_str().unwrap();
            arr.push(BusInfo {
                distance: distance.to_string(),
                stations: diff,
            });
        }
        println!("请求到的busInfos: {:#?}", arr);
        Ok(arr)
    }    
}
