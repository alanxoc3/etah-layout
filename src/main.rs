extern crate midir;

use std::error::Error;
use std::thread;
use hatel::LAYOUT;

use midir::{MidiInput, Ignore};

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

    if in_ports.len() == 0 {
        return Err("no input port found".into())
    }

    println!("Listening on all midi in connections:");
    for i in 0..in_ports.len() {
        let in_port = &in_ports[i];
        // let in_port_name = midi_in.port_name(in_port)?;
        // println!("{}: {}", i, in_port_name);

        let _conn = (&mut midi_in).connect(in_port, "midi-port", |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            println!("{}", LAYOUT.get(&"a").unwrap_or(&"invalid"));
        }, ())?;
    }


    thread::park();

    println!("Closing connection");
    Ok(())
}

/*
use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    assert_eq!(b"Hello world\n", output.stdout.as_slice());
}
*/
