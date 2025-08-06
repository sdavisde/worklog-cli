use std::{fmt, fs, path::PathBuf};

pub mod checklist;
pub mod heading;
pub mod ordered_list;
pub mod paragraph;
pub mod unordered_list;

#[derive(Debug, Clone)]
pub enum MarkdownBlock {
    Heading(heading::Heading),
    Paragraph(paragraph::Paragraph),
    UnorderedList(unordered_list::UnorderedList),
    OrderedList(ordered_list::OrderedList),
    Checklist(checklist::Checklist),
}

impl fmt::Display for MarkdownBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownBlock::Heading(heading) => heading.fmt(f),
            MarkdownBlock::Paragraph(paragraph) => paragraph.fmt(f),
            MarkdownBlock::UnorderedList(list) => list.fmt(f),
            MarkdownBlock::OrderedList(list) => list.fmt(f),
            MarkdownBlock::Checklist(checklist) => checklist.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownFile {
    pub blocks: Vec<MarkdownBlock>,
}

impl MarkdownFile {
    pub fn from_path(file_path: &PathBuf) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_path)?;
        Ok(Self::from_string(&content))
    }

    pub fn from_string(content: &str) -> Self {
        let lines: Vec<&str> = content.lines().collect();

        let mut blocks = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines
            if line.is_empty() {
                i += 1;
                continue;
            }

            // Check for heading
            if heading::HEADING_REGEX.is_match(line) {
                if let Some(heading_block) = heading::parse_heading(line) {
                    blocks.push(heading_block);
                } else {
                    // Fallback to paragraph if parsing fails
                    blocks.push(MarkdownBlock::Paragraph(paragraph::Paragraph {
                        content: line.to_string(),
                    }));
                }
                i += 1;
            }
            // Check for checklist (must come before unordered list check)
            else if checklist::CHECKLIST_REGEX.is_match(line) {
                let (checklist_block, consumed) = checklist::parse_checklist(&lines, i);
                blocks.push(checklist_block);
                i += consumed;
            }
            // Check for unordered list
            else if unordered_list::UNORDERED_LIST_REGEX.is_match(line) {
                let (list_block, consumed) = unordered_list::parse_unordered_list(&lines, i);
                blocks.push(list_block);
                i += consumed;
            }
            // Check for ordered list
            else if ordered_list::ORDERED_LIST_REGEX.is_match(line) {
                let (list_block, consumed) = ordered_list::parse_ordered_list(&lines, i);
                blocks.push(list_block);
                i += consumed;
            }
            // Default to paragraph - collect consecutive non-markdown lines
            else {
                let (paragraph_block, consumed) = Self::parse_paragraph(&lines, i);
                blocks.push(paragraph_block);
                i += consumed;
            }
        }

        MarkdownFile { blocks }
    }

    fn parse_paragraph(lines: &[&str], start_index: usize) -> (MarkdownBlock, usize) {
        let mut paragraph_lines = Vec::new();
        let mut i = start_index;

        while i < lines.len() {
            let line = lines[i].trim();

            // Stop if we hit an empty line
            if line.is_empty() {
                break;
            }

            // Stop if we hit a line that looks like other markdown elements
            if heading::HEADING_REGEX.is_match(line)
                || checklist::CHECKLIST_REGEX.is_match(line)
                || unordered_list::UNORDERED_LIST_REGEX.is_match(line)
                || ordered_list::ORDERED_LIST_REGEX.is_match(line)
            {
                break;
            }

            paragraph_lines.push(line);
            i += 1;
        }

        let content = paragraph_lines.join(" ");
        (
            MarkdownBlock::Paragraph(paragraph::Paragraph { content }),
            i - start_index,
        )
    }

    pub fn to_string(&self) -> String {
        self.blocks
            .iter()
            .map(|block| block.to_string())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    // pub fn to_vec(&self) -> Vec<String> {
    //     self.blocks.iter().map(|block| block.to_string()).collect()
    // }

    pub fn set_title(&self, new_title: &str) -> MarkdownFile {
        let new_heading = MarkdownBlock::Heading(heading::Heading::new(1, new_title.to_string()));
        let mut new_blocks = self.blocks.clone();

        // todo: probably a cleaner way to check if the first block is a heading
        if let Some(MarkdownBlock::Heading(_)) = self.blocks.get(0) {
            new_blocks[0] = new_heading;
        } else {
            new_blocks.insert(0, new_heading);
        }

        MarkdownFile { blocks: new_blocks }
    }
}
