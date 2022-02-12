extern crate midir;

use std::collections::HashSet;
use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use hatel::LAYOUT;
use hatel::NUM_TO_NOTE;
use std::process::Command;

use midir::{MidiInput, MidiInputConnection};

fn num_to_note(note_num: u8) -> &'static str {
    NUM_TO_NOTE.get(&(note_num % 12)).unwrap_or(&"invalid")
}

fn main() {
    let mut input_map = HashMap::from([]);

    let scheduler = thread::spawn(move || {
        loop {
            let wait_time = Duration::from_millis(1000);
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
    connection: MidiInputConnection<()>,
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

        let local_midi_in = MidiInput::new("midir reading input").unwrap();
        let in_port = touched_ports_map.get(port_name).unwrap();

        let conn = local_midi_in.connect(in_port, "midi-port", |stamp, message, _| {
            midi_listener(stamp, &message);
        }, ());

        if conn.is_ok() {
            input_map.insert(port_name.clone(), MidiChannel { connection: conn.unwrap(), portname: port_name.clone() });
        } else {
            println!("Lost Connection: {}", port_name);
            input_map.remove(port_name).unwrap().connection.close();
        }
    }
}

fn midi_listener(_stamp: u64, message: &[u8]) {
    // println!("{}: {:?} (len = {})", stamp, message, message.len());
    if message.len() >= 2 {
        let note = num_to_note(message[1]);
        if message[2] != 0 {
            println!("{}: {}", note, LAYOUT.get(&note).unwrap_or(&"invalid"));
        }
    }

    // Command::new("echo")
    //     .arg("hello")
    //     .spawn()
    //     .expect("echo failed");
}
