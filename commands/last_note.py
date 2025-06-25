"""
Last note command for worklog CLI tool
"""

from pathlib import Path
from datetime import datetime, date
from utils.date import get_today, format_date
from core.note_manager import note_manager


def last_note():
    """
    Open the most recent daily note that is not today
    
    Returns:
        int: 0 on success, 1 on error
    """
    try:
        # Get the data directory where notes are stored
        data_dir = note_manager.config.data_dir
        
        if not data_dir.exists():
            print("Error: No worklog data directory found")
            return 1
        
        # Get today's date
        today = get_today()
        today_str = today.strftime('%Y-%m-%d')
        
        # Find all markdown files in the data directory
        note_files = list(data_dir.glob('*.md'))
        
        if not note_files:
            print("Error: No daily notes found")
            return 1
        
        # Filter out today's note and sort by date (newest first)
        recent_notes = []
        for note_file in note_files:
            # Extract date from filename (YYYY-MM-DD.md)
            filename = note_file.stem  # removes .md extension
            if filename != today_str:
                try:
                    note_date = datetime.strptime(filename, '%Y-%m-%d').date()
                    recent_notes.append((note_date, note_file))
                except ValueError:
                    # Skip files that don't match the expected date format
                    continue
        
        if not recent_notes:
            print("Error: No previous daily notes found")
            return 1
        
        # Sort by date (newest first) and get the most recent
        recent_notes.sort(key=lambda x: x[0], reverse=True)
        most_recent_date, most_recent_file = recent_notes[0]
        
        print(f"Opening most recent note: {most_recent_date.strftime('%Y-%m-%d')}")
        
        # Open the most recent note - convert date to string format
        date_str = format_date(most_recent_date)
        note_manager.open_note(date_str)
        
        return 0
        
    except Exception as e:
        print(f"Error: {e}")
        return 1 