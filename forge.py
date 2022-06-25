import json
import os

with open('forgefile.json', 'w') as infile:
    contents = json.loads(infile)

for key, value in contents.items():
    print('key', key, 'value', value)
