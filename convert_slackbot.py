#!/usr/bin/env python3

import json
import sys


if len(sys.argv) < 2:
    print('Usage: convert_slackbot.py INPUT_JSON', file=sys.stderr)
    sys.exit(1)

with open(sys.argv[1]) as f:
    responses = json.load(f)['responses']

converted = []

for res in responses:
    converted.append({
        'triggers': res['triggers'],
        'responses': res['responses'],
    })

print(json.dumps({"responses": converted}, ensure_ascii=False, indent='  '))

