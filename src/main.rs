extern crate midir;

use std::u8;
use std::sync::Mutex;
use std::fs::OpenOptions;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use hatel::{KeyEmulationType, LAYOUT, NUM_TO_NOTE};
use std::process::Command;
use itertools::concat;
use std::io::Write;

use midir::{MidiInput, MidiInputConnection};
use clap::{Parser};

// Arguments the program accepts.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Method for key input
    #[clap(arg_enum)]
    key_emulation: KeyEmulationType,
}

fn num_to_note(note_num: u8) -> char {
    NUM_TO_NOTE.get(&(note_num % 12)).unwrap().clone()
}

const AFTER_PRESS_WAIT: u64 = 100;

fn main() {
    let args = Args::parse();
    let mut input_map = HashMap::from([]);

    let _scheduler = thread::spawn(move || {
        let wait_time = Duration::from_millis(1000);
        loop {
            refresh_connections(&args.key_emulation, &mut input_map);
            thread::sleep(wait_time);
        }
    });

    thread::park();

    // for midichan in conn_vec.drain(..) {
    //     midichan.connection.close();
    // }
}

struct MidiChannel {
    connection: MidiInputConnection<(String, KeyEmulationType)>
}

fn refresh_connections(key_emulation: &KeyEmulationType, input_map: &mut HashMap<String, MidiChannel>) {
    let mut touched_ports_map = HashMap::new();

    let init_midi = MidiInput::new("midir reading input").unwrap();
    for in_port in init_midi.ports() {
        let local_midi_in = MidiInput::new("midir reading input").unwrap();
        let in_port_name = String::from(&local_midi_in.port_name(&in_port).unwrap());
        touched_ports_map.insert(in_port_name, in_port);
    }

    let prev_ports: HashSet<String> = input_map.keys().cloned().collect();
    let touched_ports: HashSet<String> = touched_ports_map.keys().cloned().collect();
    for x in prev_ports.difference(&touched_ports) {
        println!("Lost Connection: {}", x);
        input_map.remove(x).unwrap().connection.close();
    }

    for port_name in touched_ports.difference(&prev_ports) {
        println!("New Connection: {}", port_name);
        BUFFER_MAP.lock().unwrap().insert(port_name.clone(), HashSet::new());
        ENABLED_MAP.lock().unwrap().insert(port_name.clone(), false);

        let local_midi_in = MidiInput::new("midir reading input").unwrap();
        let in_port = touched_ports_map.get(port_name).unwrap();

        let pair = (port_name.clone(), key_emulation.clone());
        let conn = local_midi_in.connect(in_port, "midi-port", |_, message, closure_pair| {
            midi_listener(closure_pair.1.clone(), &message, closure_pair.0.clone());
        }, pair);

        if conn.is_ok() {
            input_map.insert(port_name.clone(), MidiChannel { connection: conn.unwrap() });
        } else {
            println!("Lost Connection: {}", port_name);
            input_map.remove(port_name).unwrap().connection.close();
        }
    }
}

lazy_static! {
    static ref BUFFER_MAP: Mutex<HashMap<String, HashSet<char>>> = Mutex::new(HashMap::new());
    static ref ENABLED_MAP: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
}

// static map maybe?
fn midi_listener(key_emulation: KeyEmulationType, message: &[u8], port_name: String) {
    // // i want the current buffer (a, c, d, g, 3) & buffer begin time.
    // let millis = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_millis();

    // println!("{}: {:?} (len = {})", stamp, message, message.len());
    if message.len() >= 2 {
        let note = num_to_note(message[1]);
        if message[2] != 0 {
            if BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().len() == 0 {
                BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().insert(note.clone());

                let new_port_name = port_name.clone();
                thread::spawn(move || {
                    let wait_time = Duration::from_millis(AFTER_PRESS_WAIT);
                    thread::sleep(wait_time);

                    let snapshot_of_charset = BUFFER_MAP.lock().unwrap().get(&new_port_name.clone()).unwrap().clone();
                    BUFFER_MAP.lock().unwrap().get_mut(&new_port_name.clone()).unwrap().clear();
                    let layout_str = chars_to_layout_str(&snapshot_of_charset);
                    let function_pressed = snapshot_of_charset.contains(&'1');
                    let modifiers = get_modifiers(&key_emulation, &snapshot_of_charset);

                    if *ENABLED_MAP.lock().unwrap().get(&new_port_name.clone()).unwrap() {
                        // call hatel
                        if layout_str.len() >= 5 {
                            ENABLED_MAP.lock().unwrap().insert(new_port_name.clone(), false);
                            call_ttrack("piano/hatel", "1s");
                        } else {
                            simulate_keyboard_press(&key_emulation, function_pressed, modifiers, layout_str);
                            call_ttrack("piano/hatel", "3s");
                        }
                    } else if modifiers.len() == 4 && function_pressed {
                        ENABLED_MAP.lock().unwrap().insert(new_port_name.clone(), true);
                        call_ttrack("piano/midi", "1s");
                    } else {
                        call_ttrack("piano/midi", "3s");
                    }
                });
            } else {
                BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().insert(note.clone());
            }

            // println!("{}: {}, {}, {}", note, LAYOUT.get(&note.to_string().as_str()).unwrap_or(&"invalid"), millis, stamp);
        }
    }
}

fn get_modifiers(key_emulation: &KeyEmulationType, chars: &HashSet<char>) -> Vec<String> {
    let mut modifiers = Vec::new();
    let mut sorted_chars: Vec<&char> = chars.into_iter().collect();
    sorted_chars.sort();

    for character in &sorted_chars {
        if **character == '0' || (**character >= '2' && **character <= '4') {
            match LAYOUT.get(character.to_string().as_str()) {
                Some(key) => {
                    let val = String::from(key[*key_emulation as usize]);
                    if val.len() != 0 {
                        modifiers.push(val);
                    }
                },
                None => { }
            }
        }
    }
    return modifiers;
}

fn chars_to_layout_str(chars: &HashSet<char>) -> String {
    let mut layout_chars = Vec::new();

    for character in chars {
        if *character >= 'a' && *character <= 'g' {
            layout_chars.push(character.clone());
        }
    }

    layout_chars.sort();

    return layout_chars.into_iter().collect();
}

fn simulate_keyboard_press(key_emulation: &KeyEmulationType, _function_pressed: bool, modifiers: Vec<String>, layout_str: String) {
    match LAYOUT.get(layout_str.as_str()) {
        Some(key) => {
            match *key_emulation {
                KeyEmulationType::Echo => {
                    let mut _cmd = Command::new("echo");
                    _cmd.arg("key: ").arg(key[*key_emulation as usize]);

                    if modifiers.len() > 0 {
                        _cmd.arg(format!("-- ({})", modifiers.join(", ")));
                    }

                    _cmd.spawn().expect("echo failed");
                },
                KeyEmulationType::Xdotool => {
                    Command::new("xdotool")
                        .arg("key")
                        .arg(concat([modifiers, vec![String::from(key[*key_emulation as usize])]]).join(&String::from("+")))
                        .spawn()
                        .expect("xdotool failed");
                },
                KeyEmulationType::Osascript => {
                    let mut _cmd = Command::new("osascript");
                    _cmd.arg("-e");

                    let argstr = format!("tell application \"System Events\" to key code {}", key[*key_emulation as usize]);

                    let argstr = if modifiers.len() == 1 {
                        format!("{} using {} down", argstr, modifiers.join(""))
                    } else if modifiers.len() > 1 {
                        format!("{} using {{{} down}}", argstr, modifiers.join(" down, "))
                    } else {
                        argstr
                    };

                    _cmd.arg(argstr).spawn().expect("osascript failed");
                },
                KeyEmulationType::Pi => {
                    let mut buffer: [u8; 8] = [0; 8];
                    for modifier in modifiers {
                        if modifier.len() > 0 {
                            buffer[0] = buffer[0] | u8::from_str_radix(modifier.as_str(), 16).unwrap();
                        }
                    }

                    let key_str = key[*key_emulation as usize];
                    if key_str.len() > 0 {
                        buffer[2] = u8::from_str_radix(key_str, 16).unwrap();
                    }

                    let mut file = OpenOptions::new().write(true).open("/dev/hidg0").unwrap();
                    file.write_all(&buffer).expect("failed to write to hid device"); // write key
                    file.write_all(&[0; 8]).expect("failed to write to hid device"); // release keys
                },
            }
        },
        None      => {},
    }
}

// Temporary until we have hooks
fn call_ttrack(group: &'static str, duration: &'static str) {
    Command::new("ttrack")
        .arg("rec")
        .arg(group)
        .arg(duration)
        .spawn()
        .expect("ttrack failed");
}
