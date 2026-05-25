//! Botgo-compatible constants.

#![allow(non_upper_case_globals)]

pub const HeaderTraceID: &str = "X-Tps-trace-ID";
pub const APIDomain: &str = crate::DEFAULT_API_URL;
pub const SandBoxAPIDomain: &str = crate::SANDBOX_API_URL;
pub const TokenDomain: &str = "https://bots.qq.com";

#[cfg(test)]
mod tests {
    #[test]
    fn test_constants() {
        assert_eq!(super::HeaderTraceID, "X-Tps-trace-ID");
        assert_eq!(super::APIDomain, crate::DEFAULT_API_URL);
        assert_eq!(super::SandBoxAPIDomain, crate::SANDBOX_API_URL);
        assert_eq!(super::TokenDomain, "https://bots.qq.com");
    }
}
