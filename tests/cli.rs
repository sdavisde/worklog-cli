use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create the template file that the CLI expects
    let template_dir = temp_dir.path().join("templates");
    fs::create_dir_all(&template_dir).unwrap();

    let template_content = r#"# {{DATE}}

## Tasks

### Priority

### Support

### Project Management

### Engineering

### Intake

## Notes
"#;
    fs::write(template_dir.join("daily.md"), template_content).unwrap();

    temp_dir
}

fn run_wl_command(args: &[&str], home_dir: &std::path::Path) -> std::process::Output {
    Command::new("cargo")
        .args(&["run", "--bin", "wl", "--"])
        .args(args)
        .env("HOME", home_dir)
        .output()
        .expect("Failed to execute CLI command")
}

fn get_daily_note_path(home_dir: &std::path::Path) -> std::path::PathBuf {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    home_dir
        .join(".worklog")
        .join("daily_notes")
        .join(format!("{}.md", today))
}

// Basic command tests
#[test]
fn test_wl_task_adds_task() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["task", "Test task"], temp_dir.path());

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();
    assert!(content.contains("- [ ] Test task"));
    assert!(content.contains("### Intake"));
}

#[test]
fn test_wl_note_adds_note() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["note", "Test note"], temp_dir.path());

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();
    assert!(content.contains("- Test note"));
    assert!(content.contains("## Notes"));
}

#[test]
fn test_wl_open_creates_daily_note() {
    let temp_dir = setup_test_env();

    // Create config with echo instead of real editor
    let config_dir = temp_dir.path().join(".worklog");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.yaml"), "editor_command: echo\n").unwrap();

    let output = run_wl_command(&["open"], temp_dir.path());
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    assert!(daily_note_path.exists());

    let content = fs::read_to_string(&daily_note_path).unwrap();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    assert!(content.contains(&format!("# {}", today)));
}

#[test]
fn test_wl_last_opens_most_recent_note() {
    let temp_dir = setup_test_env();

    // Create config with echo instead of real editor
    let config_dir = temp_dir.path().join(".worklog");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.yaml"), "editor_command: echo\n").unwrap();

    // Create some historical daily notes
    let daily_notes_dir = config_dir.join("daily_notes");
    fs::create_dir_all(&daily_notes_dir).unwrap();
    fs::write(daily_notes_dir.join("2023-01-01.md"), "Old note").unwrap();
    fs::write(daily_notes_dir.join("2023-01-02.md"), "Newer note").unwrap();

    let output = run_wl_command(&["last"], temp_dir.path());
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// Edge case tests
#[test]
fn test_wl_task_empty_description() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["task", ""], temp_dir.path());

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cannot add a task without a <description>"));
}

#[test]
fn test_wl_note_empty_description() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["note", ""], temp_dir.path());

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cannot add a note without a <description>"));
}

#[test]
fn test_wl_task_with_quotes() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(
        &["task", "Task with \"quotes\" and 'apostrophes'"],
        temp_dir.path(),
    );

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();
    assert!(content.contains("- [ ] Task with \"quotes\" and 'apostrophes'"));
}

#[test]
fn test_wl_note_with_special_characters() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["note", "Note with symbols: @#$%^&*()"], temp_dir.path());

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();
    assert!(content.contains("- Note with symbols: @#$%^&*()"));
}

#[test]
fn test_wl_multiple_tasks_and_notes() {
    let temp_dir = setup_test_env();

    // Add multiple items
    run_wl_command(&["task", "First task"], temp_dir.path());
    run_wl_command(&["note", "First note"], temp_dir.path());
    run_wl_command(&["task", "Second task"], temp_dir.path());
    run_wl_command(&["note", "Second note"], temp_dir.path());

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();

    // All items should be present
    assert!(content.contains("- [ ] First task"));
    assert!(content.contains("- First note"));
    assert!(content.contains("- [ ] Second task"));
    assert!(content.contains("- Second note"));
}

// CLI flags and help tests
#[test]
fn test_wl_help_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "wl", "--", "--help"])
        .output()
        .expect("Failed to execute CLI command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("task")
            || stdout.contains("note")
            || stdout.contains("open")
            || stdout.contains("last")
    );
}

#[test]
fn test_wl_version_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "wl", "--", "--version"])
        .output()
        .expect("Failed to execute CLI command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("worklog-cli") || stdout.contains("0.1.0"));
}

#[test]
fn test_wl_unknown_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "wl", "--", "unknown"])
        .output()
        .expect("Failed to execute CLI command");

    // Should fail with error
    assert!(!output.status.success());
}

#[test]
fn test_wl_no_command_defaults_to_open() {
    let temp_dir = setup_test_env();

    // Create config with echo instead of real editor
    let config_dir = temp_dir.path().join(".worklog");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.yaml"), "editor_command: echo\n").unwrap();

    let output = run_wl_command(&[], temp_dir.path());
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Should have created the daily note
    let daily_note_path = get_daily_note_path(temp_dir.path());
    assert!(daily_note_path.exists());
}

#[test]
fn test_task_goes_to_intake_section_not_notes() {
    let temp_dir = setup_test_env();
    let output = run_wl_command(&["task", "test task"], temp_dir.path());

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let daily_note_path = get_daily_note_path(temp_dir.path());
    let content = fs::read_to_string(&daily_note_path).unwrap();

    // Find the positions of the sections and the task
    let intake_section_pos = content
        .find("### Intake")
        .expect("Intake section should exist");
    let notes_section_pos = content
        .find("## Notes")
        .expect("Notes section should exist");
    let task_pos = content
        .find("- [ ] test task")
        .expect("Task should be in the file");

    assert!(
        intake_section_pos < task_pos,
        "Task should appear after ### Intake section"
    );
    assert!(
        task_pos < notes_section_pos,
        "Task should appear before ## Notes section"
    );
}
