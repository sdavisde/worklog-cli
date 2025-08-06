use regex::Regex;
use std::{fmt, sync::LazyLock};

use crate::utils::markdown::MarkdownBlock;

pub static UNORDERED_LIST_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*)[-*+]\s+(.+?)\s*$").unwrap());

#[derive(Debug, Clone, PartialEq)]
pub struct UnorderedListItem {
    pub content: String,
    pub indentation_level: usize,
}

#[derive(Debug, Clone)]
pub struct UnorderedList {
    pub items: Vec<UnorderedListItem>,
}

impl UnorderedList {
    pub fn new() -> Self {
        UnorderedList { items: Vec::new() }
    }
}

impl fmt::Display for UnorderedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self
            .items
            .iter()
            .map(|item| {
                let indent = "  ".repeat(item.indentation_level);
                format!("{}- {}", indent, item.content)
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", content)
    }
}

pub fn parse_unordered_list(lines: &[&str], start_index: usize) -> (MarkdownBlock, usize) {
    let mut items = Vec::new();
    let mut i = start_index;

    while i < lines.len() {
        let line = lines[i];

        if line.trim().is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = UNORDERED_LIST_REGEX.captures(line) {
            let whitespace = &caps[1];
            // Convert tabs to equivalent spaces (1 tab = 4 spaces) and count indentation
            let normalized_whitespace = whitespace.replace('\t', "    ");
            let indentation = normalized_whitespace.len() / 2; // 2 spaces per indentation level
            let content = caps[2].to_string();
            items.push(UnorderedListItem {
                content,
                indentation_level: indentation,
            });
            i += 1;
        } else {
            break;
        }
    }

    (
        MarkdownBlock::UnorderedList(UnorderedList { items }),
        i - start_index,
    )
}
