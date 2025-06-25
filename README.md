# Worklog CLI Tool (`wl`)

A modular CLI tool for daily task and time tracking with simple Markdown-based notes, designed for future AI querying and summarization.

## 🚀 Quick Start

### Installation

1. **Clone and install dependencies:**

   ```bash
   pip install -r requirements.txt
   ```

2. **Configure Azure DevOps (optional):**

   ```bash
   cp env.example .env
   # Edit .env with your ADO organization and project
   ```

3. **Make it globally accessible (optional):**
   ```bash
   # Add to your PATH or create an alias
   alias wl="python /path/to/worklist/wl.py"
   ```

### Basic Usage

```bash
# Open today's note in nvim
wl open
# or just
wl

# Start working on a ticket
wl start AB#12345

# Finish a ticket (moves from In Progress to Finished Work)
wl fin AB#12345

# Add a task
wl task "Fix bug in login flow"

# Interactive task management
wl tasks

# View previous day's note
wl last

# Add to ## Notes section
wl note "Test note"

# Open a specific date
wl open --date 2025-01-15
```

## 📁 Project Structure

```
worklog/
├── wl.py                 # Main CLI entry point
├── commands/             # Subcommand modules
│   ├── __init__.py
│   ├── open_note.py
│   ├── start_ticket.py
│   ├── finish_ticket.py
│   ├── add_task.py
│   ├── tasks_list.py     # Interactive task management
│   ├── last_note.py
│   └── add_note.py
├── core/                 # Core functionality
│   ├── __init__.py
│   ├── config.py         # Configuration management
│   ├── note_manager.py   # Note file operations
│   ├── ticket_utils.py   # Azure DevOps integration
│   └── parser.py         # Markdown parsing & formatting
├── templates/
│   └── daily.md          # Daily note template
├── utils/
│   ├── __init__.py
│   └── date.py           # Date utilities
├── requirements.txt
├── env.example
└── README.md
```

## ⚙️ Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Azure DevOps Configuration
ADO_ORG=your-organization-name
ADO_PROJECT=your-project-name

# Editor preference (optional, defaults to nvim)
EDITOR=nvim
```

### Data Storage

Notes are stored in `~/.worklog/` with the format `YYYY-MM-DD.md`.

## 📝 Daily Note Format

Each daily note follows this structure:

```markdown
# 2025-01-15

## In Progress

- [AB#12345](https://dev.azure.com/org/project/_workitems/edit/12345)

## Finished Work

- [AB#12346](https://dev.azure.com/org/project/_workitems/edit/12346)

## Tasks

- [ ] Fix bug in login flow
- [x] Update documentation

## Notes

Meeting with team about new features.
```

## 🔧 Commands

### `wl open` or `wl`

Opens today's note in the configured editor (default: nvim).

**Options:**

- `--date`: Specify date in YYYY-MM-DD format

### `wl start <ticket>`

Adds a ticket to the "In Progress" section.

**Examples:**

```bash
wl start AB#12345
wl start ABC#67890
```

### `wl fin <ticket>`

Moves a ticket from "In Progress" to "Finished Work".

### `wl task "<description>"`

Adds a task to the "Tasks" section.

**Examples:**

```bash
wl task "Fix bug in login flow"
wl task "Update API documentation"
```

### `wl tasks`

Interactive task management. Shows a list of all tasks for the day with checkboxes that you can toggle using arrow keys and Enter.

**Features:**

- ☐ Shows incomplete tasks
- ☑ Shows completed tasks
- Use arrow keys to navigate
- Press Enter to toggle task completion
- Automatically saves changes to your daily note
- Clean formatting with no extra spacing

**Options:**

- `--date`: Specify date in YYYY-MM-DD format

**Example:**

```bash
wl tasks
wl tasks --date 2025-01-15
```

### `wl note "<text>"`

Adds a note to the "Notes" section.

### `wl last`

Opens the most recent daily note that is not today.

## 🧠 AI-Ready Design

The tool is designed with future AI integration in mind:

- **Structured Data**: Consistent markdown format for easy parsing
- **Sectioned Content**: Clear separation of work types
- **Standardized Format**: Predictable structure for LLM processing
- **Interactive Management**: Easy task completion tracking
- **Future Commands**: Ready for `wl summary`, `wl achievements`, etc.

## 🛠️ Development

### Adding New Commands

1. Create a new file in `commands/`
2. Implement the command function
3. Add the command to `wl.py`

### Core Modules

- **`config.py`**: Manages settings and paths
- **`note_manager.py`**: Handles file operations
- **`ticket_utils.py`**: Azure DevOps integration
- **`parser.py`**: Markdown parsing & formatting with smart spacing

### Testing

```bash
# Test basic functionality
python wl.py --help
python wl.py open
python wl.py start AB#12345
python wl.py tasks
```

## 🚧 Future Features

- [x] Interactive task management (`wl tasks`)
- [ ] Time tracking (`wl time start|stop`)
- [ ] Cross-day summaries (`wl summary --since 30d`)
- [ ] Tagging system (`wl tag AB#12345 priority:high`)
- [ ] Work review (`wl review`)
- [ ] AI-powered insights and suggestions

## 📄 License

This project is open source. Feel free to contribute!

```

```
