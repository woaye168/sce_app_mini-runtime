//! 运行时切换架子：mini-runtime 支持多套星火运行时（编辑器/对战平台，按 api_version + 环境）。
//! 设计依据：doc/research/runtimes.md §6（api_version 是组装总钥匙 + 引擎经 update-info 按 api 分发）。
//!
//! 当前实现：
//! - EditorApi（星火编辑器-api\<N\>）：引擎 = wineditor@api_pak_version[\<N\>].wineditor，
//!   解出 version-\<N\>/ 取游戏子集；spawn 目标 = version-\<N\>/SCE（编辑器壳，游戏跑在 sceengine.dll）。
//! - TesterTest / TesterProd（星火对战平台 测试/正式）：引擎 = win 包（scegame 一体），
//!   spawn 目标 = runtime/scegame.exe。0.2.0 已有链路归入此类（结构相同，仅环境域不同）。

use std::path::{Path, PathBuf};

/// 运行时种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeKind {
    /// 星火编辑器-api\<N\>（开发调试/发布用；游戏跑在 version-\<N\>/sceengine.dll，经 SCE 壳拉起）
    EditorApi(u32),
    /// 星火对战平台 测试环境（e.production.spark.xd.com_test）
    TesterTest,
    /// 星火对战平台 正式环境（e.production.spark.xd.com）
    TesterProd,
}

impl RuntimeKind {
    /// 显示名
    pub fn display_name(&self) -> String {
        match self {
            RuntimeKind::EditorApi(n) => format!("星火编辑器-{n} 运行时"),
            RuntimeKind::TesterTest => "星火对战平台 测试环境运行时".into(),
            RuntimeKind::TesterProd => "星火对战平台 正式环境运行时".into(),
        }
    }

    /// 环境域名（影响凭证文件/update-info 域/host 分配）
    pub fn env_domain(&self) -> &'static str {
        match self {
            RuntimeKind::EditorApi(_) => "editor-pd.spark.xd.com",
            RuntimeKind::TesterTest => "e.production.spark.xd.com_test",
            RuntimeKind::TesterProd => "e.production.spark.xd.com",
        }
    }

    /// 引擎二进制包名（update-info 的二进制项）
    pub fn engine_package(&self) -> &'static str {
        match self {
            RuntimeKind::EditorApi(_) => "wineditor",
            RuntimeKind::TesterTest | RuntimeKind::TesterProd => "win",
        }
    }

    /// update-info 的 variation 参数
    pub fn update_variation(&self) -> &'static str {
        match self {
            RuntimeKind::EditorApi(_) => "windows_editor",
            RuntimeKind::TesterTest | RuntimeKind::TesterProd => "client",
        }
    }

    /// 该运行时的 api_version（编辑器=版本号；对战平台暂无概念，取编辑器 api13 对应的包集）
    pub fn api_version(&self, project_api: u32) -> u32 {
        match self {
            RuntimeKind::EditorApi(n) => *n,
            _ => project_api,
        }
    }

    /// 游戏客户端 spawn 目标（exe 路径，相对 runtime_dir）
    pub fn client_exe(&self, runtime_dir: &Path) -> PathBuf {
        match self {
            RuntimeKind::EditorApi(n) => runtime_dir.join(format!("version-{n}")).join("SCE"),
            _ => runtime_dir.join("scegame.exe"),
        }
    }

    /// 引擎是否已就绪（spawn 目标存在）
    pub fn engine_ready(&self, runtime_dir: &Path) -> bool {
        self.client_exe(runtime_dir).is_file()
    }
}

/// 在载荷目录里探测可用的游戏客户端 exe（不指定种类时用）：
/// 优先 scegame.exe（对战平台），否则扫描 version-*/SCE（编辑器，取 api 最大者）。
pub fn detect_client_exe(runtime_dir: &Path) -> Option<(RuntimeKind, PathBuf)> {
    let scegame = runtime_dir.join("scegame.exe");
    if scegame.is_file() {
        return Some((RuntimeKind::TesterTest, scegame));
    }
    let mut best: Option<(u32, PathBuf)> = None;
    for e in std::fs::read_dir(runtime_dir).ok()?.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if let Some(n) = name.strip_prefix("version-").and_then(|s| s.parse::<u32>().ok()) {
            let exe = e.path().join("SCE");
            if exe.is_file() && best.as_ref().map(|(b, _)| n > *b).unwrap_or(true) {
                best = Some((n, exe));
            }
        }
    }
    best.map(|(n, exe)| (RuntimeKind::EditorApi(n), exe))
}

/// 按项目 api_version 选默认运行时（编辑器系）
pub fn default_for_project(project_api: u32) -> RuntimeKind {
    RuntimeKind::EditorApi(project_api)
}

/// 从用户偏好/字符串解析运行时（设置页/CLI 用）
pub fn parse(s: &str, project_api: u32) -> RuntimeKind {
    match s {
        "editor" | "" => default_for_project(project_api),
        "tester_test" | "tester" => RuntimeKind::TesterTest,
        "tester_prod" => RuntimeKind::TesterProd,
        other => {
            // editor-13 / editor-2000 形式
            if let Some(n) = other.strip_prefix("editor-") {
                if let Ok(v) = n.parse::<u32>() {
                    return RuntimeKind::EditorApi(v);
                }
            }
            default_for_project(project_api)
        }
    }
}

/// 运行时的序列化键（设置持久化用）
pub fn key_of(kind: &RuntimeKind) -> String {
    match kind {
        RuntimeKind::EditorApi(n) => format!("editor-{n}"),
        RuntimeKind::TesterTest => "tester_test".into(),
        RuntimeKind::TesterProd => "tester_prod".into(),
    }
}
