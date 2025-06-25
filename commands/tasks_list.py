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
        
        # Load the note
        content, parser = note_manager.load_note(date)
        
        # Get tasks from the Tasks section
        task_lines = parser.get_section_content('Tasks')
        
        if not task_lines:
            print("No tasks found for this date.")
            return 0
        
        # Parse tasks and their completion status
        tasks = []
        for line in task_lines:
            line = line.strip()
            if line.startswith('- ['):
                # Extract completion status and description
                if line.startswith('- [x]') or line.startswith('- [X]'):
                    completed = True
                    description = line[5:].strip()
                elif line.startswith('- [ ]'):
                    completed = False
                    description = line[5:].strip()
                else:
                    # Handle other formats (like "- ] task" from add_task)
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
        
        # Add exit option
        choices.append({
            'name': "Exit",
            'value': -1
        })
        
        while True:
            # Show the task list
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
            task['completed'] = not task['completed']
            
            # Update the task line
            if task['completed']:
                new_line = f"- [x] {task['description']}"
            else:
                new_line = f"- [ ] {task['description']}"
            
            # Update the content
            updated_content = parser.remove_from_section('Tasks', task['line'])
            updated_parser = parse_markdown(updated_content)
            final_content = updated_parser.add_to_section('Tasks', new_line, deduplicate=False)
            
            # Save the updated note
            note_manager.save_note(final_content, date)
            
            # Update the choices for next iteration
            status = "☑" if task['completed'] else "☐"
            choices[selected]['name'] = f"{status} {task['description']}"
            
            # Update the task object
            task['line'] = new_line
            
            print(f"Task '{task['description']}' marked as {'completed' if task['completed'] else 'incomplete'}")
        
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