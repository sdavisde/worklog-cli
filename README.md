# Worklog CLI Tool (`wl`)

A modular CLI tool for daily task and time tracking with simple Markdown-based notes, designed for future AI querying and summarization.

## 🚀 Quick Start

### Installation

> With either option, `wl` will be added as an executable command on your PATH

#### Option 1: Install from GitHub (Recommended)
```bash
cargo install --git https://github.com/sdavisde/worklog-cli
```

#### Option 2: Build from source
```bash
git clone https://github.com/sdavisde/worklog-cli
cd worklog-cli
cargo build --release
# Binary will be in target/release/worklog-cli
```

### Basic Usage

```bash
# Open today's note in nvim
# TODO: this command should cut all unfinished tasks from the previous note and add them to the current days note
wl open
# or just
wl

# Add a task
wl task "Fix bug in login flow"
# TODO: I put these in "Intake" because i think it'll make sense to do that for work, but it'd be nice for this to be customizable

# View previous day's note
wl last

# Add to ## Notes section (todo)
wl note "Test note"

# Open a specific date (todo)
wl open --date 2025-01-15
```

## ⚙️ Configuration

### Config.yaml

Edit `~/.worklog/config.yaml` to customize the behavior of worklog.

```yaml
# Default Config

editor_command: nvim

# todo: allow configuration of daily note template
```

### Data Storage

Notes are stored in `~/.worklog/daily-notes/` with the format `YYYY-MM-DD.md`.

## 📝 Daily Note Format

Each daily note follows this structure:

```markdown
# 2025-01-15

## Tasks

- [ ] Fix bug in login flow
- [x] Update documentation

### Intake

<!-- Where quick-add tasks are added for later triaging into the tasks section -->
- [ ] Where

## Notes

Meeting with team about new features.
```

## 🧠 AI-Ready Design

The tool is designed with future AI integration in mind:

- **Structured Data**: Consistent markdown format for easy parsing
- **Sectioned Content**: Clear separation of work types
- **Standardized Format**: Predictable structure for LLM processing
- **Interactive Management**: Easy task completion tracking
- **Future Commands**: Ready for `wl summary`, `wl achievements`, etc.

## 🛠️ Development

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
# cargo test

# Run the CLI locally
cargo run -- --help
cargo run -- open
`cargo run -- tasks
```

### Adding New Commands

1. Create a new module in `src/commands/`
2. Implement the command using clap derive macros
3. Add the command to the main CLI enum in `src/main.rs`

### Core Architecture

Built with modern Rust practices:
- **Clap**: Command-line argument parsing with derive macros
- **Chrono**: Date and time handling
- **Serde**: Configuration and data serialization
- **Error handling**: Result types and proper error propagation

## 🚧 Future Features

- [ ] Interactive task management (`wl tasks`)
- [ ] Cross-day summaries (`wl summary --since 30d`)
- [ ] Tagging system (`wl tag AB#12345 priority:high`)
- [ ] Work review (`wl review`)
- [ ] AI-powered insights and suggestions
