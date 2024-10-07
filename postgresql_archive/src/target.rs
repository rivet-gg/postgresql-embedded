pub fn get_target_triple() -> String {
    std::env::var("OVERRIDE_TARGET").unwrap_or_else(|_| target_triple::TARGET.to_string())
}

