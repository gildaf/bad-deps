pub fn get_string() -> String {
    if cfg!(feature = "test") {
        "a test".to_string()
    } else {
        "not a test".to_string()
    }
}
