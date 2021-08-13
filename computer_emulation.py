#!/usr/bin/env python3
import argparse
import time
from hatel_layout import hatel_layout
from pynput.keyboard import Key, Controller
import subprocess

default_value = None

def print_devices():
    for n in range(pygame.midi.get_count()):
        current_device = pygame.midi.get_device_info(n)
        if current_device[2] == 1:
            print (n,current_device)
            global default_value
            default_value = int(n)

def raw_to_note(raw_number):
    num = (raw_number - 21) % 12
    note_dict = {
            0: 'a',
            2: 'b',
            3: 'c',
            5: 'd',
            7: 'e',
            8: 'f',
            10: 'g',
            1: '0',
            4: '1',
            6: '2',
            9: '3',
            11: '4',
            }
    return note_dict[num]

def buf_to_key(buf):
    l = list(buf)
    l.sort()
    s = "".join(l)
    if s in hatel_layout: return hatel_layout[s]
    else: return None

def readInput(input_device):
    current_buffer = set()
    start = time.time()
    while True:
        if input_device.poll():
            event = input_device.read(1)[0]
            note_info = event[0]
            print(note_info)

            if note_info[2] == 0 or note_info[0] == 128 and note_info[2] == 64:
                if not current_buffer:
                    start = time.time()
                current_buffer.add(raw_to_note(note_info[1]))

        if current_buffer and time.time() - start > .1:
            key = buf_to_key(current_buffer)
            print(key)
            if key:
                if key == Key.shift:
                    keyboard.press(key)
                else:
                    subprocess.run(["ttrack", "rec", "hatel", "3s"]) # TODO: Make a hook or something.
                    keyboard.press(key)
                    keyboard.release(key)
                    if keyboard.shift_pressed:
                        keyboard.release(Key.shift)
            current_buffer.clear()

import pygame.midi
print("Available channels are:")
pygame.midi.init()
print_devices()
print()

parser = argparse.ArgumentParser(description='Midi to keystrokes')
parser.add_argument('--channel', nargs='?', default=default_value, type=int, help='The correct midi channel.')
args = parser.parse_args()

# start the program!
keyboard = Controller()
pygame.midi.init()
my_input = pygame.midi.Input(args.channel)
readInput(my_input)
