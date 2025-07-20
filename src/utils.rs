use std::{fs, path::PathBuf};

use chrono::Local;

pub fn get_today_date() -> String {
    return Local::now().format("%Y-%m-%d").to_string();
}

/**
 * Dynamic fn to add a line to the end of a section in markdown
 * @param file_path: &PathBuf - Path to the markdown file
 * @param new_line: &str - The line to be inserted (e.g., "refactor service to be more efficient")
 * @param target_header: &str - The section to insert the line into (e.g., "### Intake")
 */
pub fn insert_line_in_markdown(file_path: &PathBuf, new_line: &str, target_header: &str) -> String {
    let content = fs::read_to_string(&file_path)
        .expect(&format!("Failed to read contents from {:?}", file_path).to_string());

    // Generate a substring that will serve as our marker for where the target section ends (the header for the next section)
    let target_heading_num = target_header.chars().filter(|c| *c == '#').count();
    let mut stop_section_heading_prefix = '#'.to_string().repeat(target_heading_num - 1);
    stop_section_heading_prefix.push_str(" ");

    let lines: Vec<&str> = content.lines().collect();

    let mut result = Vec::new();
    let mut in_target_section = false;
    let mut target_section_found = false;
    let mut line_inserted = false;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with(target_header) {
            in_target_section = true;
            target_section_found = true;
            result.push(line.to_string());
        } else if in_target_section && line.starts_with(&stop_section_heading_prefix) {
            // Found next section, insert line before this
            result.pop();
            result.push(new_line.to_string());
            result.push("".to_string()); // Ensure blank line after
            result.push(line.to_string());
            in_target_section = false;
            line_inserted = true;
        } else if in_target_section && i == lines.len() - 1 {
            // Last line and still in target section
            result.pop();
            result.push(line.to_string());
            result.push(new_line.to_string());
            result.push("".to_string()); // Ensure blank line after
            line_inserted = true;
        } else {
            result.push(line.to_string());
        }
    }

    // If target section was found but line wasn't inserted yet (e.g., empty section at end of file)
    if target_section_found && in_target_section && !line_inserted {
        result.push(new_line.to_string());
        result.push("".to_string());
    }

    result.join("\n")
}
