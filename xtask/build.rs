use std::env;

fn main() {
    println!(
        "cargo:rustc-env=OUT_DIR={}",
        &env::var_os("OUT_DIR").unwrap().into_string().unwrap()
    );
}
