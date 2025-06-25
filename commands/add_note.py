"""
Add note command for worklog CLI tool
"""

from utils.date import parse_date
from core.note_manager import note_manager


def add_note(note_text, date_str=None):
    """
    Add a note to the Notes section of the daily note
    
    Args:
        note_text (str): The note text to add
        date_str (str): Date string in YYYY-MM-DD format (default: today)
        
    Returns:
        int: 0 on success, 1 on error
    """
    try:
        date = parse_date(date_str) if date_str else None
        note_manager.add_note(note_text, date)
        return 0
    except ValueError as e:
        print(f"Error: {e}")
        return 1 