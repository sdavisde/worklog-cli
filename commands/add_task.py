"""
Add task command for worklog CLI tool
"""

from utils.date import parse_date
from core.note_manager import note_manager


def add_task(task_description, date_str=None):
    """
    Add a task to the Tasks section
    
    Args:
        task_description (str): Task description
        date_str (str): Date string in YYYY-MM-DD format (default: today)
    """
    try:
        if not task_description.strip():
            print("Error: Task description cannot be empty")
            return 1
        
        date = parse_date(date_str) if date_str else None
        note_manager.add_task(task_description, date)
        
        print(f"Added task: {task_description}")
        
    except ValueError as e:
        print(f"Error: {e}")
        return 1
    
    return 0 