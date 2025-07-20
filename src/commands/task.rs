// Tasks should be able to be added to the daily note.

use std::fs;

use crate::daily_note::{create_daily_note_if_not_exists, get_daily_note_path};

pub fn add_task(task: &str) {
    let daily_note_path = get_daily_note_path();
    create_daily_note_if_not_exists(&daily_note_path).expect("Failed to verify daily note exists");
    // 1. Read the daily note file.
    let daily_note_contents =
        fs::read_to_string(&daily_note_path).expect("Failed to read daily note");

    let updated_note_contents = insert_task_in_markdown(&daily_note_contents, &task);

    fs::write(&daily_note_path, updated_note_contents).expect("Failed to save daily note");
}

/**
 * Find the "Intake" section nested under ## Tasks and insert - [ ] {{new_task}} at the end
 */
fn insert_task_in_markdown(content: &str, new_task: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_tasks_section = false;
    let mut tasks_section_found = false;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("### Intake") {
            in_tasks_section = true;
            tasks_section_found = true;
            result.push(line.to_string());
        } else if in_tasks_section && line.starts_with("## ") {
            // Found next H2 section, insert task before this
            result.push(new_task.to_string());
            result.push("".to_string()); // Ensure blank line after
            result.push(line.to_string());
            in_tasks_section = false;
        } else if in_tasks_section && i == lines.len() - 1 {
            // Last line and still in tasks section
            result.push(line.to_string());
            result.push(new_task.to_string());
            result.push("".to_string()); // Ensure blank line after
        } else {
            result.push(line.to_string());
        }
    }

    // If Tasks section was found but no next H2, and we didn't hit end-of-file case
    if tasks_section_found && in_tasks_section && !result.last().map_or(false, |l| l == new_task) {
        result.push(new_task.to_string());
        result.push("".to_string());
    }

    result.join("\n")
}
