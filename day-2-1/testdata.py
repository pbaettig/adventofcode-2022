#!/usr/bin/env python3

from random import choice

shapes_me = 'XYZ'
shapes_opponent = 'ABC'

while True:
    print(f'{choice(shapes_opponent)} {choice(shapes_me)}')