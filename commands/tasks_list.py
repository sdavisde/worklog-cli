"""
Tasks command for worklog CLI tool - Interactive task management
"""

import questionary
from utils.date import parse_date
from core.note_manager import note_manager
from core.parser import parse_markdown


def tasks_list(date_str=None):
    """
    Show interactive task list for the specified date
    
    Args:
        date_str (str): Date string in YYYY-MM-DD format (default: today)
        
    Returns:
        int: 0 on success, 1 on error
    """
    try:
        date = parse_date(date_str) if date_str else None
        
        while True:
            # Load the note and get tasks
            content, parser = note_manager.load_note(date)
            task_lines = parser.get_section_content('Tasks')
            
            if not task_lines:
                print("No tasks found for this date.")
                return 0
            
            # Parse tasks and their completion status
            tasks = []
            for line in task_lines:
                line = line.strip()
                if line.startswith('- ['):
                    if line.startswith('- [x]') or line.startswith('- [X]'):
                        completed = True
                        description = line[5:].strip()
                    elif line.startswith('- [ ]'):
                        completed = False
                        description = line[5:].strip()
                    else:
                        completed = False
                        description = line[3:].strip()
                    tasks.append({
                        'line': line,
                        'completed': completed,
                        'description': description
                    })
            
            if not tasks:
                print("No tasks found for this date.")
                return 0
            
            # Create interactive choices
            choices = []
            for i, task in enumerate(tasks):
                status = "☑" if task['completed'] else "☐"
                choices.append({
                    'name': f"{status} {task['description']}",
                    'value': i
                })
            choices.append({'name': "Exit", 'value': -1})
            
            date_display = date.strftime('%Y-%m-%d') if date else 'today'
            print(f"\nTasks for {date_display}:\n")
            selected = questionary.select(
                "Select a task to toggle:",
                choices=choices
            ).ask()
            
            if selected == -1:
                break
            
            # Toggle the selected task
            task = tasks[selected]
            new_completed = not task['completed']
            new_line = f"- [x] {task['description']}" if new_completed else f"- [ ] {task['description']}"
            
            # Remove the old line and add the new one
            updated_content = parser.remove_from_section('Tasks', task['line'])
            updated_parser = parse_markdown(updated_content)
            final_content = updated_parser.add_to_section('Tasks', new_line, deduplicate=False)
            note_manager.save_note(final_content, date)
            
            print(f"Task '{task['description']}' marked as {'completed' if new_completed else 'incomplete'}")
        
        return 0
        
    except ValueError as e:
        print(f"Error: {e}")
        return 1
    except KeyboardInterrupt:
        print("\nExiting...")
        return 0
    except Exception as e:
        print(f"Error: {e}")
        return 1 