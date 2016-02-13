extern crate rand;

extern crate rquake_fs;
extern crate rquake_common;
extern crate rquake_engine;

#[cfg(windows)]
extern crate rquake_win;

use rquake_common::{Timer,Window};
use rquake_engine::Host;

#[cfg(windows)]
use rquake_win::WinWindow;

use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

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
    let mut rng = rand::weak_rng();
    
    let _ = cmdline::parse_cmdline();
    
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
    
    let mut timer = Timer::new();
    timer.set_bounds(0.001, 0.1);
    timer.set_target(1.0 / 72.0);
    
    while window.is_running() {
        window.handle_message();

        if let Some(time_step) = timer.next() {
            host.frame(time_step);
            
            {
                // Test code, fill bitmap with random values.
                // Slow in debug build.
                let mut bb = window.get_backbuffer();
                let mut bmp = bb.get_buffer();
                for v in bmp.iter_mut() {
                    *v = rng.gen::<u8>();
                    //*v = ((*v as i32 + 1) % 255) as u8;
                }
            }

            window.render();
        } else {
            sleep(Duration::from_millis(1));
        }
    }
    
    host.shutdown();
}
