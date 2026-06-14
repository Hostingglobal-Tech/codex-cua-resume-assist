#[cfg(not(windows))]
fn main() {
    eprintln!("codex-cua-win-capture only runs on Windows");
    std::process::exit(2);
}

#[cfg(windows)]
fn main() {
    if let Err(e) = windows_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(windows)]
fn windows_main() -> Result<(), String> {
    use image::RgbaImage;
    use serde_json::json;
    use std::env;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::PathBuf;
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::{CloseHandle, HWND};
    use windows_sys::Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, CAPTUREBLT,
        DIB_RGB_COLORS, HBITMAP, SRCCOPY,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetSystemMetrics, GetWindowTextLengthW, GetWindowTextW,
        GetWindowThreadProcessId, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
    };

    let mut args = env::args_os();
    let _ = args.next();
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("usage: codex-cua-win-capture.exe C:\\path\\screen.png".to_string())?;

    let hwnd = unsafe { GetForegroundWindow() };
    let mut pid = 0u32;
    if !hwnd.is_null() {
        unsafe {
            GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);
        }
    }

    let title = window_text(hwnd);
    let class_name = class_name(hwnd);
    let process_path = process_path(pid).unwrap_or_default();
    let process_name = process_path
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or("")
        .to_string();
    let window_kind = classify_window(&process_name, &title, &class_name);

    let screen = capture_screen(&output)?;

    println!(
        "{}",
        json!({
            "ok": true,
            "screenshot": output.display().to_string(),
            "active_window": {
                "kind": window_kind,
                "process_name": process_name,
                "class_name": class_name,
            },
            "screen": screen,
        })
    );
    fn window_text(hwnd: HWND) -> String {
        if hwnd.is_null() {
            return String::new();
        }
        let len = unsafe { GetWindowTextLengthW(hwnd) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0u16; len as usize + 1];
        let got = unsafe { GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
        utf16_lossy(&buf[..got.max(0) as usize])
    }

    fn class_name(hwnd: HWND) -> String {
        if hwnd.is_null() {
            return String::new();
        }
        let mut buf = vec![0u16; 512];
        let got = unsafe { GetClassNameW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
        utf16_lossy(&buf[..got.max(0) as usize])
    }

    fn process_path(pid: u32) -> Option<String> {
        if pid == 0 {
            return None;
        }
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
        if handle.is_null() {
            return None;
        }
        let mut buf = vec![0u16; 32768];
        let mut len = buf.len() as u32;
        let ok = unsafe { QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut len) };
        unsafe {
            CloseHandle(handle);
        }
        if ok == 0 {
            return None;
        }
        Some(utf16_lossy(&buf[..len as usize]))
    }

    fn utf16_lossy(slice: &[u16]) -> String {
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }

    fn classify_window(process_name: &str, title: &str, class_name: &str) -> &'static str {
        let p = process_name.to_ascii_lowercase();
        let t = title.to_ascii_lowercase();
        let c = class_name.to_ascii_lowercase();
        if p.contains("wezterm") || t.contains("wezterm") {
            "wezterm"
        } else if p == "powershell.exe"
            || p == "pwsh.exe"
            || t.contains("powershell")
            || c.contains("powershell")
        {
            "powershell"
        } else if p == "cmd.exe" || t.contains("command prompt") {
            "cmd"
        } else if p.contains("windowsterminal") || c.contains("cascadia") {
            "windows-terminal"
        } else {
            "other"
        }
    }

    fn capture_screen(path: &PathBuf) -> Result<serde_json::Value, String> {
        let x = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
        let y = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };
        let width = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
        let height = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };
        if width <= 0 || height <= 0 {
            return Err(format!("invalid virtual screen size {width}x{height}"));
        }

        let screen_dc = unsafe { GetDC(null_mut()) };
        if screen_dc.is_null() {
            return Err("GetDC failed".to_string());
        }
        let mem_dc = unsafe { CreateCompatibleDC(screen_dc) };
        if mem_dc.is_null() {
            unsafe {
                ReleaseDC(null_mut(), screen_dc);
            }
            return Err("CreateCompatibleDC failed".to_string());
        }
        let bitmap: HBITMAP = unsafe { CreateCompatibleBitmap(screen_dc, width, height) };
        if bitmap.is_null() {
            unsafe {
                DeleteDC(mem_dc);
                ReleaseDC(null_mut(), screen_dc);
            }
            return Err("CreateCompatibleBitmap failed".to_string());
        }

        let old = unsafe { SelectObject(mem_dc, bitmap as _) };
        let copied = unsafe {
            BitBlt(
                mem_dc,
                0,
                0,
                width,
                height,
                screen_dc,
                x,
                y,
                SRCCOPY | CAPTUREBLT,
            )
        };
        unsafe {
            SelectObject(mem_dc, old);
        }
        if copied == 0 {
            unsafe {
                DeleteObject(bitmap as _);
                DeleteDC(mem_dc);
                ReleaseDC(null_mut(), screen_dc);
            }
            return Err("BitBlt failed".to_string());
        }

        let mut info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [unsafe { std::mem::zeroed() }],
        };
        let mut pixels = vec![0u8; width as usize * height as usize * 4];
        let got = unsafe {
            GetDIBits(
                screen_dc,
                bitmap,
                0,
                height as u32,
                pixels.as_mut_ptr() as *mut _,
                &mut info,
                DIB_RGB_COLORS,
            )
        };
        unsafe {
            DeleteObject(bitmap as _);
            DeleteDC(mem_dc);
            ReleaseDC(null_mut(), screen_dc);
        }
        if got == 0 {
            return Err("GetDIBits failed".to_string());
        }

        for px in pixels.chunks_exact_mut(4) {
            px.swap(0, 2);
            px[3] = 255;
        }
        let img = RgbaImage::from_vec(width as u32, height as u32, pixels)
            .ok_or("image buffer size mismatch".to_string())?;
        img.save(path)
            .map_err(|e| format!("save {} failed: {e}", path.display()))?;

        Ok(json!({
            "x": x,
            "y": y,
            "width": width,
            "height": height,
        }))
    }
    Ok(())
}
