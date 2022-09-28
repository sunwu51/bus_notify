use reqwest::Client;
use std::env;
use crate::bus::BusInfo;
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
pub struct SlackSvc {
    url: String,
    client: Client,
}

impl SlackSvc {
    pub fn new() -> SlackSvc {
        let token = env::var("SLACK_TOKEN");
        SlackSvc { client: reqwest::Client::new(), url: format!("https://hooks.slack.com/services/T0445SPU4BB/B0445SSQWM7/{}", token.unwrap_or("x".to_string())) }
    }
    pub async fn notify_slack(&self, arr: &Vec<BusInfo>) {
        if arr.len() > 0 {
            let mut str = String::from("");

            for ele in arr {
                let time = Utc::now().with_timezone(&Shanghai);
                let time = time.format("%H:%M");
                str += &time.to_string();
                str += " ";
                str += &(ele.to_string() + "\n");
            }

            let res = self.client.post(&self.url)
                .json(&serde_json::json!({ "text": str }))
                .send()
                .await
                .unwrap()
                .text()
                .await;
            if res.is_ok() {
                println!("发送成功共{}辆车", arr.len());
            }
        } else {
            println!("没有合法车辆不发slack");
        }
    }
}
