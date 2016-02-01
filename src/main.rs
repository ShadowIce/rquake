extern crate time;

extern crate rquake_fs;
extern crate rquake_common;
extern crate rquake_engine;

#[cfg(windows)]
extern crate rquake_win;

use rquake_common::*;
use rquake_engine::*;

#[cfg(windows)]
use rquake_win::*;

mod cmdline;

#[cfg(windows)]
fn create_window() -> Result<Box<Window>, &'static str> {
    let res = WinWindow::create_window();
    match res {
        Ok(window) => Ok(Box::new(window)),
        Err(err) => Err(err),
    }
}

fn main() {
    const TARGET_FRAMETIME : f32 = 1.0 / 60.0;
    
    let cmdconfig = cmdline::parse_cmdline();
    
    let host = Host::new();
    host.init();
    
    let window = create_window();
    let mut window = match window {
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        },
        Ok(window) => window,
    };
    
    let mut oldtime = time::precise_time_s() as f32;
    let mut acc_time = 0.0f32;
    
    while window.is_running() {
        window.handle_message();

        let newtime = time::precise_time_s() as f32;
        acc_time = acc_time + (oldtime - newtime);
        oldtime = newtime;

        if acc_time > TARGET_FRAMETIME {
            host.frame(TARGET_FRAMETIME);
            acc_time = acc_time - TARGET_FRAMETIME;
        }
    }
    
    host.shutdown();
}
