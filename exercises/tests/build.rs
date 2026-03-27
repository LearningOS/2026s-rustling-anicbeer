// build.rs
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 关键：不要减 5！直接用当前时间
    println!("cargo:rustc-env=TEST_FOO={}", now);

    // 其他保持不变
    println!("cargo:rustc-cfg=feature=\"pass\"");
    println!("cargo:rerun-if-changed=tests7.rs");
    println!("cargo:rerun-if-changed=tests8.rs");
}