//! 业务核心模块
pub mod auth;
pub mod capture;
pub mod debug;
pub mod host;
pub mod local_host;
pub mod locate;
pub mod login;
pub mod login_state;
pub mod payload;
pub mod runtimes;
pub mod staging;
pub mod verify;
pub mod zcompress;

/// 路径显示统一为正斜杠（用户约定：本应用所有路径展示一律用 /）
pub fn disp(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

/// Windows 下不弹黑色控制台窗口的子进程（GUI 应用 spawn 控制台程序如 tar/taskkill 会闪黑框）
#[cfg(windows)]
pub fn silent_command(program: &str) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let mut cmd = std::process::Command::new(program);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

#[cfg(not(windows))]
pub fn silent_command(program: &str) -> std::process::Command {
    std::process::Command::new(program)
}
