use std::io::Error;

// 拉取公交的实时状态，并将接口返回的文本直接返回
pub async fn fetch_bus_info(bus_no: &str, direction: u8) -> Result<String, Error> {
    let url = "https://job.dwmm136.cn/z_busapi/bus_shishi.php?do=luxian&op=line_view";
    let params = [
        ("login_openid", "oKZtK5J6tn_3ouHEoaBB_y5O3ogU"), ("cityid", "027"),
        ("lineId", "110100013999"), ("lineNo", "110000"),
        ("name", bus_no), ("direction", &direction.to_string()[..]),   
    ];
    println!("url {}", url);
    println!("params {:?}", params);

    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("Referer", "https://servicewechat.com/wxf5fe85d673cb8da2/72/page-frame.html")
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        ;
    Ok(res)
}