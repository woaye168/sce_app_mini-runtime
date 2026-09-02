// Windows 下把 assets/app.ico 嵌进所有 exe（资源管理器/任务栏图标）
fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");
    println!("cargo:rerun-if-changed=assets/app.rc");
    #[cfg(windows)]
    embed_resource::compile("assets/app.rc", embed_resource::NONE);
}
