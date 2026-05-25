//! Botgo-compatible OpenAPI request options.

#![allow(non_snake_case)]

/// Concrete OpenAPI options after applying option functions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Options {
    /// Replaces the default endpoint URL for message APIs.
    pub url: std::option::Option<String>,
    /// Adds `hidetip=true` to retract message APIs.
    pub hide_tip: bool,
}

/// Option value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiOption {
    Url(String),
    HideTip,
}

/// Option alias.
pub type Option = OpenApiOption;

impl Options {
    pub fn apply(&mut self, option: OpenApiOption) {
        match option {
            OpenApiOption::Url(url) => self.url = Some(url),
            OpenApiOption::HideTip => self.hide_tip = true,
        }
    }

    pub fn from_options<I, O>(options: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        options
            .into_iter()
            .fold(Self::default(), |mut opts, option| {
                opts.apply(option.into());
                opts
            })
    }
}

impl From<&OpenApiOption> for OpenApiOption {
    fn from(option: &OpenApiOption) -> Self {
        option.clone()
    }
}

/// Replaces the default send URL.
pub fn WithURL(url: impl Into<String>) -> OpenApiOption {
    OpenApiOption::Url(url.into())
}

/// Hides the retract-message tip.
pub fn WithHideTip() -> OpenApiOption {
    OpenApiOption::HideTip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_match_expected_builders() {
        let opts = Options::from_options([WithURL("https://example.com"), WithHideTip()]);
        assert_eq!(opts.url.as_deref(), Some("https://example.com"));
        assert!(opts.hide_tip);
    }
}
