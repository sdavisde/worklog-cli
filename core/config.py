"""
Configuration management for worklog CLI tool
"""

import os
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables from .env file if it exists
load_dotenv()


class Config:
    """Configuration class for worklog settings"""
    
    def __init__(self):
        self.data_dir = Path.home() / '.worklog'
        self.ado_org = os.getenv('ADO_ORG', '')
        self.ado_project = os.getenv('ADO_PROJECT', '')
        self.editor = os.getenv('EDITOR', 'nvim')
        
        # Ensure data directory exists
        self.data_dir.mkdir(exist_ok=True)
    
    def get_note_path(self, date=None):
        """Get the path for a note file for the given date"""
        from utils.date import parse_date
        
        if date is None:
            from datetime import datetime
            date = datetime.today()
        else:
            date = parse_date(date)
        
        return self.data_dir / f"{date.strftime('%Y-%m-%d')}.md"
    
    def get_template_path(self):
        """Get the path to the daily note template"""
        return Path(__file__).parent.parent / 'templates' / 'daily.md'
    
    def get_ado_url(self, ticket_id):
        """Generate Azure DevOps URL for a ticket"""
        if not self.ado_org or not self.ado_project:
            return None
        
        return f"https://dev.azure.com/{self.ado_org}/{self.ado_project}/_workitems/edit/{ticket_id}"


# Global config instance
config = Config() 