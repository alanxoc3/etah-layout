extern crate midir;

use std::sync::Mutex;
use lazy_static::lazy_static;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::collections::HashSet;
use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use hatel::LAYOUT;
use hatel::NUM_TO_NOTE;
use std::process::Command;
use itertools::concat;

use midir::{MidiInput, MidiInputConnection};

fn num_to_note(note_num: u8) -> char {
    NUM_TO_NOTE.get(&(note_num % 12)).unwrap().clone()
}

const AFTER_PRESS_WAIT: u64 = 100;

fn main() {
    let mut input_map = HashMap::from([]);

    let _scheduler = thread::spawn(move || {
        let wait_time = Duration::from_millis(1000);
        loop {
            refresh_connections(&mut input_map);
            thread::sleep(wait_time);
        }
    });

    thread::park();

    // for midichan in conn_vec.drain(..) {
    //     midichan.connection.close();
    // }
}

struct MidiChannel {
    connection: MidiInputConnection<String>,
    portname: String,
}

fn refresh_connections(input_map: &mut HashMap<String, MidiChannel>) {
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

        let conn = local_midi_in.connect(in_port, "midi-port", |stamp, message, closure_port_name| {
            midi_listener(stamp, &message, &closure_port_name);
        }, port_name.clone());

        if conn.is_ok() {
            input_map.insert(port_name.clone(), MidiChannel { connection: conn.unwrap(), portname: port_name.clone() });
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
fn midi_listener(stamp: u64, message: &[u8], port_name: &String) {
    // i want the current buffer (a, c, d, g, 3) & buffer begin time.
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // println!("{}: {:?} (len = {})", stamp, message, message.len());
    if message.len() >= 2 {
        let note = num_to_note(message[1]);
        if message[2] != 0 {
            if BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().len() == 0 {
                BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().insert(note.clone());

                let new_port_name = port_name.clone();
                let local_thread = thread::spawn(move || {
                    let wait_time = Duration::from_millis(AFTER_PRESS_WAIT);
                    thread::sleep(wait_time);

                    let snapshot_of_charset = BUFFER_MAP.lock().unwrap().get(&new_port_name.clone()).unwrap().clone();
                    BUFFER_MAP.lock().unwrap().get_mut(&new_port_name.clone()).unwrap().clear();
                    let layout_str = chars_to_layout_str(&snapshot_of_charset);
                    let modifiers = get_modifiers(&snapshot_of_charset);

                    if *ENABLED_MAP.lock().unwrap().get(&new_port_name.clone()).unwrap() {
                        if layout_str.len() >= 5 {
                            ENABLED_MAP.lock().unwrap().insert(new_port_name.clone(), false);
                        } else {
                            let key = LAYOUT.get(layout_str.as_str()).unwrap_or(&"invalid");
                            if *key != "invalid" {
                                Command::new("xdotool")
                                    .arg("key")
                                    .arg(concat([modifiers, vec![String::from(*key)]]).join(&String::from("+")))
                                    .spawn()
                                    .expect("echo failed");
                            }
                        }
                    } else if modifiers.len() == 5 {
                        ENABLED_MAP.lock().unwrap().insert(new_port_name.clone(), true);
                    }
                });
            } else {
                BUFFER_MAP.lock().unwrap().get_mut(&port_name.clone()).unwrap().insert(note.clone());
            }

            // println!("{}: {}, {}, {}", note, LAYOUT.get(&note.to_string().as_str()).unwrap_or(&"invalid"), millis, stamp);
        }
    }
}

fn get_modifiers(chars: &HashSet<char>) -> Vec<String> {
    let mut modifiers = Vec::new();
    let mut sorted_chars: Vec<&char> = chars.into_iter().collect();
    sorted_chars.sort();

    for character in &sorted_chars {
        if **character >= '0' && **character <= '4' {
            modifiers.push(String::from(*LAYOUT.get(character.to_string().as_str()).unwrap_or(&"invalid")));
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
