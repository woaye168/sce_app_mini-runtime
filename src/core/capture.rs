//! 进程窗口截屏（CLI 自验能力）：按窗口标题找窗口 → WGC 截取 → PNG。
//! 方案来自 sce_app_editor-patch core/capture.rs 的实测结论（WGC 截窗口在遮挡下仍可用）。

use anyhow::{anyhow, Result};
use std::path::Path;
use std::time::Duration;

#[cfg(windows)]
pub fn capture_by_title(title_substr: &str, out: &Path) -> Result<(u32, u32)> {
    let hwnd = find_window_by_title(title_substr)
        .ok_or_else(|| anyhow!("找不到标题含「{title_substr}」的窗口（游戏在运行？）"))?;
    ensure_visible(hwnd);
    wgc_capture(hwnd, out)
}

/// 按标题子串找最大可见窗口
#[cfg(windows)]
fn find_window_by_title(title_substr: &str) -> Option<*mut std::ffi::c_void> {
    use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextW, IsIconic, IsWindowVisible,
    };

    struct Ctx {
        needle: String,
        best: HWND,
        best_area: i64,
        best_min: HWND, // 最小化的兜底
    }
    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = &mut *(lparam as *mut Ctx);
        let mut buf = [0u16; 256];
        let n = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        if n > 0 {
            let title = String::from_utf16_lossy(&buf[..n as usize]);
            if title.contains(&ctx.needle) {
                use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;
                let mut rc = std::mem::zeroed();
                let _ = GetWindowRect(hwnd, &mut rc);
                let area = (rc.right - rc.left) as i64 * (rc.bottom - rc.top) as i64;
                if IsWindowVisible(hwnd) != 0 && IsIconic(hwnd) == 0 {
                    if area > ctx.best_area {
                        ctx.best_area = area;
                        ctx.best = hwnd;
                    }
                } else if ctx.best_min.is_null() {
                    ctx.best_min = hwnd;
                }
            }
        }
        1
    }
    let mut ctx = Ctx {
        needle: title_substr.to_string(),
        best: std::ptr::null_mut(),
        best_area: 0,
        best_min: std::ptr::null_mut(),
    };
    unsafe {
        EnumWindows(Some(enum_proc), &mut ctx as *mut Ctx as LPARAM);
    }
    if !ctx.best.is_null() {
        Some(ctx.best)
    } else if !ctx.best_min.is_null() {
        Some(ctx.best_min)
    } else {
        None
    }
}

/// 最小化时先不激活地恢复（WGC 对最小化窗口建 GraphicsCaptureItem 会失败）
#[cfg(windows)]
fn ensure_visible(hwnd: *mut std::ffi::c_void) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{IsIconic, ShowWindow, SW_SHOWNOACTIVATE};
    unsafe {
        if IsIconic(hwnd) != 0 {
            ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

/// WGC 截整个窗口 → PNG
#[cfg(windows)]
fn wgc_capture(hwnd: *mut std::ffi::c_void, path: &Path) -> Result<(u32, u32)> {
    use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
    use windows_capture::frame::Frame;
    use windows_capture::graphics_capture_api::InternalCaptureControl;
    use windows_capture::settings::{
        ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
        MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
    };
    use windows_capture::window::Window;

    struct CapHandler {
        done: std::sync::mpsc::Sender<Result<image::RgbaImage, String>>,
    }
    impl GraphicsCaptureApiHandler for CapHandler {
        type Flags = std::sync::mpsc::Sender<Result<image::RgbaImage, String>>;
        type Error = anyhow::Error;

        fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
            Ok(Self { done: ctx.flags })
        }

        fn on_frame_arrived(
            &mut self,
            frame: &mut Frame,
            capture_control: InternalCaptureControl,
        ) -> Result<(), Self::Error> {
            let r = (|| {
                let (w, h) = (frame.width(), frame.height());
                let mut buf = frame.buffer_crop(0, 0, w, h).map_err(|e| format!("{e}"))?;
                let raw = buf.as_nopadding_buffer().map_err(|e| format!("{e}"))?;
                image::RgbaImage::from_raw(w, h, raw.to_vec()).ok_or_else(|| "构造图像失败".to_string())
            })();
            let _ = self.done.send(r);
            let _ = capture_control.stop();
            Ok(())
        }
    }

    let window = Window::from_raw_hwnd(hwnd);
    let (tx, rx) = std::sync::mpsc::channel();
    let settings = Settings::new(
        window,
        CursorCaptureSettings::Default,
        DrawBorderSettings::Default,
        SecondaryWindowSettings::Default,
        MinimumUpdateIntervalSettings::Default,
        DirtyRegionSettings::Default,
        ColorFormat::Rgba8,
        tx,
    );
    // start 是阻塞变体（占当前线程跑消息循环直到 stop），recv_timeout 永远执行不到；
    // 改 start_free_threaded 后台跑捕获，主线程等帧，超时后 stop 回收线程
    let control =
        CapHandler::start_free_threaded(settings).map_err(|e| anyhow!("启动窗口捕获失败: {e}"))?;
    let img = match rx.recv_timeout(Duration::from_secs(15)) {
        Ok(r) => r.map_err(|e| anyhow!(e))?,
        Err(_) => {
            let _ = control.stop();
            return Err(anyhow!("窗口捕获超时（15s 未收到帧）"));
        }
    };
    let (w, h) = (img.width(), img.height());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    img.save(path).map_err(|e| anyhow!("保存 PNG 失败: {e}"))?;
    Ok((w, h))
}
