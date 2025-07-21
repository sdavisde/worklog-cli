use std::fs;

use crate::{commands::daily_note::{create_daily_note_if_not_exists, get_daily_note_path}, utils::markdown::insert_line_in_markdown};

pub fn add_note(note: &str) {
    let daily_note_path = get_daily_note_path();
    create_daily_note_if_not_exists(&daily_note_path).expect("Failed to verify daily note exists");

    // Prepend the note passed in with a dash for markdown lists
    let note = format!("- {}", note);
    let updated_note_contents = insert_line_in_markdown(&daily_note_path, &note, "## Notes");

    fs::write(&daily_note_path, updated_note_contents).expect("Failed to save daily note");
}
