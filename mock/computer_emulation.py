#!/usr/bin/env python3
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

def readInput(all_midi_inputs, debug):
    import time
    import subprocess
    global modifiers_pressed
    current_buffer = set()
    start = time.time()
    while True:
        for midi_input in list(all_midi_inputs):
            if midi_input.poll():
                event = midi_input.read(1)[0]
                note_info = event[0]
                if debug:
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
                    subprocess.run(["ttrack", "rec", "piano/hatel", "3s"]) # TODO: Make a hook or something.
                    modifiers_pressed = set()

                current_buffer.clear()

# start the program!
import pygame.midi
import argparse
import threading
import signal

print("Available channels are:")
pygame.midi.init()

for n in range(pygame.midi.get_count()):
    current_device = pygame.midi.get_device_info(n)
    if current_device[2] == 1:
        print (n,current_device)
        global default_value
        default_value = int(n)
print()

parser = argparse.ArgumentParser(description='Midi to keystrokes')
parser.add_argument('--channel', nargs='?', default=default_value, type=int, help='The correct midi channel.')
parser.add_argument('--debug', action='store_true', help='Logs midi keystroke info to stdout.')
args = parser.parse_args()

pygame.midi.init()
input_thread = threading.Thread(target=lambda: readInput([pygame.midi.Input(args.channel)], args.debug))
input_thread.daemon = True
input_thread.start()

while True:
    signal.pause()
