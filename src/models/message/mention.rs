use serde::{Deserialize, Serialize};

fn is_at_mention_space(c: char) -> bool {
    matches!(c, ' ' | '\u{00a0}')
}

fn remove_at_mentions(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(start) = rest.find("<@!") {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start + 3..];
        let digit_len = after_marker
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .map(char::len_utf8)
            .sum::<usize>();

        if digit_len > 0 && after_marker[digit_len..].starts_with('>') {
            rest = &after_marker[digit_len + 1..];
        } else {
            output.push_str("<@!");
            rest = after_marker;
        }
    }

    output.push_str(rest);
    output
}

/// Parsed command data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CMD {
    pub cmd: String,
    pub content: String,
}

pub fn mention_user(user_id: impl std::fmt::Display) -> String {
    format!("<@{user_id}>")
}

pub fn mention_all_user() -> &'static str {
    "@everyone"
}

pub fn mention_channel(channel_id: impl std::fmt::Display) -> String {
    format!("<#{channel_id}>")
}

pub fn emoji(id: impl std::fmt::Display) -> String {
    format!("<emoji:{id}>")
}

pub fn etl_input(input: &str) -> String {
    remove_at_mentions(input)
        .trim_matches(is_at_mention_space)
        .to_string()
}

pub fn parse_command(input: &str) -> CMD {
    let cleaned = etl_input(input);
    match cleaned.split_once(' ') {
        Some((cmd, content)) => CMD {
            cmd: cmd.trim_matches(is_at_mention_space).to_string(),
            content: content.to_string(),
        },
        None => CMD {
            cmd: cleaned.trim_matches(is_at_mention_space).to_string(),
            content: String::new(),
        },
    }
}
