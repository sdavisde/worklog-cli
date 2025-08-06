use std::fs;
use tempfile::NamedTempFile;

use worklog_cli::utils::markdown::unordered_list::UnorderedListItem;
use worklog_cli::utils::markdown::*;

#[test]
fn test_parse_single_heading() {
    let content = "# Main Title\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 1);
            assert_eq!(heading.content, "Main Title");
        }
        _ => panic!("Expected heading block"),
    }
}

#[test]
fn test_parse_multiple_headings() {
    let content = "# Main Title\n## Subtitle\n### Sub-subtitle\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 3);

    match &file.blocks[0] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 1);
            assert_eq!(heading.content, "Main Title");
        }
        _ => panic!("Expected heading block"),
    }

    match &file.blocks[1] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 2);
            assert_eq!(heading.content, "Subtitle");
        }
        _ => panic!("Expected heading block"),
    }

    match &file.blocks[2] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 3);
            assert_eq!(heading.content, "Sub-subtitle");
        }
        _ => panic!("Expected heading block"),
    }
}

#[test]
fn test_parse_unordered_list() {
    let content = "- First item\n- Second item\n* Third item\n+ Fourth item\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::UnorderedList(list) => {
            assert_eq!(list.items.len(), 4);
            assert_eq!(
                list.items[0],
                UnorderedListItem {
                    content: "First item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[1],
                UnorderedListItem {
                    content: "Second item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[2],
                UnorderedListItem {
                    content: "Third item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[3],
                UnorderedListItem {
                    content: "Fourth item".to_string(),
                    indentation_level: 0
                }
            );
        }
        _ => panic!("Expected unordered list block"),
    }
}

#[test]
fn test_parse_ordered_list() {
    let content = "1. First item\n2. Second item\n3. Third item\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::OrderedList(list) => {
            assert_eq!(list.items.len(), 3);
            assert_eq!(list.items[0], "First item");
            assert_eq!(list.items[1], "Second item");
            assert_eq!(list.items[2], "Third item");
        }
        _ => panic!("Expected ordered list block"),
    }
}

#[test]
fn test_parse_checklist() {
    let content = "- [ ] Todo item\n- [x] Completed item\n- [ ] Another todo\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Checklist(checklist) => {
            assert_eq!(checklist.items.len(), 3);
            assert_eq!(checklist.items[0], (false, "Todo item".to_string()));
            assert_eq!(checklist.items[1], (true, "Completed item".to_string()));
            assert_eq!(checklist.items[2], (false, "Another todo".to_string()));
        }
        _ => panic!("Expected checklist block"),
    }
}

#[test]
fn test_parse_paragraph() {
    let content = "This is a simple paragraph.\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(paragraph.content, "This is a simple paragraph.");
        }
        _ => panic!("Expected paragraph block"),
    }
}

#[test]
fn test_parse_mixed_content() {
    let content = r#"# Main Title

This is a paragraph.

## Tasks

- [ ] First task
- [x] Completed task

## Shopping List

- Milk
- Bread
- Eggs

## Priorities

1. High priority
2. Medium priority
3. Low priority

Another paragraph at the end.
"#;

    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    // Should be 9 blocks: heading + paragraph + heading + checklist + heading + list + heading + list + paragraph
    assert_eq!(file.blocks.len(), 9);

    // Check main title
    match &file.blocks[0] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 1);
            assert_eq!(heading.content, "Main Title");
        }
        _ => panic!("Expected heading block at index 0"),
    }

    // Check paragraph
    match &file.blocks[1] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(paragraph.content, "This is a paragraph.");
        }
        _ => panic!("Expected paragraph block at index 1"),
    }

    // Check tasks heading
    match &file.blocks[2] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 2);
            assert_eq!(heading.content, "Tasks");
        }
        _ => panic!("Expected heading block at index 2"),
    }

    // Check checklist
    match &file.blocks[3] {
        MarkdownBlock::Checklist(checklist) => {
            assert_eq!(checklist.items.len(), 2);
            assert_eq!(checklist.items[0], (false, "First task".to_string()));
            assert_eq!(checklist.items[1], (true, "Completed task".to_string()));
        }
        _ => panic!("Expected checklist block at index 3"),
    }

    // Check shopping list heading
    match &file.blocks[4] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 2);
            assert_eq!(heading.content, "Shopping List");
        }
        _ => panic!("Expected heading block at index 4"),
    }

    // Check unordered list
    match &file.blocks[5] {
        MarkdownBlock::UnorderedList(list) => {
            assert_eq!(list.items.len(), 3);
            assert_eq!(
                list.items[0],
                UnorderedListItem {
                    content: "Milk".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[1],
                UnorderedListItem {
                    content: "Bread".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[2],
                UnorderedListItem {
                    content: "Eggs".to_string(),
                    indentation_level: 0
                }
            );
        }
        _ => panic!("Expected unordered list block at index 5"),
    }

    // Check priorities heading
    match &file.blocks[6] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 2);
            assert_eq!(heading.content, "Priorities");
        }
        _ => panic!("Expected heading block at index 6"),
    }

    // Check ordered list
    match &file.blocks[7] {
        MarkdownBlock::OrderedList(list) => {
            assert_eq!(list.items.len(), 3);
            assert_eq!(list.items[0], "High priority");
            assert_eq!(list.items[1], "Medium priority");
            assert_eq!(list.items[2], "Low priority");
        }
        _ => panic!("Expected ordered list block at index 7"),
    }

    // Check final paragraph
    match &file.blocks[8] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(paragraph.content, "Another paragraph at the end.");
        }
        _ => panic!("Expected paragraph block at index 8"),
    }
}

#[test]
fn test_parse_empty_file() {
    let content = "";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 0);
}

#[test]
fn test_parse_whitespace_only() {
    let content = "   \n\n  \n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 0);
}

#[test]
fn test_list_with_empty_lines() {
    let content = "- First item\n\n- Second item\n- Third item\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::UnorderedList(list) => {
            assert_eq!(list.items.len(), 3);
            assert_eq!(
                list.items[0],
                UnorderedListItem {
                    content: "First item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[1],
                UnorderedListItem {
                    content: "Second item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[2],
                UnorderedListItem {
                    content: "Third item".to_string(),
                    indentation_level: 0
                }
            );
        }
        _ => panic!("Expected unordered list block"),
    }
}

#[test]
fn test_checklist_with_empty_lines() {
    let content = "- [ ] First task\n\n- [x] Completed task\n\n- [ ] Another task\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Checklist(checklist) => {
            assert_eq!(checklist.items.len(), 3);
            assert_eq!(checklist.items[0], (false, "First task".to_string()));
            assert_eq!(checklist.items[1], (true, "Completed task".to_string()));
            assert_eq!(checklist.items[2], (false, "Another task".to_string()));
        }
        _ => panic!("Expected checklist block"),
    }
}

#[test]
fn test_parse_nested_unordered_list() {
    let content =
        "- Top level item\n  - Nested item\n    - Double nested item\n- Another top level item\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::UnorderedList(list) => {
            assert_eq!(list.items.len(), 4);
            assert_eq!(
                list.items[0],
                UnorderedListItem {
                    content: "Top level item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[1],
                UnorderedListItem {
                    content: "Nested item".to_string(),
                    indentation_level: 1
                }
            );
            assert_eq!(
                list.items[2],
                UnorderedListItem {
                    content: "Double nested item".to_string(),
                    indentation_level: 2
                }
            );
            assert_eq!(
                list.items[3],
                UnorderedListItem {
                    content: "Another top level item".to_string(),
                    indentation_level: 0
                }
            );
        }
        _ => panic!("Expected unordered list block"),
    }
}

#[test]
fn test_parse_multiline_paragraph() {
    let content = "This is the first line.\nThis is the second line.\nThis is the third line.\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(
                paragraph.content,
                "This is the first line. This is the second line. This is the third line."
            );
        }
        _ => panic!("Expected paragraph block"),
    }
}

#[test]
fn test_parse_paragraphs_separated_by_empty_lines() {
    let content = "First paragraph line one.\nFirst paragraph line two.\n\nSecond paragraph line one.\nSecond paragraph line two.\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 2);

    match &file.blocks[0] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(
                paragraph.content,
                "First paragraph line one. First paragraph line two."
            );
        }
        _ => panic!("Expected first paragraph block"),
    }

    match &file.blocks[1] {
        MarkdownBlock::Paragraph(paragraph) => {
            assert_eq!(
                paragraph.content,
                "Second paragraph line one. Second paragraph line two."
            );
        }
        _ => panic!("Expected second paragraph block"),
    }
}

#[test]
fn test_parse_heading_with_extra_whitespace() {
    let content = "   # Main Title   \n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Heading(heading) => {
            assert_eq!(heading.level, 1);
            assert_eq!(heading.content, "Main Title");
        }
        _ => panic!("Expected heading block"),
    }
}

#[test]
fn test_parse_checklist_with_uppercase_x() {
    let content =
        "- [ ] Todo item\n- [X] Completed with uppercase X\n- [x] Completed with lowercase x\n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::Checklist(checklist) => {
            assert_eq!(checklist.items.len(), 3);
            assert_eq!(checklist.items[0], (false, "Todo item".to_string()));
            assert_eq!(
                checklist.items[1],
                (true, "Completed with uppercase X".to_string())
            );
            assert_eq!(
                checklist.items[2],
                (true, "Completed with lowercase x".to_string())
            );
        }
        _ => panic!("Expected checklist block"),
    }
}

#[test]
fn test_parse_list_with_trailing_whitespace() {
    let content = "- First item   \n- Second item\t\n* Third item   \t   \n";
    let temp_file = create_temp_file(content);
    let file = MarkdownFile::from_path(&temp_file.path().to_path_buf()).unwrap();

    assert_eq!(file.blocks.len(), 1);
    match &file.blocks[0] {
        MarkdownBlock::UnorderedList(list) => {
            assert_eq!(list.items.len(), 3);
            assert_eq!(
                list.items[0],
                UnorderedListItem {
                    content: "First item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[1],
                UnorderedListItem {
                    content: "Second item".to_string(),
                    indentation_level: 0
                }
            );
            assert_eq!(
                list.items[2],
                UnorderedListItem {
                    content: "Third item".to_string(),
                    indentation_level: 0
                }
            );
        }
        _ => panic!("Expected unordered list block"),
    }
}

fn create_temp_file(content: &str) -> NamedTempFile {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(temp_file.path(), content).expect("Failed to write to temp file");
    temp_file
}
