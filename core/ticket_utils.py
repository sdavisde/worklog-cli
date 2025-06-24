"""
Azure DevOps ticket utilities for worklog CLI tool
"""

import re
from .config import config


def parse_ticket(ticket_str):
    """
    Parse a ticket string (e.g., 'AB#12345') and extract the ID
    
    Args:
        ticket_str (str): Ticket string in format like 'AB#12345'
        
    Returns:
        tuple: (prefix, ticket_id) or (None, None) if invalid format
        
    Examples:
        >>> parse_ticket('AB#12345')
        ('AB', '12345')
        >>> parse_ticket('ABC#67890')
        ('ABC', '67890')
    """
    # Match pattern like AB#12345, ABC#67890, etc.
    pattern = r'^([A-Z]+)#(\d+)$'
    match = re.match(pattern, ticket_str)
    
    if match:
        prefix = match.group(1)
        ticket_id = match.group(2)
        return prefix, ticket_id
    
    return None, None


def format_ticket_link(ticket_str):
    """
    Format a ticket as a markdown link
    
    Args:
        ticket_str (str): Ticket string (e.g., 'AB#12345')
        
    Returns:
        str: Markdown link format or just the ticket string if no ADO config
        
    Examples:
        >>> format_ticket_link('AB#12345')
        '- [AB#12345](https://dev.azure.com/org/project/_workitems/edit/12345)'
    """
    prefix, ticket_id = parse_ticket(ticket_str)
    
    if not prefix or not ticket_id:
        return f"- {ticket_str}"
    
    ado_url = config.get_ado_url(ticket_id)
    
    if ado_url:
        return f"- [{ticket_str}]({ado_url})"
    else:
        return f"- {ticket_str}"


def is_valid_ticket(ticket_str):
    """
    Check if a ticket string is in valid format
    
    Args:
        ticket_str (str): Ticket string to validate
        
    Returns:
        bool: True if valid format, False otherwise
    """
    prefix, ticket_id = parse_ticket(ticket_str)
    return prefix is not None and ticket_id is not None


def extract_ticket_from_line(line):
    """
    Extract ticket information from a markdown line
    
    Args:
        line (str): Markdown line (e.g., '- [AB#12345](url)')
        
    Returns:
        str: Ticket string or None if not found
    """
    # Match markdown link format: [AB#12345](url)
    link_pattern = r'\[([A-Z]+#\d+)\]'
    match = re.search(link_pattern, line)
    
    if match:
        return match.group(1)
    
    # Match plain ticket format: AB#12345
    plain_pattern = r'\b([A-Z]+#\d+)\b'
    match = re.search(plain_pattern, line)
    
    if match:
        return match.group(1)
    
    return None 