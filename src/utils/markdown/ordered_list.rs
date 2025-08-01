use regex::Regex;
use std::{fmt, sync::LazyLock};

use crate::utils::markdown::MarkdownBlock;

pub static ORDERED_LIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(\d+)\.\s+(.+)$").unwrap()
});

#[derive(Debug, Clone)]
pub struct OrderedList {
    pub items: Vec<String>,
}

impl fmt::Display for OrderedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self.items.iter()
            .enumerate()
            .map(|(i, item)| format!("{}. {}", i + 1, item))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", content)
    }
}

pub fn parse_ordered_list(lines: &[&str], start_index: usize) -> (MarkdownBlock, usize) {
    let mut items = Vec::new();
    let mut i = start_index;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = ORDERED_LIST_REGEX.captures(line) {
            items.push(caps[2].to_string());
            i += 1;
        } else {
            break;
        }
    }

    (MarkdownBlock::OrderedList(OrderedList { items }), i - start_index)
}
