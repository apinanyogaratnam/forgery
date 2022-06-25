import os

process_id: int = os.getpid()
user: os.uname = os.uname()

command: str = input(f'{user[1]}$')

os.system(command)
