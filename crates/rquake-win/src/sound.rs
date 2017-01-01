use rquake_common::NativeSoundEngine;
use winapi::*;
use user32::*;
use std::mem;
use std::ptr;

#[link(name = "Dsound")]
extern "system" {
    fn DirectSoundCreate(
        pcGuidDevice: LPCGUID, ppDS: *mut LPDIRECTSOUND, pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
}

pub struct DirectSoundEngine {
    ds_device : LPDIRECTSOUND,
    ds_buffer : LPDIRECTSOUNDBUFFER,
}

impl DirectSoundEngine {
    pub fn new() -> DirectSoundEngine {
        DirectSoundEngine {
            ds_device : ptr::null_mut(),
            ds_buffer : ptr::null_mut(),
        }
    }
}

impl NativeSoundEngine for DirectSoundEngine {
    fn init(&mut self) {
        let mut format : WAVEFORMATEX = unsafe { mem::zeroed() };
        format.wFormatTag = WAVE_FORMAT_PCM;
        format.nChannels = 2;
        format.wBitsPerSample = 16;
        format.nSamplesPerSec = 11025;
        format.nBlockAlign = format.nChannels * format.wBitsPerSample / 8;
        format.cbSize = 0;
        format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32; 
        
        let hr = unsafe { DirectSoundCreate(ptr::null_mut(), &mut self.ds_device, ptr::null_mut()) };
        if hr != DS_OK {
            println!("DirectSoundCreate failed: {}", hr);
            self.shutdown();
            return;
        }
        let mut hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_null() {
            println!("No main window found.");
            hwnd = unsafe { GetDesktopWindow() };
        }
        unsafe {(*self.ds_device).SetCooperativeLevel(hwnd, DSSCL_PRIORITY); }

        let mut dsbufdesc : DSBUFFERDESC = unsafe { mem::zeroed() };
        dsbufdesc.dwSize = mem::size_of::<DSBUFFERDESC>() as DWORD;
	    dsbufdesc.dwFlags = DSBCAPS_PRIMARYBUFFER;
	    
        let hr = unsafe { (*self.ds_device).CreateSoundBuffer(&dsbufdesc, &mut self.ds_buffer, ptr::null_mut()) };
        if hr != DS_OK {
            println!("CreateSoundBuffer failed: {}", hr);
            self.shutdown();
            return;
        }

        let hr = unsafe { (*self.ds_buffer).SetFormat(&format) };
        if hr != DS_OK {
            println!("DirectSoundBuffer.SetFormat failed: {}", hr);
            self.shutdown();
            return;
        }

        let mut dsbcaps : DSBCAPS = unsafe { mem::zeroed() };
        dsbcaps.dwSize = mem::size_of::<DSBCAPS>() as DWORD;
        let hr = unsafe { (*self.ds_buffer).GetCaps(&mut dsbcaps) };
        if hr != DS_OK {
            println!("DirectSoundBuffer.GetCaps failed: {}", hr);
            self.shutdown();
            return;
        }

        println!("DirectSoundEngine initialized.");
    }

    fn shutdown(&mut self) {
        if !self.ds_buffer.is_null() {
            unsafe { 
                (*self.ds_buffer).Stop();
                (*self.ds_buffer).Release(); 
            }
        }

        if !self.ds_device.is_null() {
            unsafe { (*self.ds_device).Release(); }
        }
        println!("DirectSoundEngine shut down.");
    }
}