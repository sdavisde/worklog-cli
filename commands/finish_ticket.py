"""
Finish ticket command for worklog CLI tool
"""

from utils.date import parse_date
from core.note_manager import note_manager
from core.ticket_utils import is_valid_ticket


def finish_ticket(ticket, date_str=None):
    """
    Finish working on a ticket (move from In Progress to Finished Work)
    
    Args:
        ticket (str): Ticket string (e.g., 'AB#12345')
        date_str (str): Date string in YYYY-MM-DD format (default: today)
    """
    try:
        # Validate ticket format
        if not is_valid_ticket(ticket):
            print(f"Error: Invalid ticket format '{ticket}'. Expected format: AB#12345")
            return 1
        
        date = parse_date(date_str) if date_str else None
        note_manager.move_ticket(ticket, 'In Progress', 'Finished Work', date)
        
        print(f"Finished working on {ticket}")
        
    except ValueError as e:
        print(f"Error: {e}")
        return 1
    
    return 0 