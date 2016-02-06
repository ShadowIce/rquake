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
    
    let mut timer = utils::Timer::new();
    timer.set_bounds(0.001, 0.1);
    timer.set_target(1.0 / 72.0);
    
    while window.is_running() {
        window.handle_message();

        if let Some(time_step) = timer.next() {
            host.frame(time_step);
        }
    }
    
    host.shutdown();
}
