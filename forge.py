import json
import os

filename = 'forgefile.json'
initialize_command_key = '.init'

with open(filename) as infile:
    contents = json.load(infile)

initialize_commands = contents.get(initialize_command_key, None)

if initialize_commands:
    for command in initialize_commands:
        print('command', command)
        # os.system(command)


