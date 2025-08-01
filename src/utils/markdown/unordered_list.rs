use regex::Regex;
use std::{fmt, sync::LazyLock};

use crate::utils::markdown::MarkdownBlock;

pub static UNORDERED_LIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*[-*+]\s+(.+)$").unwrap()
});


#[derive(Debug, Clone)]
pub struct UnorderedList {
    pub items: Vec<String>,
}

impl UnorderedList {
    pub fn new() -> Self {
        UnorderedList { items: Vec::new() }
    }

}

impl fmt::Display for UnorderedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self.items.iter()
            .map(|item| format!("- {}", item))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", content)
    }
}

pub fn parse_unordered_list(lines: &[&str], start_index: usize) -> (MarkdownBlock, usize) {
    let mut items = Vec::new();
    let mut i = start_index;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = UNORDERED_LIST_REGEX.captures(line) {
            items.push(caps[1].to_string());
            i += 1;
        } else {
            break;
        }
    }

    (MarkdownBlock::UnorderedList(UnorderedList { items }), i - start_index)
}
