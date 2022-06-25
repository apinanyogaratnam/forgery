import json
import os
import sys

from colored import stylize, fg

# get system info
process_id: int = os.getpid()
user: os.uname_result = os.uname()

filename = 'forgefile.json'
initialize_command_key = '.init'

# open file to parse commands
with open(filename) as infile:
    contents = json.load(infile)

# get the init commands to run before each command is run
initialize_commands = contents.get(initialize_command_key, None)

# get the init commands and combine it to run
combined_commands: str = ''
if initialize_commands:
    for command in initialize_commands:
        combined_commands += f'{command}; '

# get the command from the user to run
print(stylize(f'{user.nodename}@{user.sysname}:~$ ', fg('green')), end='')
command_to_execute = input()

# parse and validate the entered command
split_commands: list = command_to_execute.split(' ')

if not split_commands:
    raise Exception('No command provided')

if split_commands[0] != 'forge':
    raise Exception('Invalid command')

sub_command: str = ' '.join(split_commands[1:])

sub_commands_to_execute: list | None = contents.get(sub_command, None)

if not sub_commands_to_execute:
    command: str = f'{combined_commands}{sub_command}'
    os.system(command)
    sys.exit(1)

for sub_command in sub_commands_to_execute:
    os.system(f'{combined_commands}{sub_command}')
