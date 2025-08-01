use regex::Regex;
use std::{fmt, sync::LazyLock};

use crate::utils::markdown::MarkdownBlock;

pub static HEADING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(#{1,6})\s+(.+?)\s*$").unwrap()
});

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u32,
    pub content: String,
}

impl Heading {
    pub fn new(level: u32, content: String) -> Self {
        Heading { level, content }
    }

    pub fn replace(&mut self, new_content: String) {
        self.content = new_content;
    }

}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", "#".repeat(self.level as usize), self.content)
    }
}

pub fn parse_heading(line: &str) -> Option<MarkdownBlock> {
    let caps = HEADING_REGEX.captures(line)?;
    let level = caps[1].len() as u32;
    let content = caps[2].to_string();

    Some(MarkdownBlock::Heading(Heading { level, content }))
}
