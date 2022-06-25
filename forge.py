import json
import os

filename = 'forgefile.json'

with open(filename) as infile:
    contents = json.load(infile)

for key, value in contents.items():
    print('key', key, 'value', value)
