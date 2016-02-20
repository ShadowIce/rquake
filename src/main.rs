extern crate rand;

extern crate rquake_fs;
extern crate rquake_common;
extern crate rquake_engine;

#[cfg(windows)]
extern crate rquake_win;

use rquake_common::{Timer,Window};
use rquake_engine::Host;
use rquake_fs::{PackFile};

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
    
    // Test code, reads a file from the resources
    let mut pf0 = match PackFile::open("Id1/PAK0.PAK") {
        Ok(f) => f,
        Err(_) => return,
    };
    
    let pause_bitmap = match pf0.read_lmp("gfx/pause.lmp") {
        Ok(f) => f,
        Err(_) => return,
    };
    
    // Create main window
    let window = create_window();
    let mut window = match window {
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        },
        Ok(window) => window,
    };
    
    // Create game timer
    let mut timer = Timer::new();
    timer.set_bounds(0.001, 0.1);
    timer.set_target(1.0 / 72.0);
    
    // Game loop
    while window.is_running() {
        window.handle_message();

        if let Some(time_step) = timer.next() {
            host.frame(time_step);
            
            {
                // Test code, fill bitmap with random values.
                // Slow in debug build.
                let mut bb = window.get_backbuffer();
                let bb_width = bb.get_width();
                let mut bmp = bb.get_buffer(); // bb borrowed as mutable here on windows
                for v in bmp.iter_mut() {
                    *v = rng.gen::<u32>();
                }
                
                // Test code, draw pause logo
                // memcpy anyone?
                for y in 0..pause_bitmap.height {
                    let src_x = (pause_bitmap.width * y) as usize;
                    let dst_x = (bb_width * y as u32) as usize;
                    let mut src = pause_bitmap.bitmap.iter().skip(src_x).take(pause_bitmap.width as usize);
                    for v in bmp.iter_mut().skip(dst_x).take(pause_bitmap.width as usize) {
                        *v = *(src.next().unwrap_or(&0u32));
                    }
                }
            }

            window.render();
        } else {
            sleep(Duration::from_millis(1));
        }
    }
    
    host.shutdown();
}
