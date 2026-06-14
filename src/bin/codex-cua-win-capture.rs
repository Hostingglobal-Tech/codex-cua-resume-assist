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
    use windows_sys::Win32::Foundation::{CloseHandle, HWND, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, GetWindowDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        CAPTUREBLT, DIB_RGB_COLORS, HBITMAP, HDC, SRCCOPY,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
        VK_RETURN,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetSystemMetrics, GetWindowRect, GetWindowTextLengthW,
        GetWindowTextW, GetWindowThreadProcessId, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN,
        SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    };

    let mut args = env::args_os();
    let _ = args.next();
    if args
        .next()
        .as_deref()
        .and_then(|arg| arg.to_str())
        .is_some_and(|arg| arg == "--type")
    {
        let text = args.next().and_then(|arg| arg.into_string().ok()).ok_or(
            "usage: codex-cua-win-capture.exe --type TEXT [--expect-hwnd HWND]".to_string(),
        )?;
        let mut expect_hwnd = None;
        while let Some(arg) = args.next() {
            if arg == "--expect-hwnd" {
                expect_hwnd = args.next().and_then(|value| value.into_string().ok());
            }
        }
        return type_into_foreground(&text, expect_hwnd.as_deref());
    }
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

    let title = read_window_text(hwnd);
    let class_name = read_class_name(hwnd);
    let process_path = process_path_for_pid(pid).unwrap_or_default();
    let process_name = process_path
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or("")
        .to_string();
    let window_kind = classify_window(&process_name, &title, &class_name);

    let screen = capture_screen(&output, hwnd)?;

    println!(
        "{}",
        json!({
            "ok": true,
            "screenshot": output.display().to_string(),
            "active_window": {
                "hwnd": hwnd as usize,
                "kind": window_kind,
                "process_name": process_name,
                "class_name": class_name,
                "title": title,
            },
            "screen": screen,
        })
    );
    fn read_window_text(hwnd: HWND) -> String {
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

    fn read_class_name(hwnd: HWND) -> String {
        if hwnd.is_null() {
            return String::new();
        }
        let mut buf = vec![0u16; 512];
        let got = unsafe { GetClassNameW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
        utf16_lossy(&buf[..got.max(0) as usize])
    }

    fn process_path_for_pid(pid: u32) -> Option<String> {
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
        } else if p.contains("windowsterminal") || c.contains("cascadia") {
            "windows-terminal"
        } else if p == "powershell.exe"
            || p == "pwsh.exe"
            || t.contains("powershell")
            || c.contains("powershell")
        {
            "powershell"
        } else if p == "cmd.exe" || t.contains("command prompt") {
            "cmd"
        } else {
            "other"
        }
    }

    fn foreground_window_meta() -> (usize, String, String, String, &'static str) {
        let hwnd = unsafe { GetForegroundWindow() };
        let mut pid = 0u32;
        if !hwnd.is_null() {
            unsafe {
                GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);
            }
        }
        let title = read_window_text(hwnd);
        let class_name = read_class_name(hwnd);
        let process_path = process_path_for_pid(pid).unwrap_or_default();
        let process_name = process_path
            .rsplit(['\\', '/'])
            .next()
            .unwrap_or("")
            .to_string();
        let window_kind = classify_window(&process_name, &title, &class_name);
        (hwnd as usize, process_name, class_name, title, window_kind)
    }

    fn type_into_foreground(text: &str, expect_hwnd: Option<&str>) -> Result<(), String> {
        let text = text.trim();
        if text.is_empty() || text.len() > 80 || text.contains('\n') || text.contains('\r') {
            return Err("refusing unsafe --type text".to_string());
        }
        let (hwnd, process_name, class_name, title, window_kind) = foreground_window_meta();
        if let Some(expected) = expect_hwnd {
            if expected.trim() != hwnd.to_string() {
                return Err(format!(
                    "foreground window changed before typing: expected_hwnd={} actual_hwnd={}",
                    expected.trim(),
                    hwnd
                ));
            }
        }
        send_text_plus_enter(text)?;
        println!(
            "{}",
            json!({
                "ok": true,
                "action": "type_text_enter",
                "active_window": {
                    "hwnd": hwnd,
                    "kind": window_kind,
                    "process_name": process_name,
                    "class_name": class_name,
                    "title": title,
                }
            })
        );
        Ok(())
    }

    fn keyboard_input(w_vk: u16, w_scan: u16, flags: u32) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: w_vk,
                    wScan: w_scan,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }
    }

    fn send_input_pair(down: INPUT, up: INPUT) -> Result<(), String> {
        let mut inputs = [down, up];
        let sent = unsafe {
            SendInput(
                inputs.len() as u32,
                inputs.as_mut_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            )
        };
        if sent != inputs.len() as u32 {
            return Err("SendInput failed".to_string());
        }
        Ok(())
    }

    fn send_text_plus_enter(text: &str) -> Result<(), String> {
        for unit in text.encode_utf16() {
            let down = keyboard_input(0, unit, KEYEVENTF_UNICODE);
            let up = keyboard_input(0, unit, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP);
            send_input_pair(down, up)?;
        }
        let enter_down = keyboard_input(VK_RETURN, 0, 0);
        let enter_up = keyboard_input(VK_RETURN, 0, KEYEVENTF_KEYUP);
        send_input_pair(enter_down, enter_up)
    }

    fn capture_screen(path: &PathBuf, foreground_hwnd: HWND) -> Result<serde_json::Value, String> {
        match capture_virtual_screen(path) {
            Ok(meta) => Ok(meta),
            Err(screen_err) => {
                if foreground_hwnd.is_null() {
                    return Err(screen_err);
                }
                match capture_foreground_window(path, foreground_hwnd) {
                    Ok(mut meta) => {
                        if let Some(obj) = meta.as_object_mut() {
                            obj.insert(
                                "fallback_reason".to_string(),
                                json!(format!("virtual screen capture failed: {screen_err}")),
                            );
                        }
                        Ok(meta)
                    }
                    Err(window_err) => Err(format!(
                        "{screen_err}; foreground window capture failed: {window_err}"
                    )),
                }
            }
        }
    }

    fn capture_virtual_screen(path: &PathBuf) -> Result<serde_json::Value, String> {
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
        let result = capture_dc_region(path, screen_dc, x, y, width, height, "virtual_screen");
        unsafe {
            ReleaseDC(null_mut(), screen_dc);
        }
        result
    }

    fn capture_foreground_window(path: &PathBuf, hwnd: HWND) -> Result<serde_json::Value, String> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let ok = unsafe { GetWindowRect(hwnd, &mut rect as *mut RECT) };
        if ok == 0 {
            return Err("GetWindowRect failed".to_string());
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            return Err(format!("invalid foreground window size {width}x{height}"));
        }

        let window_dc = unsafe { GetWindowDC(hwnd) };
        if window_dc.is_null() {
            return Err("GetWindowDC failed".to_string());
        }
        let result = capture_dc_region(path, window_dc, 0, 0, width, height, "foreground_window");
        unsafe {
            ReleaseDC(hwnd, window_dc);
        }
        result.map(|mut meta| {
            if let Some(obj) = meta.as_object_mut() {
                obj.insert("screen_x".to_string(), json!(rect.left));
                obj.insert("screen_y".to_string(), json!(rect.top));
            }
            meta
        })
    }

    fn capture_dc_region(
        path: &PathBuf,
        source_dc: HDC,
        source_x: i32,
        source_y: i32,
        width: i32,
        height: i32,
        source: &str,
    ) -> Result<serde_json::Value, String> {
        let mem_dc = unsafe { CreateCompatibleDC(source_dc) };
        if mem_dc.is_null() {
            return Err("CreateCompatibleDC failed".to_string());
        }
        let bitmap: HBITMAP = unsafe { CreateCompatibleBitmap(source_dc, width, height) };
        if bitmap.is_null() {
            unsafe {
                DeleteDC(mem_dc);
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
                source_dc,
                source_x,
                source_y,
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
                source_dc,
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
            "source": source,
            "x": source_x,
            "y": source_y,
            "width": width,
            "height": height,
        }))
    }
    Ok(())
}
