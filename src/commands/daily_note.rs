use std::{fs, path::PathBuf, process::Command};

use chrono::NaiveDate;

use crate::{
    config::Config,
    utils::{
        markdown::{MarkdownBlock, MarkdownFile},
        time::get_today_date,
    },
};

// todo: eventually, it'd be nice to have a struct/impl like "DailyNote"

pub fn open_daily_note(config: Config, create_fresh: bool) -> Result<String, String> {
    let daily_note_path = get_daily_note_path();
    create_daily_note_if_not_exists(&daily_note_path, create_fresh)?;

    let mut daily_note_cmd = Command::new(&config.editor_command);
    daily_note_cmd.arg(daily_note_path);

    let status = daily_note_cmd
        .status()
        .expect("Failed to fetch status for editor command");
    if !status.success() {
        return Err(format!(
            "Editor command failed with exit code {:?}",
            status.code()
        ));
    }
    return Ok("Success".to_string());
}

pub fn create_daily_note_if_not_exists(
    daily_note_path: &PathBuf,
    create_fresh: bool,
) -> Result<MarkdownFile, String> {
    if daily_note_path.exists() {
        return MarkdownFile::from_path(&daily_note_path).map_err(|e| e.to_string());
    }

    let last_note_path = get_last_daily_note_path();
    let today = get_today_date();

    let note_source = if create_fresh || last_note_path.is_err() {
        from_template_file()?
    } else {
        let last_note_file = MarkdownFile::from_path(&last_note_path.unwrap())
            .map_err(|e| format!("Failed to read last note: {}", e))?;
        last_note_file.filter_completed_tasks().set_title(&today)
    };

    // Replace {{DATE}} with actual date
    let mut note = note_source.clone();
    // update note header by replacing {{ DATE }} with today's date
    if let MarkdownBlock::Heading(header) = &mut note.blocks[0] {
        let mut new_header = header.clone();
        new_header.content = new_header.content.replace("{{DATE}}", &today);
        note.blocks[0] = MarkdownBlock::Heading(new_header);
    }

    // Create parent directory if needed
    if let Some(parent) = daily_note_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Write file
    fs::write(daily_note_path, note.to_string())
        .map_err(|e| format!("Failed to write daily note: {}", e))?;

    Ok(note)
}

pub fn open_last_daily_note(config: Config) -> Result<String, String> {
    let last_note_path = get_last_daily_note_path()?;

    let mut editor_cmd = Command::new(&config.editor_command);
    editor_cmd.arg(&last_note_path);

    let status = editor_cmd
        .status()
        .expect("Failed to fetch status for editor command");

    if !status.success() {
        return Err(format!(
            "Editor command failed with exit code {:?}",
            status.code()
        ));
    }

    Ok(format!(
        "Opened last daily note: {}",
        last_note_path.display()
    ))
}

fn get_last_daily_note_path() -> Result<PathBuf, String> {
    let today_note_path = get_daily_note_path();
    let home = std::env::var("HOME").map_err(|_| "Failed to find HOME env variable")?;
    let daily_notes_dir = PathBuf::from(&home).join(".worklog").join("daily_notes");

    if !daily_notes_dir.exists() {
        return Err("Daily notes directory does not exist".to_string());
    }

    let entries = fs::read_dir(&daily_notes_dir)
        .map_err(|e| format!("Failed to read daily notes directory: {}", e))?;

    let mut valid_dates = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let date_str = path.file_stem().and_then(|stem| stem.to_str());
        if date_str.is_none() {
            continue;
        }
        if today_note_path
            .to_str()
            .unwrap()
            .find(date_str.unwrap())
            .is_some()
        {
            continue;
        }

        let date = NaiveDate::parse_from_str(date_str.unwrap_or_default(), "%Y-%m-%d");

        valid_dates.push((date, path));
    }

    if valid_dates.is_empty() {
        return Err("No daily notes found".to_string());
    }

    valid_dates.sort_by(|a, b| {
        match (&a.0, &b.0) {
            (Ok(date_a), Ok(date_b)) => date_b.cmp(date_a), // reverse for newest first
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
            (Err(_), Err(_)) => std::cmp::Ordering::Equal,
        }
    });
    Ok(valid_dates[0].1.clone())
}

pub fn get_daily_note_path() -> PathBuf {
    let today_date = get_today_date();
    get_note_path(&today_date)
}

fn get_note_path(date: &str) -> PathBuf {
    let home = std::env::var("HOME").expect("Failed to find HOME env variable");

    return PathBuf::from(&home)
        .join(".worklog")
        .join("daily_notes")
        .join(format!("{}.md", date));
}

fn from_template_file() -> Result<MarkdownFile, String> {
    // if the template file does not exist, we should create it with the default template
    if !PathBuf::from("templates/daily.md").exists() {
        fs::write(
            "templates/daily.md",
            "# {{DATE}}

## Tasks

### Priority

### Support

### Project Management

### Engineering

### Intake

## Notes

    ",
        )
        .map_err(|e| format!("Failed to write template: {}", e))?;
    }

    let template = fs::read_to_string("templates/daily.md")
        .map_err(|e| format!("Failed to read template: {}", e))?;
    Ok(MarkdownFile::from_string(&template))
}
