extern crate pkg_config;
use pkg_config::Config;

fn main() {
    let config = Config::new();

    if cfg!(feature = "gui") {
        for lib in &["x11"] {
            config.probe(lib).unwrap();
        }
    } else if cfg!(feature = "tui") {
        for lib in &["ncurses"] {
            config.probe(lib).unwrap();
        }
    }

    println!("cargo::rerun-if-changed=build.rs");
}
