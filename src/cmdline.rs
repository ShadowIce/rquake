extern crate clap;
use self::clap::{Arg, App};

pub struct CmdConfig {
    pub nosound : bool,
    pub windowed : bool,
}

pub fn parse_cmdline() -> CmdConfig {
    let matches = App::new("Quake")
        .version("0.1")
        .author("Maurice Gilden <MauriceG@gmx.net>")
        .about("Rust port of Quake")
        .arg(Arg::with_name("nosound")
            .long("nosound")
            .help("disables sound"))
        .arg(Arg::with_name("windowed")
            .long("windowed")
            .help("start quake in windowed mode"))
        .get_matches();
        
    CmdConfig {
        nosound : matches.is_present("nosound"),
        windowed : matches.is_present("windowed"),
    }
}
