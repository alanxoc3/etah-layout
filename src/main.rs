extern crate midir;

use std::collections::HashSet;
use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

use midir::{MidiInput, MidiInputConnection};

fn main() {
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let mut input_map = HashMap::from([]);
    let wait_time = Duration::from_millis(1000);

    let _mode_thread = thread::spawn(move || { chord_thread(cmd_rx) });

    let _scheduler = thread::spawn(move || {
        loop {
            refresh_connections(&mut input_map, cmd_tx.clone());
            thread::sleep(wait_time);
        }
    });

    thread::park();

    // for midichan in conn_vec.drain(..) {
    //     midichan.connection.close();
    // }
}

struct MidiChannel {
    midi_thread: thread::JoinHandle<()>,
    midi_connection: MidiInputConnection<(mpsc::Sender<MidiThreadMessage>,)>,
    midi_thread_tx: mpsc::Sender<MidiThreadMessage>
}

fn close_connection(port_name: &String, input_map: &mut HashMap<String, MidiChannel>) {
    let chan = input_map.remove(port_name).unwrap();
    chan.midi_connection.close();
    chan.midi_thread_tx.send(MidiThreadMessage { variant: MidiThreadMessageVariant::Terminate, note: 0 }).unwrap();
    chan.midi_thread.join().expect("could not wait for midi thread to close");
    println!("Lost Connection: {}", &port_name);
}

fn refresh_connections(input_map: &mut HashMap<String, MidiChannel>, mode_tx: mpsc::Sender<String>) {
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
        close_connection(x, input_map);
    }

    for port_name in touched_ports.difference(&prev_ports) {
        let local_midi_in = MidiInput::new("midir reading input").unwrap();
        let in_port = touched_ports_map.get(port_name).unwrap();

        let (midi_tx, midi_rx) = mpsc::channel();
        let conn = local_midi_in.connect(in_port, "midi-port", |_, message, pair| {
            if message.len() >= 3 {
                pair.0.send(MidiThreadMessage { variant: if message[2] != 0 { MidiThreadMessageVariant::Press } else { MidiThreadMessageVariant::Release }, note: message[1] }).unwrap();
            }
        }, (midi_tx.clone(),));

        if conn.is_ok() {
            println!("New Connection: {}", port_name);
            let local_mode_tx = mode_tx.clone();
            let local_midi_thread = thread::spawn(move || { midi_thread(midi_rx, local_mode_tx); });
            input_map.insert(port_name.clone(), MidiChannel { midi_thread: local_midi_thread, midi_connection: conn.unwrap(), midi_thread_tx: midi_tx });
        }
    }
}

enum MidiThreadMessageVariant { Terminate, Timeout, Press, Release }
struct MidiThreadMessage { variant: MidiThreadMessageVariant, note: u8 }
fn midi_thread(midi_rx: mpsc::Receiver<MidiThreadMessage>, mode_tx: mpsc::Sender<String>) {
    for received in midi_rx {
        let variant_str = match received.variant {
            MidiThreadMessageVariant::Terminate => "Terminate",
            MidiThreadMessageVariant::Timeout => "Timeout",
            MidiThreadMessageVariant::Press=> "Press",
            MidiThreadMessageVariant::Release => "Release",
        };

        println!("send: {} {}", variant_str, received.note);
        mode_tx.send(format!("{}: {}", received.note, variant_str)).unwrap();
    }
}

fn chord_thread(rx: mpsc::Receiver<String>) {
    for received in rx {
        println!("receive: {}", received);
    }
}
