extern crate midir;

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
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
    midi_connection: MidiInputConnection<(mpsc::Sender<MidiThreadMessage>,String)>,
    midi_thread_tx: mpsc::Sender<MidiThreadMessage>
}

fn close_connection(port_name: &String, input_map: &mut HashMap<String, MidiChannel>) {
    let chan = input_map.remove(port_name).unwrap();
    chan.midi_connection.close();
    chan.midi_thread_tx.send(MidiThreadMessage { source: port_name.clone(), start_time: Instant::now(), variant: MidiThreadMessageVariant::Terminate, note: 0 }).unwrap();
    chan.midi_thread.join().expect("could not wait for midi thread to close");
    println!("Lost Connection: {}", &port_name);
}

fn refresh_connections(input_map: &mut HashMap<String, MidiChannel>, mode_tx: mpsc::Sender<ChordThreadMessage>) {
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
            // note on and off messages always have a length of 3
            if message.len() == 3 {
                let variant = match message[0] & 0b01110000 {
                    0b00000000 => MidiThreadMessageVariant::Release,   // note off
                    0b00010000 => MidiThreadMessageVariant::Press,     // note on
                    default    => MidiThreadMessageVariant::Terminate  // ignore, doesn't terminate
                };

                if variant != MidiThreadMessageVariant::Terminate {
                    println!("message ({}): {:?}", message.len(), message);
                    pair.0.send(MidiThreadMessage { source: pair.1.clone(), start_time: Instant::now(), variant: variant, note: message[1] }).unwrap();
                }
            }
        }, (midi_tx.clone(), port_name.clone()));

        if conn.is_ok() {
            println!("New Connection: {}", port_name);
            let local_mode_tx = mode_tx.clone();
            let local_midi_thread = thread::spawn(move || { midi_thread(midi_rx, local_mode_tx); });
            input_map.insert(port_name.clone(), MidiChannel { midi_thread: local_midi_thread, midi_connection: conn.unwrap(), midi_thread_tx: midi_tx });
        }
    }
}

#[derive(PartialEq)] enum MidiStateVariant { Press, Timeout }

type ChordState = Vec<u8>;
struct TimeoutChord { timeoutStart: Instant, notes: Vec<u8>, }
struct MidiState<'a> {
    noteToChord:  HashMap<u8, &'a ChordState>,
    chords: HashSet<ChordState>,
    sourceToTimeoutChord: HashMap<String, TimeoutChord>,
}

impl<'a> MidiState<'a> {
    fn new(source: String, first_note: u8) -> MidiState<'a> {
        MidiState { noteToChord: HashMap::new(), chords: HashSet::new(), sourceToTimeoutChord: HashMap::new() }
    }
}

// Responsible for state or whatever calls chord thread.
#[derive(PartialEq)] enum MidiThreadMessageVariant { Terminate, Timeout, Press, Release }
struct MidiThreadMessage { source: String, start_time: Instant, variant: MidiThreadMessageVariant, note: u8 }
fn midi_thread(midi_rx: mpsc::Receiver<MidiThreadMessage>, mode_tx: mpsc::Sender<ChordThreadMessage>) {
    for m in midi_rx {
        match m.variant {
            MidiThreadMessageVariant::Press     => {

            },
            MidiThreadMessageVariant::Release   => {

            },
            MidiThreadMessageVariant::Terminate => {

            },
            MidiThreadMessageVariant::Timeout   => {
            },
        };

        if m.variant == MidiThreadMessageVariant::Press || m.variant == MidiThreadMessageVariant::Release {
            let variant = match m.variant {
                MidiThreadMessageVariant::Press => ChordThreadMessageVariant::Press,
                MidiThreadMessageVariant::Release => ChordThreadMessageVariant::Release,
                _ => ChordThreadMessageVariant::Press
            };

            mode_tx.send(ChordThreadMessage {variant: variant, modifiers: vec![], notes: vec![]}).unwrap();
        }
    }
}

#[derive(PartialEq)] enum ChordThreadMessageVariant { Press, Release }
#[derive(PartialEq)] enum Note { A, B, C, D, E, F, G, M0, M1, M2, M3, M4 }

impl Note {
    fn from(num: u8) -> Note { match num % 12 {
        0       => Note::C,
        1       => Note::M1,
        2       => Note::D,
        3       => Note::M2,
        4       => Note::E,
        5       => Note::F,
        6       => Note::M3,
        7       => Note::G,
        8       => Note::M4,
        9       => Note::A,
        10      => Note::M0,
        default => Note::B, // default is 11. the compiler doesn't know that all other values are impossible due to the modulus above.
    } }

    fn is_modifier(&self) -> bool { *self == Note::M0 || *self == Note::M1 || *self == Note::M2 || *self == Note::M3 || *self == Note::M4 }
}

enum Modifier { M0, M1, M2, M3, M4 }
struct ChordThreadMessage { variant: ChordThreadMessageVariant, modifiers: Vec<Modifier>, notes: Vec<Note> }

// common for all modes...
// receives "enum(press,release)", list of modifiers, list of notes (chord)
// executes: cmd p:0 d:123 d:af r:abd ...

// Responsible for the actual press logic.
fn chord_thread(rx: mpsc::Receiver<ChordThreadMessage>) {
    for m in rx {
        let variant = match m.variant {
            ChordThreadMessageVariant::Press => "PRESS",
            ChordThreadMessageVariant::Release => "RELEASE"
        };

        // println!("receive: {}", variant);
    }
}
