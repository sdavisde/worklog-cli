use regex::Regex;
use std::{fmt, sync::LazyLock};

use crate::utils::markdown::MarkdownBlock;

pub static CHECKLIST_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*[-*+]\s+\[([ xX])\]\s+(.+?)\s*$").unwrap());

#[derive(Debug, Clone)]
pub struct Checklist {
    pub items: Vec<(bool, String)>,
}

impl Checklist {
    pub fn new() -> Self {
        Checklist { items: Vec::new() }
    }

    // pub fn add_item(&mut self, item: String) {
    //     self.items.push((false, item));
    // }

    // pub fn toggle_item(&mut self, index: usize) {
    //     if let Some((checked, _)) = self.items.get_mut(index) {
    //         *checked = !*checked;
    //     }
    // }
}

impl fmt::Display for Checklist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self
            .items
            .iter()
            .map(|(checked, item)| {
                let checkbox = if *checked { "[x]" } else { "[ ]" };
                format!("- {} {}", checkbox, item)
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", content)
    }
}

pub fn parse_checklist(lines: &[&str], start_index: usize) -> (MarkdownBlock, usize) {
    let mut items = Vec::new();
    let mut i = start_index;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = CHECKLIST_REGEX.captures(line) {
            let checkbox = &caps[1];
            let checked = checkbox == "x" || checkbox == "X";
            let content = caps[2].to_string();
            items.push((checked, content));
            i += 1;
        } else {
            break;
        }
    }

    (
        MarkdownBlock::Checklist(Checklist { items }),
        i - start_index,
    )
}
