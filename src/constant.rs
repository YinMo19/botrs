//! Constants shared across the QQ Bot API client.

pub(crate) const TOKEN_API_URL: &str = "https://bots.qq.com";

#[cfg(test)]
mod tests {
    #[test]
    fn test_constants() {
        assert_eq!(super::TOKEN_API_URL, "https://bots.qq.com");
    }
}
