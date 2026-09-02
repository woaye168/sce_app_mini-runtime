//! 载荷 0 依赖自举（0.2.0）：官方 update-info 通道查版本 → OSS 直下 7z → 解包落位。
//! 通道实证见 doc/research/scegame-reverse.md §12。
//! - update-info：`POST https://updater-pd.tapsce.cn/api/map/update-info?<参数全在 query>`（空 body，免签名）
//! - 包下载：`https://<item.url>`（OSS 公共读），md5 校验，tar.exe 解 7z（Windows 10+ 自带）
//! - 版本跟随：每次 sync 实时查 update-info = 跟随星火编辑器服务器侧更新

use anyhow::{anyhow, Result};
use md5::Digest;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// 基础包（非注册表版本管理，落 Res/<name>/<name>.pak）
const BASE_PACKAGES: &[&str] = &[
    "client_base", "startup", "fonts", "engineres", "uistyle", "refconfig",
    "shadercache_windows_ui",
];
/// 注册表版本包（落 Res/_m/...，版本取 api_pak_version.json[<api>] —— 但 0 依赖时用 update-info 返回值）
const REGISTRY_PACKAGES: &[&str] = &[
    "script", "appui", "gameui", "lite", "shadercache_editor_dxbc", "shadercache_editor_dxbc_extra",
];
/// 引擎包
const ENGINE_PACKAGE: &str = "win";

#[derive(Debug, Clone)]
pub struct UpdateItem {
    pub name: String,
    pub version: u64,
    pub url: String,
    pub md5: String,
    pub size: u64,
    /// 解压根提示（Res / Res/maps/script_libs / "" 等）
    pub path: String,
}

pub struct SyncParams {
    pub runtime_dir: PathBuf,
    pub env_domain: String,
    pub api_version: u32,
    /// 项目依赖库（libs.json 的键；空 = 只装基础运行时）
    pub project_libs: Vec<String>,
    /// 项目根（可选）：用于推导本机编辑器目录做基座资产兜底
    pub project_root: Option<PathBuf>,
    /// 可选：只打印将下载什么
    pub dry_run: bool,
    /// 运行时种类（决定引擎包与 spawn 目标；缺省编辑器-api）
    pub runtime_kind: Option<crate::core::runtimes::RuntimeKind>,
}

impl SyncParams {
    pub fn kind(&self) -> crate::core::runtimes::RuntimeKind {
        self.runtime_kind
            .unwrap_or(crate::core::runtimes::RuntimeKind::EditorApi(self.api_version))
    }
}

fn http_client() -> Result<reqwest::blocking::Client> {
    let mut builder = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300));
    if let Some(p) = crate::core::verify::proxy() {
        builder = builder.proxy(p);
    }
    Ok(builder.build()?)
}

/// 查 update-info（query-string POST，空 body，免签名——抓包/试调实证）
/// variation：client=游戏/对战平台侧；windows_editor=编辑器侧（引擎包 wineditor 走这个）
pub fn update_info(names: &[String], api_version: u32) -> Result<Vec<UpdateItem>> {
    update_info_var(names, api_version, "client")
}

pub fn update_info_var(names: &[String], api_version: u32, variation: &str) -> Result<Vec<UpdateItem>> {
    let list = names.join(";");
    let url = format!(
        "https://updater-pd.tapsce.cn/api/map/update-info?list={}&version=2&api_version={}&sample=0&suffix=client&default_part=1&variation={}",
        list, api_version, variation
    );
    let client = http_client()?;
    let resp = client
        .post(&url)
        .header("Content-Length", "0")
        .send()
        .map_err(|e| anyhow!("update-info 请求失败: {e}"))?;
    let text = resp.text()?;
    // 响应：line1 version / line2 buffer_type / line3 JSON
    let json_line = text
        .lines()
        .find(|l| l.trim_start().starts_with('{'))
        .ok_or_else(|| anyhow!("update-info 响应格式异常: {}", text.chars().take(200).collect::<String>()))?;
    let v: Value = serde_json::from_str(json_line)?;
    let items = v
        .get("items")
        .and_then(|x| x.as_array())
        .ok_or_else(|| anyhow!("update-info 缺 items"))?;
    let mut out = Vec::new();
    for it in items {
        out.push(UpdateItem {
            name: it["name"].as_str().unwrap_or_default().to_string(),
            version: it["version"].as_u64().unwrap_or(0),
            url: it["url"].as_str().unwrap_or_default().to_string(),
            md5: it["md5"].as_str().unwrap_or_default().to_string(),
            size: it["size"].as_u64().unwrap_or(0),
            path: it["path"].as_str().unwrap_or_default().to_string(),
        });
    }
    Ok(out)
}

/// 同步载荷：查询 → 下载 → 解包 → 落位 → 合成版本注册表
pub fn sync(params: &SyncParams, log: &mut dyn FnMut(String)) -> Result<()> {
    let kind = params.kind();
    let engine_pkg = kind.engine_package();
    log(format!("运行时：{}（引擎包 {engine_pkg}）", kind.display_name()));

    // ① 组包名清单（引擎包名按运行时；编辑器引擎走 wineditor + windows_editor 变体）
    let mut names: Vec<String> = REGISTRY_PACKAGES
        .iter()
        .chain(BASE_PACKAGES.iter())
        .map(|s| s.to_string())
        .collect();
    if engine_pkg != ENGINE_PACKAGE {
        names.push(engine_pkg.to_string());
    } else {
        names.push(ENGINE_PACKAGE.to_string());
    }
    for lib in &params.project_libs {
        names.push(lib.clone());
    }
    log(format!("查询 update-info（{} 个包，api_version={}）...", names.len(), params.api_version));
    // 编辑器引擎包用 windows_editor 变体查（wineditor 在该变体下）；lua 包用默认 client
    let mut items = update_info_var(&names, params.api_version, kind.update_variation())?;
    if kind.update_variation() != "client" {
        // wineditor 变体下 lua 包可能不全，补一次 client 变体查（去重）
        let lua_names: Vec<String> = REGISTRY_PACKAGES
            .iter()
            .chain(BASE_PACKAGES.iter())
            .map(|s| s.to_string())
            .chain(params.project_libs.iter().cloned())
            .collect();
        let extra = update_info_var(&lua_names, params.api_version, "client")?;
        for e in extra {
            if !items.iter().any(|x| x.name == e.name && x.version == e.version) {
                items.push(e);
            }
        }
    }
    log(format!("在线版本解析到 {} 个包", items.len()));

    let mut registry: Vec<(String, u64)> = Vec::new(); // (name, version) 已落位
    // 预统计待下载包数（进度「第 i/N 个包」用；判重逻辑与主循环一致）
    let pending_total = items
        .iter()
        .filter(|item| {
            if item.url.is_empty() {
                return false;
            }
            let Some((dest_dir, _)) = place_target(params, item) else {
                return false;
            };
            let already = if item.name == ENGINE_PACKAGE || item.name == "wineditor" {
                kind.engine_ready(&params.runtime_dir)
            } else {
                dest_dir.exists()
            };
            !already
        })
        .count();
    let mut dl_idx = 0usize;
    for item in &items {
        if item.url.is_empty() {
            continue;
        }
        let target = place_target(params, item);
        if target.is_none() {
            log(format!("[skip] {}（不在本次清单布局）", item.name));
            continue;
        }
        let (dest_dir, dest_note) = target.unwrap();
        // 引擎包以 spawn 目标存在与否判重（win→scegame.exe；wineditor→version-<api>/SCE）；包以落位目录判重
        let already = if item.name == ENGINE_PACKAGE || item.name == "wineditor" {
            kind.engine_ready(&params.runtime_dir)
        } else {
            dest_dir.exists()
        };
        if already {
            log(format!("[skip] {} v{} 已存在", item.name, item.version));
            registry.push((item.name.clone(), item.version));
            continue;
        }
        if params.dry_run {
            log(format!(
                "下载 {} v{}（{:.1}MB）...（dry-run）",
                item.name,
                item.version,
                item.size as f64 / 1048576.0
            ));
            continue;
        }
        dl_idx += 1;
        let bytes = download(item, dl_idx, pending_total, log)?;
        extract_and_place(&bytes, &dest_dir)?;
        // _m 包： pak 之外再解出散文件（引擎对 _m/maps 库与 _m/script 等按散文件探测优先）
        if REGISTRY_PACKAGES.contains(&item.name.as_str()) || item.path.starts_with("Res/maps") {
            if let Ok(Some(pak)) = find_pak(&dest_dir) {
                match upak_extract(&pak, &dest_dir) {
                    Ok(n) => log(format!("      └─ 解包散文件 {} 个（{}）", n, pak.file_name().unwrap_or_default().to_string_lossy())),
                    Err(e) => log(format!("      └─ [warn] pak 解散失败: {e}")),
                }
            }
        }
        // 引擎包：scegame → scegame.exe（PowerShell/双击兼容）
        if item.name == ENGINE_PACKAGE {
            let raw = params.runtime_dir.join("scegame");
            let exe = params.runtime_dir.join("scegame.exe");
            if raw.exists() && !exe.exists() {
                std::fs::copy(&raw, &exe)?;
            }
        }
        log(format!("[ok] {} v{} -> {}", item.name, item.version, dest_note));
        registry.push((item.name.clone(), item.version));
    }

    if params.dry_run {
        return Ok(());
    }

    // ② 合成版本注册表（api_pak_version.json 最小集 + VERSION.JSON 骨架）
    write_registry_files(params, &items)?;

    // ③ 基座资产（update-info 不分发的编辑器资产：ui/font/regular 游戏字体、characters、effect、fonts）
    sync_base_assets(params, log)?;

    // ④ 目录骨架
    for d in ["ResCache", "User", "User/debug", &format!("Update/{}/Res", params.env_domain)] {
        std::fs::create_dir_all(params.runtime_dir.join(d))?;
    }
    Ok(())
}

/// 基座资产同步：优先本机编辑器目录（按项目 tsconfig 推导），否则下载 base_assets.7z
/// （env MINI_RUNTIME_BASE_ASSETS_URL 覆盖；默认我们的 GitHub release asset）。
fn sync_base_assets(params: &SyncParams, log: &mut dyn FnMut(String)) -> Result<()> {
    let res_root = params
        .runtime_dir
        .join("Update")
        .join(&params.env_domain)
        .join("Res");
    let font_dir = res_root.join("ui").join("font").join("regular");
    if font_dir.exists() {
        log("基座资产已存在（ui/font/regular）".into());
        return Ok(());
    }

    // ① 本机编辑器兜底（仅当传了项目路径时可推导）
    if let Some(proj) = &params.project_root {
        if let Some((editor_update, editor_res)) = derive_editor_paths(proj) {
            log(format!("从本机编辑器复制基座资产: {editor_update}"));
            copy_editor_assets(&editor_update, &editor_res, params)?;
            return Ok(());
        }
    }

    // ② 自分发下载（env MINI_RUNTIME_BASE_ASSETS_URL 覆盖 = 房主分发口/公开直链直下）
    let bytes = download_base_assets(log)?;
    let tmp = std::env::temp_dir().join(format!("mr_base_{}.7z", std::process::id()));
    std::fs::write(&tmp, &bytes)?;
    let tmp_x = std::env::temp_dir().join(format!("mr_base_{}_x", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_x);
    std::fs::create_dir_all(&tmp_x)?;
    let status = crate::core::silent_command("tar")
        .args(["-xf"])
        .arg(&tmp)
        .arg("-C")
        .arg(&tmp_x)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("tar 失败: {e}"))?;
    let _ = std::fs::remove_file(&tmp);
    if !status.success() {
        return Err(anyhow!("基座资产解包失败"));
    }
    // 落位：ui/+fonts/ → Update/<env>/Res/；characters/+effect/ → Res/
    copy_tree(&tmp_x.join("ui"), &res_root.join("ui"))?;
    copy_tree(&tmp_x.join("fonts"), &res_root.join("fonts"))?;
    copy_tree(&tmp_x.join("characters"), &params.runtime_dir.join("Res").join("characters"))?;
    copy_tree(&tmp_x.join("effect"), &params.runtime_dir.join("Res").join("effect"))?;
    let _ = std::fs::remove_dir_all(&tmp_x);
    log("基座资产落位完成".into());
    Ok(())
}

/// 下载 base_assets.7z。
/// env MINI_RUNTIME_BASE_ASSETS_URL 覆盖 = 公开直链直下；
/// 默认走 GitHub API（仓库私有，asset 直链匿名 404——必须 API 定位 asset + octet-stream + token，
/// 同 bgd_sce_tools net.rs 方案）。token 来源：env MINI_RUNTIME_GITHUB_TOKEN →
/// Windows 凭据管理器 `bgd_sce_tools/github_token`（宿主应用设置里配置的 fine-grained PAT，共用）。
fn download_base_assets(log: &mut dyn FnMut(String)) -> Result<Vec<u8>> {
    let client = http_client()?;
    if let Ok(url) = std::env::var("MINI_RUNTIME_BASE_ASSETS_URL") {
        log(format!("下载基座资产（自定义 URL）: {url}"));
        return Ok(client
            .get(&url)
            .send()
            .map_err(|e| anyhow!("基座资产下载失败: {e}"))?
            .bytes()
            .map_err(|e| anyhow!("基座资产读取失败: {e}"))?
            .to_vec());
    }
    let token = std::env::var("MINI_RUNTIME_GITHUB_TOKEN")
        .ok()
        .filter(|t| !t.is_empty())
        .or_else(|| {
            keyring::Entry::new("bgd_sce_tools", "github_token")
                .ok()?
                .get_password()
                .ok()
        })
        .ok_or_else(|| {
            anyhow!(
                "基座资产在私有仓库 release 上，需要 GitHub token：设环境变量 MINI_RUNTIME_GITHUB_TOKEN，\
                 或在宿主 bgd_sce_tools 设置里配置 github_token（本应用共用同一凭据）"
            )
        })?;
    log("经 GitHub API 定位基座资产（私有仓库）".into());
    let resp = client
        .get("https://api.github.com/repos/woaye168/sce_app_mini-runtime/releases/latest")
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "sce_app_mini-runtime")
        .send()
        .map_err(|e| anyhow!("查询最新 release 失败: {e}"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!(
            "查询最新 release 失败: {status} {:.120}（私有仓库 404 多为 token 未覆盖 sce_app_mini-runtime，请在 GitHub 给 PAT 补仓库授权）",
            body
        ));
    }
    let rel: Value = resp
        .json()
        .map_err(|e| anyhow!("release 响应解析失败: {e}"))?;
    let asset_url = rel
        .get("assets")
        .and_then(|a| a.as_array())
        .and_then(|a| {
            a.iter()
                .find(|x| x.get("name").and_then(|n| n.as_str()) == Some("base_assets.7z"))
        })
        .and_then(|x| x.get("url"))
        .and_then(|u| u.as_str())
        .map(String::from)
        .ok_or_else(|| anyhow!("最新 release 里没有 base_assets.7z"))?;
    log("下载基座资产: base_assets.7z".into());
    let resp = client
        .get(&asset_url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/octet-stream")
        .header("User-Agent", "sce_app_mini-runtime")
        .send()
        .map_err(|e| anyhow!("基座资产下载失败: {e}"))?;
    if !resp.status().is_success() {
        return Err(anyhow!("基座资产下载失败: {}", resp.status()));
    }
    let bytes = resp
        .bytes()
        .map_err(|e| anyhow!("基座资产读取失败: {e}"))?;
    Ok(bytes.to_vec())
}

/// 从项目 script/tsconfig.json 的 typeRoots 推导编辑器 update/Res 目录
/// （同 sce_app_editor-patch locate 链：.../update/<env>/res/_m/... 前缀）
fn derive_editor_paths(project: &Path) -> Option<(String, String)> {
    let content = std::fs::read_to_string(project.join("script").join("tsconfig.json")).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    let roots = v.pointer("/compilerOptions/typeRoots")?.as_array()?;
    for r in roots {
        let s = r.as_str()?.replace('\\', "/");
        // typeRoots 形如 D:/sce_online/Update/editor-pd.../Res/_m/...（大小写不敏感匹配）
        let lower = s.to_lowercase();
        if let Some(idx) = lower.find("/res/_m/") {
            let update_dir = s[..idx].to_string();
            let editor_root = Path::new(&update_dir)
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.to_string_lossy().replace('\\', "/"))?;
            let res = format!("{editor_root}/Res");
            if Path::new(&update_dir).exists() && Path::new(&res).exists() {
                return Some((update_dir, res));
            }
        }
    }
    None
}

/// 从本机编辑器目录复制基座资产
fn copy_editor_assets(editor_update: &str, editor_res: &str, params: &SyncParams) -> Result<()> {
    let res_root = params
        .runtime_dir
        .join("Update")
        .join(&params.env_domain)
        .join("Res");
    let pairs = [
        (format!("{editor_update}/res/ui/font/regular"), res_root.join("ui/font/regular")),
        (format!("{editor_update}/res/fonts"), res_root.join("fonts")),
        (format!("{editor_res}/characters"), params.runtime_dir.join("Res/characters")),
        (format!("{editor_res}/effect"), params.runtime_dir.join("Res/effect")),
    ];
    for (src, dst) in pairs {
        if Path::new(&src).exists() {
            std::fs::create_dir_all(&dst)?;
            copy_tree(Path::new(&src), &dst)?;
        }
    }
    Ok(())
}

/// 计算包落位目录（None = 不认识，跳过）
fn place_target(params: &SyncParams, item: &UpdateItem) -> Option<(PathBuf, String)> {
    let res_root = params
        .runtime_dir
        .join("Update")
        .join(&params.env_domain)
        .join("Res");
    let name = item.name.as_str();
    // 引擎包：内容即引擎根，解压到载荷根
    // - win（对战平台）：内容 = Win 根（scegame/dll/embedded_packages/update 骨架）
    // - wineditor（编辑器）：内容 = version-<api>/ + launcher_update/ + update/<env>/res 基座
    if name == ENGINE_PACKAGE || name == "wineditor" {
        return Some((params.runtime_dir.clone(), "runtime 根（引擎）".into()));
    }
    // 注册表版本包 → _m 形态（_m/<sub>/<ver>/<name>/<name>.pak）
    if REGISTRY_PACKAGES.contains(&name) {
        let sub = registry_m_subpath(name);
        return Some((
            res_root
                .join("_m")
                .join(&sub)
                .join(item.version.to_string())
                .join(name),
            format!("_m/{}/{}/{}", sub, item.version, name),
        ));
    }
    // 依赖库（path=Res/maps[/sub]）→ _m/maps[/sub]/<name>/<ver>/<name>
    if item.path.starts_with("Res/maps") {
        let sub = item.path.trim_start_matches("Res/"); // "maps/..."
        return Some((
            res_root
                .join("_m")
                .join(sub)
                .join(name)
                .join(item.version.to_string())
                .join(name),
            format!("_m/{}/{}/{}/{}", sub, name, item.version, name),
        ));
    }
    // 基础包 → Res/<name>/（pak 落其中）
    if BASE_PACKAGES.contains(&name) {
        return Some((res_root.join(name), format!("Res/{name}")));
    }
    None
}

/// 注册表包的 _m 子路径（script 直接在 _m 下；maps 库在 _m/maps 下）
fn registry_m_subpath(name: &str) -> String {
    match name {
        "script" | "appui" | "gameui" | "lite" | "shadercache_editor_dxbc"
        | "shadercache_editor_dxbc_extra" => name.to_string(),
        _ => format!("maps/{name}"),
    }
}

/// 下载 + md5 校验（流式读取，周期性回报进度：已下MB/总MB + 第 i/N 个包）
fn download(item: &UpdateItem, idx: usize, total: usize, log: &mut dyn FnMut(String)) -> Result<Vec<u8>> {
    use std::io::Read;
    let url = format!("https://{}", item.url);
    let client = http_client()?;
    let mut resp = client
        .get(&url)
        .send()
        .map_err(|e| anyhow!("下载失败 {}: {e}", item.name))?;
    let total_bytes = resp.content_length().unwrap_or(item.size);
    let mut buf: Vec<u8> = Vec::with_capacity(total_bytes as usize);
    let mut chunk = [0u8; 256 * 1024];
    // 首条进度立即打（让用户马上看到「第 i/N 个包」），之后每 ~800ms 刷一次
    let mut last = std::time::Instant::now() - std::time::Duration::from_secs(1);
    loop {
        let n = resp
            .read(&mut chunk)
            .map_err(|e| anyhow!("读取失败 {}: {e}", item.name))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if last.elapsed().as_millis() >= 800 {
            log(format!(
                "下载 {} v{}（{:.1}MB/{:.1}MB），正在下载 第 {}/{} 个包",
                item.name,
                item.version,
                buf.len() as f64 / 1048576.0,
                total_bytes as f64 / 1048576.0,
                idx,
                total
            ));
            last = std::time::Instant::now();
        }
    }
    log(format!(
        "下载 {} v{}（{:.1}MB/{:.1}MB）完成，第 {}/{} 个包",
        item.name,
        item.version,
        buf.len() as f64 / 1048576.0,
        total_bytes as f64 / 1048576.0,
        idx,
        total
    ));
    if !item.md5.is_empty() {
        let got = format!("{:x}", md5::Md5::digest(&buf));
        if got != item.md5.to_lowercase() {
            return Err(anyhow!("md5 校验失败 {}: 期望 {} 实得 {}", item.name, item.md5, got));
        }
    }
    Ok(buf)
}

/// 解 7z 到目标目录（系统 tar.exe；包内容 = 单个 <Name>.pak 或 Win 根文件集）
/// 部分包在线上是 TNND 加密（magic + XOR CREATEEASY）——先按 magic 识别解密再解包。
fn extract_and_place(bytes: &[u8], dest_dir: &Path) -> Result<()> {
    let plain;
    let data: &[u8] = if bytes.starts_with(b"TNND") {
        let key = b"CREATEEASY";
        plain = bytes[4..]
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ key[i % key.len()])
            .collect::<Vec<u8>>();
        &plain
    } else {
        bytes
    };
    let tmp = std::env::temp_dir().join(format!("mr_pak_{}.7z", std::process::id()));
    std::fs::write(&tmp, data)?;
    let tmp_extract = std::env::temp_dir().join(format!("mr_pak_{}_x", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_extract);
    std::fs::create_dir_all(&tmp_extract)?;
    let status = crate::core::silent_command("tar")
        .args(["-xf"])
        .arg(&tmp)
        .arg("-C")
        .arg(&tmp_extract)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("调用 tar 失败: {e}"))?;
    let _ = std::fs::remove_file(&tmp);
    if !status.success() {
        return Err(anyhow!("tar 解包失败（{status}）"));
    }
    std::fs::create_dir_all(dest_dir)?;
    copy_tree(&tmp_extract, dest_dir)?;
    let _ = std::fs::remove_dir_all(&tmp_extract);
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let s = entry.path();
        let d = dst.join(entry.file_name());
        if s.is_dir() {
            std::fs::create_dir_all(&d)?;
            copy_tree(&s, &d)?;
        } else {
            std::fs::copy(&s, &d)?;
        }
    }
    Ok(())
}

/// 合成 api_pak_version.json（最小可用集）+ VERSION.JSON 骨架
/// 规则：凡 _m 落位的包（注册表包 + path 为 Res/maps 的库 + 依赖自动展开项）都进
/// #package_path 与 <api> 表——global_default/spark_core 等隐式基础库也必须登记，
/// 否则引擎回退到 Res/maps/<lib> 找不到（0.2.0 实测）。
fn write_registry_files(params: &SyncParams, items: &[UpdateItem]) -> Result<()> {
    let update_root = params.runtime_dir.join("Update").join(&params.env_domain);
    std::fs::create_dir_all(&update_root)?;

    let mut package_path = serde_json::Map::new();
    let mut api_table = serde_json::Map::new();
    for item in items {
        let name = item.name.as_str();
        if name == ENGINE_PACKAGE {
            continue;
        }
        let mpath = if REGISTRY_PACKAGES.contains(&name) {
            format!("Res/_m/{}", registry_m_subpath(name))
        } else if item.path.starts_with("Res/maps") {
            format!("Res/_m/{}/{}", item.path.trim_start_matches("Res/"), name)
        } else {
            continue; // 基础 Res/ 包不进注册表
        };
        package_path.insert(name.to_string(), Value::String(mpath));
        api_table.insert(name.to_string(), Value::from(item.version));
    }
    let mut root = serde_json::Map::new();
    root.insert("#package_path".into(), Value::Object(package_path));
    root.insert(params.api_version.to_string(), Value::Object(api_table));
    std::fs::write(
        update_root.join("api_pak_version.json"),
        serde_json::to_string_pretty(&Value::Object(root))?,
    )?;

    std::fs::write(
        update_root.join("VERSION.JSON"),
        r##"{"#@#format_version":{"time":"2024_6","version":1}}"##,
    )?;
    std::fs::write(update_root.join("map_pak_version.json"), "{}")?;
    Ok(())
}

/// 找目录下第一个 .pak
fn find_pak(dir: &Path) -> Result<Option<PathBuf>> {
    for entry in std::fs::read_dir(dir)? {
        let p = entry?.path();
        if p.extension().map(|e| e == "pak").unwrap_or(false) {
            return Ok(Some(p));
        }
    }
    Ok(None)
}

/// UPAK 解包（SCE 变体：头 "UPAK" + u32 条目数 + u32 总校验；
/// 条目 = 名字\0 + u32 offset + u32 size + u32 条目校验；内容明文）
fn upak_extract(pak: &Path, dest_dir: &Path) -> Result<usize> {
    let data = std::fs::read(pak)?;
    if !data.starts_with(b"UPAK") {
        return Err(anyhow!("非 UPAK 格式: {}", pak.display()));
    }
    let count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let mut pos = 12usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let end = data[pos..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| anyhow!("条目名越界"))?
            + pos;
        let name = std::str::from_utf8(&data[pos..end])
            .map_err(|_| anyhow!("条目名非 utf8"))?
            .to_string();
        pos = end + 1;
        let off = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        let size =
            u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]) as usize;
        pos += 12; // offset + size + 条目校验
        entries.push((name, off, size));
    }
    let mut written = 0usize;
    for (name, off, size) in &entries {
        if off + size > data.len() {
            return Err(anyhow!("条目越界: {name}"));
        }
        let out = dest_dir.join(name);
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&out, &data[*off..*off + *size])?;
        written += 1;
    }
    Ok(written)
}
