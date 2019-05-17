extern crate pkg_config;

#[cfg(target_os = "windows")]
fn main() {}

#[cfg(target_os = "linux")]
use std::{env, fs::File, io::Write, path::Path};

#[cfg(target_os = "linux")]
fn main() {
    let mut config = String::new();
    let libdir = match pkg_config::get_variable("x11", "libdir") {
        Ok(libdir) => format!("Some(\"{}\")", libdir),
        Err(_) => "None".to_string(),
    };

    config.push_str(&format!(
        "pub const x11: Option<&'static str> = {};\n",
        libdir
    ));

    let config = format!("pub mod config {{ pub mod libdir {{\n{}}}\n}}", config);
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR environmental variable");
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f =
        File::create(&dest_path).expect(&format!("Failed to create file at `{:?}`", dest_path));

    f.write_all(&config.into_bytes())
        .expect(&format!("Failed to write to file `{:?}`", dest_path));

    let target = env::var("TARGET").expect("Failed to get TARGET environmental variable.\nPossible values: `linux`, `freebsd`, `dragonfly`");
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dl");
    } else if target.contains("freebsd") || target.contains("dragonfly") {
        println!("cargo:rustc-link-lib=c");
    }
}
