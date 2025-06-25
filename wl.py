#!/usr/bin/env python3
"""
Worklog CLI Tool (wl) - Daily task and time tracking
"""

import click
from datetime import datetime
from pathlib import Path
import sys

# Add the current directory to Python path for imports
sys.path.insert(0, str(Path(__file__).parent))

from commands.open_note import open_note
from commands.start_ticket import start_ticket
from commands.finish_ticket import finish_ticket
from commands.add_task import add_task
from commands.last_note import last_note


@click.group(invoke_without_command=True)
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
@click.pass_context
@click.version_option(version="0.1.0")
def cli(ctx, date):
    """Worklog CLI Tool - Daily task and time tracking"""
    if ctx.invoked_subcommand is None:
        # Default behavior: open today's note
        open_note(date)


@cli.command()
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
def open(date):
    """Open today's note in nvim"""
    open_note(date)


@cli.command()
@click.argument('ticket')
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
def start(ticket, date):
    """Start working on a ticket (add to In Progress)"""
    start_ticket(ticket, date)


@cli.command()
@click.argument('ticket')
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
def fin(ticket, date):
    """Finish working on a ticket (move to Finished Work)"""
    finish_ticket(ticket, date)


@cli.command()
@click.argument('task_description')
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
def task(task_description, date):
    """Add a task to the Tasks section"""
    add_task(task_description, date)


# Alias for open command
@cli.command()
@click.option('--date', default=None, help='Date in YYYY-MM-DD format (default: today)')
def today(date):
    """Open today's note in nvim (alias for open)"""
    open_note(date)


@cli.command()
def last():
    """Open the most recent daily note that is not today"""
    last_note()


if __name__ == '__main__':
    cli() 