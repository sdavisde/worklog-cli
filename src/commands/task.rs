// Tasks should be able to be added to the daily note.

use std::fs;

use crate::{
    commands::daily_note::{create_daily_note_if_not_exists, get_daily_note_path},
    utils::markdown::{MarkdownBlock, checklist::Checklist},
};

pub fn add_task(task: &str, create_fresh: bool) {
    let daily_note_path = get_daily_note_path();
    let mut daily_note = create_daily_note_if_not_exists(&daily_note_path, create_fresh)
        .expect("Failed to verify daily note exists");

    // Look for Intake section first, then Tasks section
    let mut target_heading_index: Option<usize> = None;

    for (index, block) in daily_note.blocks.iter().enumerate() {
        if let MarkdownBlock::Heading(heading) = block {
            if heading.content.contains("Intake") {
                target_heading_index = Some(index);
                break;
            } else if heading.content.contains("Tasks") && target_heading_index.is_none() {
                target_heading_index = Some(index);
            }
        }
    }

    if let Some(heading_index) = target_heading_index {
        let checklist_index = heading_index + 1;
        let checklist_block = daily_note.blocks.get(checklist_index);

        if let Some(MarkdownBlock::Checklist(checklist)) = checklist_block {
            let mut new_checklist = checklist.clone();
            new_checklist.items.push((false, task.to_string()));
            daily_note.blocks[checklist_index] = MarkdownBlock::Checklist(new_checklist);
        } else {
            let mut new_checklist = Checklist::new();
            new_checklist.items.push((false, task.to_string()));
            daily_note
                .blocks
                .insert(checklist_index, MarkdownBlock::Checklist(new_checklist));
        }
    }

    fs::write(&daily_note_path, daily_note.to_string()).expect("Failed to save daily note");
}
