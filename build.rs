#[cfg(windows)]
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/quake.ico")
            .set("InternalName", "RQUAKE.EXE")
            // manually set version 1.0.0.0
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        match res.compile() {
            Ok(_) => {},
            Err(_) => panic!("Resource could not be compiled"),
        };
    }
}