extern crate rquake_fs;
extern crate rquake_common;

#[cfg(windows)]
extern crate rquake_win;

//use rquake_fs::*;
use rquake_common::*;

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
    println!("Hello Quake!");
    
    let cmdconfig = cmdline::parse_cmdline();
    
    let window = create_window();
    let mut window = match window {
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        },
        Ok(window) => window,
    };
    
    while window.is_running() {
        window.handle_message();
    }
    
    //let res = PackFile::open("Id1/PAK0.PAK");
    //match res {
    //    Err(e) => { println!("{}", e); }
    //    _ => {}
    //};
}
