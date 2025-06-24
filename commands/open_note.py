"""
Open note command for worklog CLI tool
"""

from utils.date import parse_date
from core.note_manager import note_manager


def open_note(date_str=None):
    """
    Open today's note in the configured editor
    
    Args:
        date_str (str): Date string in YYYY-MM-DD format (default: today)
    """
    try:
        date = parse_date(date_str) if date_str else None
        note_manager.open_note(date)
    except ValueError as e:
        print(f"Error: {e}")
        return 1
    
    return 0 