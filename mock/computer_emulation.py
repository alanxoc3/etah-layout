#!/usr/bin/env python3
def preexec_function():
    import signal
    signal.signal(signal.SIGINT, signal.SIG_IGN)

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
    elif len(s) >= 5 and s.isalpha(): return "disable"
    else: return None

def readInput(all_midi_inputs, debug):
    import time
    import subprocess
    modifiers_pressed = set()
    hatel_enabled = False
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
                current_buffer.clear()
                ttrack_duration = "3s"

                if key == "enable":
                    ttrack_duration = "1s"
                    hatel_enabled = True
                    modifiers_pressed = set()
                elif key == "disable":
                    ttrack_duration = "1s"
                    hatel_enabled = False
                    modifiers_pressed = set()
                elif hatel_enabled and key is not None:
                    if key == "shift" or key == "ctrl" or key == "super" or key == "alt":
                        modifiers_pressed.add(key)
                    else:
                        l = list(modifiers_pressed)
                        l.append(key)

                        if key == "enable" or hatel_enabled:
                            subprocess.Popen(["xdotool", "key", "+".join(l)], preexec_fn=preexec_function)

                        modifiers_pressed = set()

                # TODO: Make a hook or something.
                subprocess.run(["ttrack", "rec", "piano/" + ("hatel" if hatel_enabled else "midi"), ttrack_duration])

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
