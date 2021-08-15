#!/usr/bin/env python3
default_value = None
modifiers_pressed = set()

def preexec_function():
    import signal
    signal.signal(signal.SIGINT, signal.SIG_IGN)

def press_modifier(key):
    global modifiers_pressed
    if key == "shift" or key == "ctrl" or key == "super" or key == "alt":
        modifiers_pressed.add(key)
        return True
    return False

def print_devices():
    for n in range(pygame.midi.get_count()):
        current_device = pygame.midi.get_device_info(n)
        if current_device[2] == 1:
            print (n,current_device)
            global default_value
            default_value = int(n)

def raw_to_note(raw_number):
    num = (raw_number - 21) % 12
    note_dict = { 0: 'a', 2: 'b', 3: 'c', 5: 'd', 7: 'e', 8: 'f', 10: 'g', 1: '0', 4: '1', 6: '2', 9: '3', 11: '4', }
    return note_dict[num]

def buf_to_key(buf):
    from hatel_layout import hatel_layout
    l = list(buf)
    l.sort()
    s = "".join(l)
    if s in hatel_layout: return hatel_layout[s]
    else: return None

def readInput(input_device):
    import time
    import subprocess
    global modifiers_pressed
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

            if key and not press_modifier(key):
                l = list(modifiers_pressed)
                l.append(key)
                subprocess.Popen(["xdotool", "key", "+".join(l)], preexec_fn=preexec_function)
                subprocess.run(["ttrack", "rec", "hatel", "3s"]) # TODO: Make a hook or something.
                modifiers_pressed = set()

            current_buffer.clear()

import pygame.midi
print("Available channels are:")
pygame.midi.init()
print_devices()
print()

import argparse
parser = argparse.ArgumentParser(description='Midi to keystrokes')
parser.add_argument('--channel', nargs='?', default=default_value, type=int, help='The correct midi channel.')
args = parser.parse_args()

# start the program!
pygame.midi.init()
my_input = pygame.midi.Input(args.channel)
readInput(my_input)
