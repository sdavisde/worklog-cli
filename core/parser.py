"""
Markdown parser for worklog daily notes
"""

import re
from typing import List, Tuple, Optional


class MarkdownParser:
    """Parser for markdown daily notes"""
    
    def __init__(self, content: str):
        self.content = content
        self.lines = content.split('\n')
    
    def find_section(self, section_name: str) -> Tuple[int, int]:
        """
        Find the start and end lines of a section
        
        Args:
            section_name (str): Section name (e.g., 'In Progress', 'Tasks')
            
        Returns:
            tuple: (start_line, end_line) or (-1, -1) if not found
        """
        start_line = -1
        end_line = -1
        
        # Look for section header (## Section Name)
        pattern = rf'^##\s*{re.escape(section_name)}\s*$'
        
        for i, line in enumerate(self.lines):
            if re.match(pattern, line, re.IGNORECASE):
                start_line = i
                break
        
        if start_line == -1:
            return -1, -1
        
        # Find the end of the section (next ## header or end of file)
        for i in range(start_line + 1, len(self.lines)):
            if self.lines[i].startswith('##'):
                end_line = i
                break
        
        if end_line == -1:
            end_line = len(self.lines)
        
        return start_line, end_line
    
    def get_section_content(self, section_name: str) -> List[str]:
        """
        Get the content lines of a section
        
        Args:
            section_name (str): Section name
            
        Returns:
            list: List of content lines (excluding header)
        """
        start_line, end_line = self.find_section(section_name)
        
        if start_line == -1:
            return []
        
        # Get lines after the header, excluding empty lines at the end
        content_lines = self.lines[start_line + 1:end_line]
        
        # Remove trailing empty lines
        while content_lines and not content_lines[-1].strip():
            content_lines.pop()
        
        return content_lines
    
    def cleanup_section_spacing(self, section_name: str) -> None:
        """
        Clean up excessive empty lines in a section
        
        Args:
            section_name (str): Section name to clean up
        """
        start_line, end_line = self.find_section(section_name)
        if start_line == -1:
            return
        
        # Get the section content lines
        section_lines = self.lines[start_line + 1:end_line]
        
        # Remove all empty lines
        cleaned_lines = [line for line in section_lines if line.strip()]
        
        # Replace the section content
        self.lines[start_line + 1:end_line] = cleaned_lines

    def add_to_section(self, section_name: str, new_line: str, deduplicate: bool = True) -> str:
        """
        Add a line to a section, creating the section if it doesn't exist
        
        Args:
            section_name (str): Section name
            new_line (str): Line to add
            deduplicate (bool): Whether to check for duplicates
            
        Returns:
            str: Updated content
        """
        start_line, end_line = self.find_section(section_name)
        
        # Check for duplicates if requested
        if deduplicate:
            existing_content = self.get_section_content(section_name)
            for line in existing_content:
                if line.strip() == new_line.strip():
                    return self.content  # Already exists
        
        # If section doesn't exist, create it at the end
        if start_line == -1:
            # Find the last section or add after the title
            last_section = -1
            for i, line in enumerate(self.lines):
                if line.startswith('##'):
                    last_section = i
            
            insert_pos = last_section + 1 if last_section != -1 else 1
            
            # Insert new section
            self.lines.insert(insert_pos, '')
            self.lines.insert(insert_pos + 1, f'## {section_name}')
            self.lines.insert(insert_pos + 2, '')
            self.lines.insert(insert_pos + 3, new_line)
            self.lines.insert(insert_pos + 4, '')  # Add newline after the inserted line
        else:
            # Clean up existing section spacing first
            self.cleanup_section_spacing(section_name)
            
            # Re-find the section after cleanup
            start_line, end_line = self.find_section(section_name)
            
            # Insert at the end of existing section
            # Check if there's already an empty line at the end of the section
            has_trailing_newline = False
            if end_line > 0 and end_line < len(self.lines):
                has_trailing_newline = not self.lines[end_line - 1].strip()
            
            self.lines.insert(end_line, new_line)
            
            # Only add newline if there isn't already one
            if not has_trailing_newline:
                self.lines.insert(end_line + 1, '')  # Add newline after the inserted line
        
        return '\n'.join(self.lines)
    
    def move_between_sections(self, item: str, from_section: str, to_section: str) -> str:
        """
        Move an item from one section to another
        
        Args:
            item (str): Item to move (can be partial match)
            from_section (str): Source section name
            to_section (str): Destination section name
            
        Returns:
            str: Updated content
        """
        # Find the item in the source section
        from_start, from_end = self.find_section(from_section)
        if from_start == -1:
            return self.content
        
        # Look for the item in the source section
        item_line = None
        item_index = -1
        
        for i in range(from_start + 1, from_end):
            line = self.lines[i].strip()
            if line and item in line:
                item_line = self.lines[i]
                item_index = i
                break
        
        if item_line is None:
            return self.content
        
        # Remove from source section
        self.lines.pop(item_index)
        
        # Add to destination section
        return self.add_to_section(to_section, item_line, deduplicate=False)
    
    def remove_from_section(self, section_name: str, item: str) -> str:
        """
        Remove an item from a section
        
        Args:
            section_name (str): Section name
            item (str): Item to remove (exact match)
            
        Returns:
            str: Updated content
        """
        start_line, end_line = self.find_section(section_name)
        if start_line == -1:
            return self.content
        
        # Find and remove the item (exact match)
        for i in range(start_line + 1, end_line):
            line = self.lines[i].strip()
            if line and line == item:
                self.lines.pop(i)
                
                # Clean up extra empty lines if the removed item was followed by one
                if i < len(self.lines) and not self.lines[i].strip():
                    self.lines.pop(i)
                
                break
        
        return '\n'.join(self.lines)


def parse_markdown(content: str) -> MarkdownParser:
    """
    Create a MarkdownParser instance
    
    Args:
        content (str): Markdown content
        
    Returns:
        MarkdownParser: Parser instance
    """
    return MarkdownParser(content) 