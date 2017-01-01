extern crate rquake_fs;
extern crate rquake_common;
extern crate rquake_engine;

#[cfg(windows)]
extern crate rquake_win;

use rquake_common::{Timer,Window,NativeSoundEngine};
use rquake_engine::{Host,SoundEngine};
use rquake_fs::{GameResourcesImpl};

#[cfg(windows)]
use rquake_win::{WinWindow,DirectSoundEngine};

use std::thread::sleep;
use std::time::Duration;

mod cmdline;

#[cfg(windows)]
fn create_window() -> Result<Box<Window>, &'static str> {
    let res = WinWindow::create_window();
    match res {
        Ok(window) => Ok(Box::new(window)),
        Err(err) => Err(err),
    }
}

#[cfg(windows)]
fn create_sound_engine() -> Box<NativeSoundEngine> {
    Box::new(DirectSoundEngine::new())
}

fn main() {
    let _ = cmdline::parse_cmdline();

    // Create main window
    let window = create_window();
    let mut window = match window {
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        },
        Ok(window) => window,
    };

    let native_snd = create_sound_engine();
    let mut snd = SoundEngine::new(native_snd);
    let mut game_res = GameResourcesImpl::new();
    let mut host = Host::new(&mut game_res, &mut snd);

    host.init();

    // Create game timer
    let mut timer = Timer::new();
    timer.set_bounds(0.001, 0.1);
    timer.set_target(1.0 / 72.0);
    
    // Game loop
    let mut pending_actions = Vec::new();
    while window.is_running() {
        let mut new_actions = window.handle_message();
        pending_actions.append(&mut new_actions);

        if let Some(time_step) = timer.next() {
            host.frame(time_step, &pending_actions);
            window.render();
        } else {
            sleep(Duration::from_millis(1));
        }
    }
    
    host.shutdown();
}
