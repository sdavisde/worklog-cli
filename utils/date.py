"""
Date and time utilities for worklog CLI tool
"""

from datetime import datetime, date


def parse_date(date_str):
    """
    Parse a date string in YYYY-MM-DD format
    
    Args:
        date_str (str): Date string in YYYY-MM-DD format
        
    Returns:
        datetime.date: Parsed date object
        
    Raises:
        ValueError: If date string is invalid
    """
    if not date_str:
        return date.today()
    
    try:
        return datetime.strptime(date_str, '%Y-%m-%d').date()
    except ValueError:
        raise ValueError(f"Invalid date format: {date_str}. Expected YYYY-MM-DD")


def format_date(date_obj):
    """
    Format a date object to YYYY-MM-DD string
    
    Args:
        date_obj (datetime.date): Date object to format
        
    Returns:
        str: Formatted date string
    """
    return date_obj.strftime('%Y-%m-%d')


def get_today():
    """Get today's date as a date object"""
    return date.today()


def get_today_str():
    """Get today's date as a formatted string"""
    return format_date(get_today()) 