extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate rquake_common;
extern crate gdi32;

use self::winapi::*;
use self::rquake_common::{BackBuffer, Window};
use self::user32::*;
use self::kernel32::{GetModuleHandleW};
use self::gdi32::*;

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
    bitmap : Vec<u8>,
}

impl WinWindow {
    /// Creates a new window. If there is a critical error the method
    /// will return an error string that should be displayed.
    pub fn create_window() -> Result<Self,&'static str> {
        const DEFAULT_WIDTH : u32 = 800;
        const DEFAULT_HEIGHT : u32 = 600;
        
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
            user32::CreateWindowExW(0, wc.lpszClassName, title.as_ptr(),
                style,
                CW_USEDEFAULT, CW_USEDEFAULT, 
                clientrect.right - clientrect.left, clientrect.bottom - clientrect.top, 
                ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut())
        };
        
        let mut bmp_info : BITMAPINFO = unsafe { mem::zeroed() }; 
        bmp_info.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
        bmp_info.bmiHeader.biWidth = DEFAULT_WIDTH as i32;
        bmp_info.bmiHeader.biHeight = DEFAULT_HEIGHT as i32;
        bmp_info.bmiHeader.biPlanes = 1;
        bmp_info.bmiHeader.biBitCount = 32;
        bmp_info.bmiHeader.biSizeImage = DEFAULT_WIDTH * DEFAULT_HEIGHT * 4;
        bmp_info.bmiHeader.biCompression = BI_RGB;
        
        let bmp : Vec<u8> = vec![55; bmp_info.bmiHeader.biSizeImage as usize];
        
        Ok(WinWindow {
            hwnd : hwnd,
            running : true, 
            bitmap : bmp,
            bitmap_info : bmp_info,
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
    
    fn get_backbuffer(&mut self) -> &mut BackBuffer {
        self as &mut BackBuffer
    }
    
    fn render(&mut self) {
        let dc = unsafe { GetDC(self.hwnd) };
        
        unsafe {
            StretchDIBits(dc, 
                0, 0, self.bitmap_info.bmiHeader.biWidth, self.bitmap_info.bmiHeader.biHeight,
                0, 0, self.bitmap_info.bmiHeader.biWidth, self.bitmap_info.bmiHeader.biHeight,
                mem::transmute(self.bitmap.as_ptr()), mem::transmute(&self.bitmap_info), DIB_RGB_COLORS, SRCCOPY);
            ReleaseDC(self.hwnd, dc);
        }
    }
}

impl BackBuffer for WinWindow {
    fn get_buffer(&mut self) -> &mut [u8] {
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
            //println!("msg: {}", msg);
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}


