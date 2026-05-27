//! Constants shared across the QQ Bot API client.

pub const HEADER_TRACE_ID: &str = "X-Tps-trace-ID";
pub const TOKEN_API_URL: &str = "https://bots.qq.com";

#[cfg(test)]
mod tests {
    #[test]
    fn test_constants() {
        assert_eq!(super::HEADER_TRACE_ID, "X-Tps-trace-ID");
        assert_eq!(super::TOKEN_API_URL, "https://bots.qq.com");
    }
}
