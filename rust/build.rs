// build.rs

fn main()
{
    if let Ok(rustflags) = std::env::var("RUSTFLAGS") {
        unsafe {
            std::env::set_var(
                "RUSTFLAGS",
                format!(
                    "{} -C target-cpu=generic -C target-feature=+crt-static -C codegen-units=1 -C \
                     relocation-model=pic -C link-args=-s -C panic=abort",
                    rustflags
                )
            );
        }
    }
    else {
        unsafe {
            std::env::set_var(
                "RUSTFLAGS",
                "-C target-cpu=generic -C target-feature=+crt-static -C codegen-units=1 -C \
                 relocation-model=pic -C link-args=-s -C panic=abort"
            );
        }
    }

    println!("cargo:rustc-link-arg=-flto=fat");

    println!("cargo:rustc-link-arg=-fPIC");

    println!("cargo:rustc-link-arg=-s");

    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
}
