use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let build_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time)
}
