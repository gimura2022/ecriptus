use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=zig/src/main.zig");
    println!("cargo:rerun-if-changed=zig/build.zig");

    let dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path: &Path = Path::new(&dir);

    env::set_current_dir(path.join("zig")).unwrap();

    let status: ExitStatus = Command::new("zig")
        .arg("build")
        .status()
        .expect("Failed to compile Zig lib");

    if !status.success() {
        panic!("Zig build failed");
    }

    env::set_current_dir(path).unwrap();

    println!("cargo:rustc-link-search=native={}", path.join("zig/zig-out/lib").display());
    println!("cargo:rustc-link-lib=static=utils_zig");

    if let Ok(rustflags) = env::var("RUSTFLAGS") {
        env::set_var(
            "RUSTFLAGS",
            format!(
                "{} -C target-cpu=generic -C target-feature=+crt-static -C codegen-units=1 -C relocation-model=pic -C link-args=-s -C panic=abort",
                rustflags
            )
        );
    } else {
        env::set_var(
            "RUSTFLAGS",
            "-C target-cpu=generic -C target-feature=+crt-static -C codegen-units=1 -C relocation-model=pic -C link-args=-s -C panic=abort"
        );
    }

    println!("cargo:rustc-link-arg=-flto=fat");
    println!("cargo:rustc-link-arg=-fPIC");
    println!("cargo:rustc-link-arg=-s");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
}
