//! 探项目依赖库（test_res002 libs.json 9 个）在哪个 variation 下能被 update-info 返回。
//! 用法：cargo run --release --example probe_libs --

fn main() {
    let client = reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7897").unwrap())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();
    let libs = ["lib_control","lib_game_options","lib_common_ai","defaultui","default_units_ts","smallcard_inventory","lib_common_sounds","smallcard_get_items","smallcard_mail"];
    let list = libs.join(";");
    for variation in ["client", "windows_editor", "server"] {
        let url = format!(
            "https://updater-pd.tapsce.cn/api/map/update-info?list={list}&version=2&api_version=13&sample=0&suffix=client&default_part=1&variation={variation}"
        );
        let resp = client.post(&url).header("Content-Length", "0").send();
        match resp {
            Ok(r) => {
                let text = r.text().unwrap_or_default();
                let jl = text.lines().find(|l| l.trim_start().starts_with('{')).unwrap_or("{}");
                let v: serde_json::Value = serde_json::from_str(jl).unwrap_or_default();
                let n = v["items"].as_array().map(|a| a.len()).unwrap_or(0);
                println!("variation={variation} => items={n}");
                if let Some(arr) = v["items"].as_array() {
                    for it in arr {
                        println!("    {} v{} path={} variation={}", it["name"].as_str().unwrap_or(""), it["version"], it["path"].as_str().unwrap_or(""), it["variation"].as_str().unwrap_or(""));
                    }
                }
            }
            Err(e) => println!("variation={variation} => ERR {e}"),
        }
    }
}
