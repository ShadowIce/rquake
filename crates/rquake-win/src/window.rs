extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate rquake_common;

use self::winapi::*;
use self::rquake_common::system::*;
use self::user32::*;
use self::kernel32::GetModuleHandleW;

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

pub struct WinWindow {
    hwnd : HWND,
    running : bool,    
}

impl WinWindow {
    pub fn create_window() -> Result<Self,&'static str> {
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
            hIcon: ptr::null_mut(),
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
        
        let style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
        let mut clientrect = RECT {
            left : 0,
            top : 0,
            right : 800,
            bottom: 600,
        };
        
        unsafe {
            AdjustWindowRect(&mut clientrect as LPRECT, style, FALSE);
        }
        
        let title = "rQuake".to_wide_null();
        let hwnd = unsafe {
            user32::CreateWindowExW(0, wc.lpszClassName, title.as_ptr(),
                style,
                CW_USEDEFAULT, CW_USEDEFAULT, 
                clientrect.right - clientrect.left, clientrect.bottom - clientrect.top, 
                ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut())
        };
        
        Ok(WinWindow { 
            hwnd : hwnd,
            running : true, 
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
    
    fn handle_message(&mut self) {
        unsafe {
            let mut msg: MSG = mem::zeroed();
            if PeekMessageW(&mut msg as LPMSG, ptr::null_mut(), 0, 0, PM_REMOVE) != FALSE {
                TranslateMessage(&msg);
                
                if msg.message == WM_QUIT { self.running = false; }
                
                DispatchMessageW(&mut msg);
            }
        }
    }
}

unsafe extern "system" fn windowproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        _ => {
            //println!("msg: {}", msg);
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}


