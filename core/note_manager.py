"""
Note management for worklog CLI tool
"""

import subprocess
from pathlib import Path
from .config import config
from .parser import parse_markdown


class NoteManager:
    """Manages daily note files"""
    
    def __init__(self):
        self.config = config
    
    def get_note_path(self, date=None):
        """Get the path for a note file"""
        return self.config.get_note_path(date)
    
    def note_exists(self, date=None):
        """Check if a note file exists"""
        return self.get_note_path(date).exists()
    
    def create_note(self, date=None):
        """
        Create a new note file from template
        
        Args:
            date: Date for the note (default: today)
            
        Returns:
            Path: Path to the created note file
        """
        note_path = self.get_note_path(date)
        template_path = self.config.get_template_path()
        
        # Create from template if it exists
        if template_path.exists():
            template_content = template_path.read_text()
            
            # Replace date placeholder if it exists
            from utils.date import format_date
            if date is None:
                from datetime import datetime
                date = datetime.today()
            
            date_str = format_date(date)
            content = template_content.replace('{{DATE}}', date_str)
            
            note_path.write_text(content)
        else:
            # Create basic template
            from utils.date import format_date
            if date is None:
                from datetime import datetime
                date = datetime.today()
            
            date_str = format_date(date)
            content = f"""# {date_str}

## In Progress

## Finished Work

## Tasks

## Notes

"""
            note_path.write_text(content)
        
        return note_path
    
    def load_note(self, date=None):
        """
        Load a note file, creating it if it doesn't exist
        
        Args:
            date: Date for the note (default: today)
            
        Returns:
            tuple: (content, parser) where content is the file content and parser is a MarkdownParser
        """
        note_path = self.get_note_path(date)
        
        if not note_path.exists():
            self.create_note(date)
        
        content = note_path.read_text()
        parser = parse_markdown(content)
        
        return content, parser
    
    def save_note(self, content, date=None):
        """
        Save content to a note file
        
        Args:
            content (str): Content to save
            date: Date for the note (default: today)
        """
        note_path = self.get_note_path(date)
        note_path.write_text(content)
    
    def open_note(self, date=None):
        """
        Open a note file in the configured editor
        
        Args:
            date: Date for the note (default: today)
        """
        note_path = self.get_note_path(date)
        
        # Ensure note exists
        if not note_path.exists():
            self.create_note(date)
        
        # Open in editor
        try:
            subprocess.run([self.config.editor, str(note_path)], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error opening note: {e}")
        except FileNotFoundError:
            print(f"Editor '{self.config.editor}' not found. Please set EDITOR environment variable.")
    
    def add_ticket_to_section(self, ticket, section_name, date=None):
        """
        Add a ticket to a specific section
        
        Args:
            ticket (str): Ticket string (e.g., 'AB#12345')
            section_name (str): Section name (e.g., 'In Progress')
            date: Date for the note (default: today)
        """
        from .ticket_utils import format_ticket_link
        
        content, parser = self.load_note(date)
        ticket_line = format_ticket_link(ticket)
        
        updated_content = parser.add_to_section(section_name, ticket_line)
        self.save_note(updated_content, date)
    
    def move_ticket(self, ticket, from_section, to_section, date=None):
        """
        Move a ticket between sections
        
        Args:
            ticket (str): Ticket string (e.g., 'AB#12345')
            from_section (str): Source section name
            to_section (str): Destination section name
            date: Date for the note (default: today)
        """
        content, parser = self.load_note(date)
        updated_content = parser.move_between_sections(ticket, from_section, to_section)
        self.save_note(updated_content, date)
    
    def add_task(self, task_description, date=None):
        """
        Add a task to the Tasks section
        
        Args:
            task_description (str): Task description
            date: Date for the note (default: today)
        """
        content, parser = self.load_note(date)
        task_line = f"- [ ] {task_description}"
        
        updated_content = parser.add_to_section('Tasks', task_line)
        self.save_note(updated_content, date)


# Global note manager instance
note_manager = NoteManager() 