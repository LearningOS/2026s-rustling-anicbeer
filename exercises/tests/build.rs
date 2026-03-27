fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 关键修改：设置 TEST_FOO 为当前时间戳减去偏移量
    // 这样测试运行时的时间戳会自然落在 [e, e+10) 范围内
    println!("cargo:rustc-env=TEST_FOO={}", timestamp - 5);

    // 为 tests8 启用 "pass" 特性（保持不变）
    println!("cargo:rustc-cfg=feature=\"pass\"");

    // 确保文件变化时重新运行构建脚本（保持不变）
    println!("cargo:rerun-if-changed=tests7.rs");
    println!("cargo:rerun-if-changed=tests8.rs");
}