extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;
use clap::{Parser, ArgEnum};

#[derive(Parser, ArgEnum, Debug, Clone, Copy)]
pub enum KeyEmulationType {
    // prints printable characters and shows the name of non-printable characters.
    Echo = 0,

    // prints the raw midi numbers in an array form.
    Midi = 1,

    // https://gitlab.com/cunidev/gestures/-/wikis/xdotool-list-of-key-codes
    Xdotool = 2,

    // https://eastmanreference.com/complete-list-of-applescript-key-codes
    Osascript = 3,
}

lazy_static! {
    pub static ref NUM_TO_NOTE: HashMap<u8, char> = {
        HashMap::from([
            (9 , 'a'),
            (11, 'b'),
            (0 , 'c'),
            (2 , 'd'),
            (4 , 'e'),
            (5 , 'f'),
            (7 , 'g'),
            (10, '0'),
            (1 , '1'),
            (3 , '2'),
            (6 , '3'),
            (8 , '4'),
        ])
    };

    pub static ref LAYOUT: HashMap<&'static str, [&'static str; 3]> = {
        HashMap::from([
            // midi      echo             xdotool          osascript
            ("a"      , ["k"            , "k"            , "40"            ]),
            ("b"      , ["l"            , "l"            , "37"            ]),
            ("c"      , ["h"            , "h"            , "4"             ]),
            ("d"      , ["j"            , "j"            , "38"            ]),
            ("e"      , ["a"            , "a"            , "0"             ]),
            ("f"      , ["e"            , "e"            , "14"            ]),
            ("g"      , ["t"            , "t"            , "17"            ]),

            ("ab"     , ["z"            , "z"            , "6"             ]),
            ("ac"     , ["u"            , "u"            , "32"            ]),
            ("ad"     , ["space"        , "space"        , "49"            ]),
            ("ae"     , ["r"            , "r"            , "15"            ]),
            ("af"     , ["m"            , "m"            , "46"            ]),
            ("ag"     , ["f"            , "f"            , "3"             ]),
            ("bc"     , ["backspace"    , "BackSpace"    , "51"            ]),
            ("bd"     , ["n"            , "n"            , "45"            ]),
            ("be"     , ["d"            , "d"            , "2"             ]),
            ("bf"     , ["p"            , "p"            , "35"            ]),
            ("bg"     , ["g"            , "g"            , "5"             ]),
            ("cd"     , ["v"            , "v"            , "9"             ]),
            ("ce"     , ["c"            , "c"            , "8"             ]),
            ("cf"     , ["w"            , "w"            , "13"            ]),
            ("cg"     , ["y"            , "y"            , "16"            ]),
            ("de"     , ["x"            , "x"            , "7"             ]),
            ("df"     , ["o"            , "o"            , "31"            ]),
            ("dg"     , ["i"            , "i"            , "34"            ]),
            ("ef"     , ["q"            , "q"            , "12"            ]),
            ("eg"     , ["s"            , "s"            , "1"             ]),
            ("fg"     , ["b"            , "b"            , "11"            ]),

            ("abc"    , ["delete"       , "Delete"       , ""              ]), // - = Key.insert Key.delete
            ("abd"    , [""             , ""             , ""              ]), // unassigned
            ("abe"    , [""             , ""             , ""              ]), // unassigned
            ("abf"    , [""             , ""             , ""              ]), // unassigned
            ("abg"    , ["escape"       , "Escape"       , "53"            ]),
            ("acd"    , ["4"            , "4"            , "21"            ]),
            ("ace"    , ["7"            , "7"            , "26"            ]),
            ("acf"    , ["9"            , "9"            , "25"            ]),
            ("acg"    , ["0"            , "0"            , "29"            ]),
            ("ade"    , [","            , "comma"        , "43"            ]),
            ("adf"    , ["enter"        , "enter"        , "36"            ]),
            ("adg"    , ["."            , "period"       , "47"            ]),
            ("aef"    , ["`"            , "grave"        , "50"            ]),
            ("aeg"    , ["\\"           , "backslash"    , "42"            ]),
            ("afg"    , ["="            , "equal"        , "24"            ]),
            ("bcd"    , ["insert"       , "Insert"       , ""              ]),
            ("bce"    , ["["            , "bracketleft"  , "33"            ]),
            ("bcf"    , [""             , ""             , ""              ]), // unassigned
            ("bcg"    , ["]"            , "bracketright" , "30"            ]),
            ("bde"    , [""             , ""             , ""              ]), // unassigned
            ("bdf"    , ["tab"          , "Tab"          , "48"            ]),
            ("bdg"    , [""             , ""             , ""              ]), // unassigned
            ("bef"    , [""             , ""             , ""              ]), // unassigned
            ("beg"    , [""             , ""             , ""              ]), // unassigned
            ("bfg"    , [""             , ""             , ""              ]), // unassigned
            ("cde"    , ["1"            , "1"            , "18"            ]),
            ("cdf"    , ["2"            , "2"            , "19"            ]),
            ("cdg"    , ["3"            , "3"            , "20"            ]),
            ("cef"    , ["5"            , "5"            , "23"            ]),
            ("ceg"    , ["6"            , "6"            , "22"            ]),
            ("cfg"    , ["8"            , "8"            , "28"            ]),
            ("def"    , ["-"            , "minus"        , "27"            ]),
            ("deg"    , ["/"            , "slash"        , "44"            ]),
            ("dfg"    , ["'"            , "apostrophe"   , "39"            ]),
            ("efg"    , [";"            , "semicolon"    , "41"            ]),

            ("0"      , ["alt"          , "alt"          , "option"        ]), // black keys
            ("1"      , [""             , ""             , ""              ]), // fn
            ("2"      , ["super"        , "super"        , "command"       ]),
            ("3"      , ["shift"        , "shift"        , "shift"         ]),
            ("4"      , ["ctrl"         , "ctrl"         , "control"       ]),
            ("01234"  , ["enable"       , "enable"       , "enable"        ]),
        ])
    };
}

// use std::sync::Mutex;
// use lazy_static::lazy_static;
// use hatel::{KeyEmulationType, LAYOUT, NUM_TO_NOTE};
// use std::process::Command;
// use itertools::concat;

// Arguments the program accepts.
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// Method for key input
//     #[clap(arg_enum)]
//     key_emulation: KeyEmulationType,
// }
// fn num_to_note(note_num: u8) -> char {
//     NUM_TO_NOTE.get(&(note_num % 12)).unwrap().clone()
// }
// 
// const AFTER_PRESS_WAIT: u64 = 100;

// lazy_static! {
//     // BUFFER_MAP is a map from input (eg piano1 or piano2) to a list of midi signals (one midi signal is represented by Vec<u8>).
//     static ref BUFFER_MAP: Mutex<HashMap<String, Vec<Vec<u8>> >> = Mutex::new(HashMap::new());
//     static ref ENABLED_MAP: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
// }
// 
// // static map maybe?
//
// fn get_modifiers(key_emulation: &KeyEmulationType, chars: &HashSet<char>) -> Vec<String> {
//     let mut modifiers = Vec::new();
//     let mut sorted_chars: Vec<&char> = chars.into_iter().collect();
//     sorted_chars.sort();
// 
//     for character in &sorted_chars {
//         if **character == '0' || (**character >= '2' && **character <= '4') {
//             match LAYOUT.get(character.to_string().as_str()) {
//                 Some(key) => {
//                     let val = String::from(key[*key_emulation as usize]);
//                     if val.len() != 0 {
//                         modifiers.push(val);
//                     }
//                 },
//                 None => { }
//             }
//         }
//     }
//     return modifiers;
// }
// 
// fn chars_to_layout_str(chars: &HashSet<char>) -> String {
//     let mut layout_chars = Vec::new();
// 
//     for character in chars {
//         if *character >= 'a' && *character <= 'g' {
//             layout_chars.push(character.clone());
//         }
//     }
// 
//     layout_chars.sort();
// 
//     return layout_chars.into_iter().collect();
// }
// 
// fn simulate_keyboard_press(key_emulation: &KeyEmulationType, _function_pressed: bool, modifiers: Vec<String>, layout_str: String, message: Vec<u8>) {
//     match LAYOUT.get(layout_str.as_str()) {
//         Some(key) => {
//             match *key_emulation {
//                 KeyEmulationType::Echo => {
//                     let mut _cmd = Command::new("echo");
//                     _cmd.arg("key: ").arg(key[*key_emulation as usize]);
// 
//                     if modifiers.len() > 0 {
//                         _cmd.arg(format!("-- ({})", modifiers.join(", ")));
//                     }
// 
//                     _cmd.spawn().expect("echo failed");
//                 },
//                 KeyEmulationType::Midi => {
//                     let mut _cmd = Command::new("echo");
// 
//                     for part in message {
//                         _cmd.arg(part.to_string());
//                     }
// 
//                     _cmd.spawn().expect("echo failed");
//                 },
//                 KeyEmulationType::Xdotool => {
//                     Command::new("xdotool")
//                         .arg("key")
//                         .arg(concat([modifiers, vec![String::from(key[*key_emulation as usize])]]).join(&String::from("+")))
//                         .spawn()
//                         .expect("xdotool failed");
//                 },
//                 KeyEmulationType::Osascript => {
//                     let mut _cmd = Command::new("osascript");
//                     _cmd.arg("-e");
// 
//                     let argstr = format!("tell application \"System Events\" to key code {}", key[*key_emulation as usize]);
// 
//                     let argstr = if modifiers.len() == 1 {
//                         format!("{} using {} down", argstr, modifiers.join(""))
//                     } else if modifiers.len() > 1 {
//                         format!("{} using {{{} down}}", argstr, modifiers.join(" down, "))
//                     } else {
//                         argstr
//                     };
// 
//                     _cmd.arg(argstr).spawn().expect("osascript failed");
//                 },
//             }
//         },
//         None      => {},
//     }
// }
// 
// // Temporary until we have hooks
// fn call_ttrack(group: &'static str, duration: &'static str) {
//     Command::new("ttrack")
//         .arg("rec")
//         .arg(group)
//         .arg(duration)
//         .spawn()
//         .expect("ttrack failed");
// }
