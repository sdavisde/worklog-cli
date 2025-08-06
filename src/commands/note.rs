use std::fs;

use crate::{
    commands::daily_note::{create_daily_note_if_not_exists, get_daily_note_path},
    utils::markdown::{
        MarkdownBlock,
        unordered_list::{UnorderedList, UnorderedListItem},
    },
};

pub fn add_note(note: &str, create_fresh: bool) {
    let daily_note_path = get_daily_note_path();
    let mut daily_note = create_daily_note_if_not_exists(&daily_note_path, create_fresh)
        .expect("Failed to verify daily note exists");

    // Prepend the note passed in with a dash for markdown lists
    let mut note_heading_index: Option<usize> = None;
    daily_note
        .blocks
        .iter_mut()
        .enumerate()
        .for_each(|(index, block)| {
            if let MarkdownBlock::Heading(heading) = block {
                if heading.content.contains("Notes") {
                    // Add the note to the heading block
                    note_heading_index = Some(index);
                }
            }
        });

    if note_heading_index.is_some() {
        let note_list_index = note_heading_index.unwrap() + 1;
        // At this point we have a ## Notes heading in the note and need to append the note to the following list
        let note_list = daily_note.blocks.get(note_list_index);
        if let Some(MarkdownBlock::UnorderedList(note_list)) = note_list {
            let mut new_note_list = note_list.clone();
            new_note_list.items.push(UnorderedListItem {
                content: note.to_string(),
                indentation_level: 0,
            });
            daily_note.blocks[note_list_index] = MarkdownBlock::UnorderedList(new_note_list);
        } else {
            let mut new_note_list_block = UnorderedList::new();
            new_note_list_block.items.push(UnorderedListItem {
                content: note.to_string(),
                indentation_level: 0,
            });
            daily_note.blocks.insert(
                note_list_index,
                MarkdownBlock::UnorderedList(new_note_list_block),
            );
        }
    }
    // If a notes section doesn't exist already, append to end
    // else {
    //     daily_note.blocks.push(MarkdownBlock::Heading("## Notes".to_string()));
    //     daily_note.blocks.push(MarkdownBlock::UnorderedList(vec![new_note_line]));
    // }

    fs::write(&daily_note_path, daily_note.to_string()).expect("Failed to save daily note");
}
