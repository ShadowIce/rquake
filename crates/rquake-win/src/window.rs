use rquake_common::{BackBuffer, Window, EventAction, ToggleFullscreen};
use winapi::*;
use user32::*;
use kernel32::{GetModuleHandleW};
use gdi32::*;
use std::ptr;
use std::mem;

// Code from https://users.rust-lang.org/t/tidy-pattern-to-work-with-lpstr-mutable-char-array/2976
// Converts utf-8 to utf-16 (or UCS-2?) and back
use std::ffi::{OsStr, OsString};
use std::os::windows::prelude::*;

trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}
trait FromWide where Self: Sized {
    fn from_wide_null(wide: &[u16]) -> Self;
}
impl FromWide for OsString {
    fn from_wide_null(wide: &[u16]) -> OsString {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        OsString::from_wide(&wide[..len])
    }
}

/// Represents the main window on Windows.
pub struct WinWindow {
    hwnd : HWND,
    running : bool,
    bitmap_info : BITMAPINFO,
    bitmap : Vec<u32>,
    window_width : i32,
    window_height : i32,
    old_window_placement : WINDOWPLACEMENT,
}

impl WinWindow {
    /// Creates a new window. If there is a critical error the method
    /// will return an error string that should be displayed.
    pub fn create_window() -> Result<Self, &'static str> {
        const DEFAULT_WIDTH : i32 = 800;
        const DEFAULT_HEIGHT : i32 = 600;
        const BITMAP_WIDTH : i32 = 320;
        const BITMAP_HEIGHT : i32 = 240;
        
        let hinstance : HINSTANCE = unsafe {
            GetModuleHandleW(ptr::null())
        };
        
        let classname = "CLS_rQuake".to_wide_null();
        let wc = WNDCLASSW {
            style : 0,
            lpfnWndProc: Some(windowproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: unsafe{ LoadIconW(hinstance, 1u16 as *const u16) },
            hCursor: unsafe{ LoadCursorW(ptr::null_mut(), IDC_ARROW) },
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            lpszClassName: classname.as_ptr()
        };
        unsafe {
            if RegisterClassW(&wc) == 0u16 {
                return Err("Failed to register window class");
            }
        }
        
        let style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE;
        let mut clientrect = RECT {
            left : 0,
            top : 0,
            right : DEFAULT_WIDTH as i32,
            bottom: DEFAULT_HEIGHT as i32,
        };
        
        unsafe {
            AdjustWindowRect(&mut clientrect as LPRECT, style, FALSE);
        }
        
        let title = "rQuake".to_wide_null();
        let hwnd = unsafe {
            CreateWindowExW(0, wc.lpszClassName, title.as_ptr(),
                style,
                CW_USEDEFAULT, CW_USEDEFAULT, 
                clientrect.right - clientrect.left, clientrect.bottom - clientrect.top, 
                ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut())
        };
        
        let mut bmp_info : BITMAPINFO = unsafe { mem::zeroed() }; 
        bmp_info.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
        bmp_info.bmiHeader.biWidth = BITMAP_WIDTH;
        bmp_info.bmiHeader.biHeight = -BITMAP_HEIGHT; // negative to place 0,0 at the top left border
        bmp_info.bmiHeader.biPlanes = 1;
        bmp_info.bmiHeader.biBitCount = 32;
        bmp_info.bmiHeader.biSizeImage = (bmp_info.bmiHeader.biWidth * -bmp_info.bmiHeader.biHeight * bmp_info.bmiHeader.biBitCount as i32 / 8) as u32;
        bmp_info.bmiHeader.biCompression = BI_RGB;
        assert_eq!(bmp_info.bmiHeader.biSizeImage % 4, 0);

        let bmp : Vec<u32> = vec![0; (bmp_info.bmiHeader.biSizeImage / 4) as usize];
        
        let mut win_placement : WINDOWPLACEMENT = unsafe { mem::zeroed() };
        win_placement.length = mem::size_of::<WINDOWPLACEMENT>() as UINT;
        
        Ok(WinWindow {
            hwnd : hwnd,
            running : true, 
            bitmap : bmp,
            bitmap_info : bmp_info,
            window_width : DEFAULT_WIDTH,
            window_height : DEFAULT_HEIGHT,
            old_window_placement : win_placement,
        })
    }
}

impl Window for WinWindow {
     fn show_window(&self) {
        unsafe { ShowWindow(self.hwnd, SW_SHOWDEFAULT); };
    }
    
    fn is_running(&self) -> bool {
        self.running
    }
    
    fn handle_message(&mut self) -> Vec<EventAction> {
        let mut actions : Vec<_> = Vec::new();
        let mut msg: MSG = unsafe { mem::zeroed() };
        if unsafe { PeekMessageW(&mut msg as LPMSG, ptr::null_mut(), 0, 0, PM_REMOVE) } != FALSE {
            unsafe { TranslateMessage(&msg) };
            
            match msg.message {
                WM_QUIT => self.running = false,
                WM_KEYDOWN => { actions.push(EventAction::ToggleFullscreen); self.toggle_fullscreen(); },
                _ => unsafe { let _ = DispatchMessageW(&mut msg); },
            }
        }
        actions
    }
    
    fn get_backbuffer(&mut self) -> &mut BackBuffer {
        self
    }
    
    fn render(&mut self) {
        let dc = unsafe { GetDC(self.hwnd) };
        
        let bmp_ptr : *const VOID = self.bitmap.as_ptr() as *const _ as *const VOID;
        let bmpinfo_ptr : *const BITMAPINFO = &self.bitmap_info as *const BITMAPINFO;
            
        unsafe {
            StretchDIBits(dc, 
                0, 0, self.window_width, self.window_height,
                0, 0, self.bitmap_info.bmiHeader.biWidth, -self.bitmap_info.bmiHeader.biHeight,
                bmp_ptr, bmpinfo_ptr, DIB_RGB_COLORS, SRCCOPY);
            ReleaseDC(self.hwnd, dc);
        }
    }
}

impl ToggleFullscreen for WinWindow {
    fn toggle_fullscreen(&mut self) {
        const MONITOR_DEFAULTTOPRIMARY : DWORD = 0x00000001;
        const HWND_TOP : HWND = 0 as HWND;
        
        unsafe {
            let style : i32 = GetWindowLongW(self.hwnd, GWL_STYLE);
            if style & (WS_OVERLAPPEDWINDOW as i32) != 0 {
                let mut mi : MONITORINFO = mem::zeroed();
                mi.cbSize = mem::size_of::<MONITORINFO>() as DWORD;
                let res_gwp = GetWindowPlacement(self.hwnd, &mut self.old_window_placement);
                let res_gmi = GetMonitorInfoW(MonitorFromWindow(self.hwnd, MONITOR_DEFAULTTOPRIMARY), &mut mi);
                if res_gwp != 0 && res_gmi != 0 {
                    self.window_width = mi.rcMonitor.right - mi.rcMonitor.left;
                    self.window_height = mi.rcMonitor.bottom - mi.rcMonitor.top;

                    SetWindowLongW(self.hwnd, GWL_STYLE, style & !(WS_OVERLAPPEDWINDOW as i32));
                    SetWindowPos(self.hwnd, HWND_TOP,
                        mi.rcMonitor.left, mi.rcMonitor.top,
                        self.window_width, self.window_height,
                        SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
                }
            } else {
                self.window_width = self.old_window_placement.rcNormalPosition.right - self.old_window_placement.rcNormalPosition.left;
                self.window_height = self.old_window_placement.rcNormalPosition.bottom - self.old_window_placement.rcNormalPosition.top;
                SetWindowLongW(self.hwnd, GWL_STYLE, style | (WS_OVERLAPPEDWINDOW as i32));
                SetWindowPlacement(self.hwnd, &self.old_window_placement);
                SetWindowPos(self.hwnd, 0 as HWND, 0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER |
                    SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
            }
        }
    }    
}

impl BackBuffer for WinWindow {
    fn get_buffer(&mut self) -> &mut [u32] {
        &mut self.bitmap
    }
    
    fn get_width(&self) -> u32 {
        self.bitmap_info.bmiHeader.biWidth as u32
    }
    
    fn get_height(&self) -> u32 {
        self.bitmap_info.bmiHeader.biHeight as u32
    }
}

unsafe extern "system" fn windowproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}
