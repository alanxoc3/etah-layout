extern crate midir;

use std::thread;
use hatel::LAYOUT;
use hatel::NUM_TO_NOTE;

use midir::{MidiInput, Ignore, MidiInputConnection};

fn num_to_note(note_num: u8) -> &'static str {
    NUM_TO_NOTE.get(&(note_num % 12)).unwrap_or(&"invalid")
}

fn main() {
    let mut conn_vec = refresh_connections();

    println!("Listening on all midi in connections:");
    for i in 0..conn_vec.len() {
        println!("{}: {}", &conn_vec[i].portnum, &conn_vec[i].portname);
    }

    thread::park();

    for midichan in conn_vec.drain(..) {
        midichan.connection.close();
    }

    println!("Closing connection");
}

struct MidiChannel {
    connection: MidiInputConnection<()>,
    portname: String,
    portnum: usize,
}

fn refresh_connections() -> Vec<MidiChannel> {
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);

    // I can use "port" or "get_name" to check the current state. There is also a close method for MidiInputConnection. See:
    // https://docs.rs/midir/0.7.0/midir/struct.MidiInput.html
    let in_ports = midi_in.ports();
    let mut conn_vec = Vec::new();

    for i in 0..in_ports.len() {
        let mut local_midi_in = MidiInput::new("midir reading input").unwrap();
        local_midi_in.ignore(Ignore::None);

        let in_port = &in_ports[i];
        let in_port_name = local_midi_in.port_name(in_port).unwrap();

        let conn = local_midi_in.connect(in_port, "midi-port", |stamp, message, _| {
            midi_listener(stamp, &message);
        }, ());

        if conn.is_ok() {
            conn_vec.push(MidiChannel { connection: conn.unwrap(), portname: in_port_name, portnum: i });
        }
    }

    return conn_vec
}

fn midi_listener(stamp: u64, message: &[u8]) {
    println!("{}: {:?} (len = {})", stamp, message, message.len());
    if message.len() >= 2 {
        let note = num_to_note(message[1]);
        println!("{}: {}", note, LAYOUT.get(&note).unwrap_or(&"invalid"));
    }
}
