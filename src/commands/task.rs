// Tasks should be able to be added to the daily note.

use std::fs;

use crate::{daily_note::{create_daily_note_if_not_exists, get_daily_note_path}, utils::insert_line_in_markdown};

pub fn add_task(task: &str) {
    let daily_note_path = get_daily_note_path();
    create_daily_note_if_not_exists(&daily_note_path).expect("Failed to verify daily note exists");

    let task = format!("- [ ] {}", task);
    let updated_note_contents = insert_line_in_markdown(&daily_note_path, &task, "### Intake");

    fs::write(&daily_note_path, updated_note_contents).expect("Failed to save daily note");
}
