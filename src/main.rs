extern crate midir;

use std::error::Error;
use std::thread;
use hatel::LAYOUT;
use hatel::NUM_TO_NOTE;

use midir::{MidiInput, Ignore};

fn num_to_note(note_num: u8) -> &'static str {
    NUM_TO_NOTE.get(&(note_num % 12)).unwrap_or(&"invalid")
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // I can use "port" or "get_name" to check the current state. There is also a close method for MidiInputConnection. See:
    // https://docs.rs/midir/0.7.0/midir/struct.MidiInput.html
    let in_ports = midi_in.ports();
    let mut conn_vec = Vec::new();

    if in_ports.len() == 0 {
        return Err("no input port found".into())
    }

    println!("Listening on all midi in connections:");
    for i in 0..in_ports.len() {
        let mut local_midi_in = MidiInput::new("midir reading input")?;
        local_midi_in.ignore(Ignore::None);

        let in_port = &in_ports[i];
        let in_port_name = local_midi_in.port_name(in_port)?;

        let conn = local_midi_in.connect(in_port, "midi-port", |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            if message.len() >= 2 {
                let note = num_to_note(message[1]);
                println!("{}: {}", note, LAYOUT.get(&note).unwrap_or(&"invalid"));
            }
        }, ());


        if conn.is_ok() {
            println!("{}: {}", i, in_port_name);
            conn_vec.push(conn);
        }
    }

    thread::park();

    println!("Closing connection");
    Ok(())
}
