use reqwest::Client;
use std::{env, fmt::Error, io::ErrorKind};

// 声明全局的静态变量
static BUS_URL: &str = "https://job.dwmm136.cn/z_busapi/bus_shishi.php?do=luxian&op=line_view";
lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
    static ref TOKEN: String = env::var("SLACK_TOKEN").unwrap_or("".to_string());
    static ref SLACK_URL: String = format!(
        "https://hooks.slack.com/services/T0445SPU4BB/B0445SSQWM7/{}",
        &TOKEN.to_string()
    );
}

// 获取公交信息，返回json字符串
pub async fn get_bus_info() -> Result<String, Box<dyn std::error::Error>> {
    let line_id = "110100015127"; // 928的内部lineId
    let params = [
        ("login_openid", "oKZtK5J6tn_3ouHEoaBB_y5O3ogU"),
        ("cityid", "027"),
        ("lineId", line_id),
        ("lineNo", "110000"),
        ("direction", "1"),
    ];
    let res = HTTP_CLIENT
        .post(BUS_URL)
        .header(
            "Referer",
            "https://servicewechat.com/wxf5fe85d673cb8da2/72/page-frame.html",
        )
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

// 向slack发送消息
pub async fn send_message(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = HTTP_CLIENT
        .post(&SLACK_URL[..])
        .json(&serde_json::json!({ "text": msg }))
        .send()
        .await?
        .status()
        ;
    if status.is_success() {
        return Ok(());
    }
    Err(Box::new(std::io::Error::new(ErrorKind::Other, "slack: msg send failed")))
}
