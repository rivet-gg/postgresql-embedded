pub fn get_target_triple() -> String {
    let target = std::env::var("OVERRIDE_TARGET").unwrap_or_else(|_| target_triple::TARGET.to_string());

    // Replace gnu targets with msvc
    if target.starts_with("x86_64-pc-windows-") {
        "x86_64-pc-windows-msvc".to_string()
    } else {
        target
    }
}

