//! 实证 wineditor@<api13 版本> 经 update-info 可下载，且含编辑器引擎（version-13 的 sceengine.dll/SCE）。
//! 这是「星火编辑器-13 运行时自举」的地基验证。用法：cargo run --release --example probe_wineditor --

fn main() {
    let client = reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7897").unwrap())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();
    // ① 读 api_pak_version.json[13].wineditor = 目标版本
    let reg = std::fs::read_to_string(r"D:\sce_online\update\editor-pd.spark.xd.com\api_pak_version.json").unwrap();
    let reg: serde_json::Value = serde_json::from_str(&reg).unwrap();
    let target = reg["13"]["wineditor"].as_u64().unwrap();
    println!("api13 目标 wineditor 版本 = {target}");

    // ② update-info 查 wineditor（variation=windows_editor）
    let url = "https://updater-pd.tapsce.cn/api/map/update-info?list=wineditor&version=2&api_version=13&sample=0&suffix=client&default_part=1&variation=windows_editor";
    let resp = client.post(url).header("Content-Length", "0").send().unwrap();
    let text = resp.text().unwrap();
    let jl = text.lines().find(|l| l.trim_start().starts_with('{')).unwrap();
    let v: serde_json::Value = serde_json::from_str(jl).unwrap();
    for it in v["items"].as_array().unwrap() {
        println!(
            "name={} version={} size={}MB path={} variation={} url={}",
            it["name"].as_str().unwrap_or(""),
            it["version"],
            it["size"].as_u64().unwrap_or(0) / 1048576,
            it["path"].as_str().unwrap_or(""),
            it["variation"].as_str().unwrap_or(""),
            it["url"].as_str().unwrap_or(""),
        );
    }
}
