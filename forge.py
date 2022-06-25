import os

process_id: int = os.getpid()
user: os.uname = os.uname()

command: str = input(f'{user[1]} {process_id}$ ')

activate_virtual_environment: str = 'source venv/bin/activate'

virtual_command: str = f'{activate_virtual_environment}; {command}'
os.system(virtual_command)
