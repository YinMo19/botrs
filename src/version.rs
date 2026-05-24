//! Botgo-compatible SDK version helpers.

pub const SDK_NAME: &str = "BotRS";

pub fn version_string() -> String {
    format!("{SDK_NAME}/v{}", crate::VERSION)
}

#[allow(non_snake_case)]
pub fn String() -> String {
    version_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_botgo_version_string() {
        assert_eq!(super::String(), format!("BotRS/v{}", crate::VERSION));
    }
}
