extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;
use clap::{Parser, ArgEnum};

#[derive(Parser, ArgEnum, Debug, Clone, Copy)]
pub enum KeyEmulationType {
    // prints printable characters and shows the name of non-printable characters.
    Echo = 0,

    // https://gitlab.com/cunidev/gestures/-/wikis/xdotool-list-of-key-codes
    Xdotool = 1,

    // https://eastmanreference.com/complete-list-of-applescript-key-codes
    Osascript = 2,

    // https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
    Pi = 3,
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

    pub static ref LAYOUT: HashMap<&'static str, [&'static str; 4]> = {
        HashMap::from([
            // midi      echo             xdotool          osascript         pi
            ("a"      , ["k"            , "k"            , "40"            , "0e"            ]),
            ("b"      , ["l"            , "l"            , "37"            , "0f"            ]),
            ("c"      , ["h"            , "h"            , "4"             , "0b"            ]),
            ("d"      , ["j"            , "j"            , "38"            , "0d"            ]),
            ("e"      , ["a"            , "a"            , "0"             , "04"            ]),
            ("f"      , ["e"            , "e"            , "14"            , "08"            ]),
            ("g"      , ["t"            , "t"            , "17"            , "17"            ]),

            ("ab"     , ["z"            , "z"            , "6"             , "1d"            ]),
            ("ac"     , ["u"            , "u"            , "32"            , "18"            ]),
            ("ad"     , ["space"        , "space"        , "49"            , "2c"            ]),
            ("ae"     , ["r"            , "r"            , "15"            , "15"            ]),
            ("af"     , ["m"            , "m"            , "46"            , "10"            ]),
            ("ag"     , ["f"            , "f"            , "3"             , "09"            ]),
            ("bc"     , ["backspace"    , "BackSpace"    , "51"            , "2a"            ]),
            ("bd"     , ["n"            , "n"            , "45"            , "11"            ]),
            ("be"     , ["d"            , "d"            , "2"             , "07"            ]),
            ("bf"     , ["p"            , "p"            , "35"            , "13"            ]),
            ("bg"     , ["g"            , "g"            , "5"             , "0a"            ]),
            ("cd"     , ["v"            , "v"            , "9"             , "19"            ]),
            ("ce"     , ["c"            , "c"            , "8"             , "06"            ]),
            ("cf"     , ["w"            , "w"            , "13"            , "1a"            ]),
            ("cg"     , ["y"            , "y"            , "16"            , "1c"            ]),
            ("de"     , ["x"            , "x"            , "7"             , "1b"            ]),
            ("df"     , ["o"            , "o"            , "31"            , "12"            ]),
            ("dg"     , ["i"            , "i"            , "34"            , "0c"            ]),
            ("ef"     , ["q"            , "q"            , "12"            , "14"            ]),
            ("eg"     , ["s"            , "s"            , "1"             , "16"            ]),
            ("fg"     , ["b"            , "b"            , "11"            , "05"            ]),

            ("abc"    , ["delete"       , "Delete"       , ""              , "4c"            ]),
            ("abd"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("abe"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("abf"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("abg"    , ["escape"       , "Escape"       , "53"            , "29"            ]),
            ("acd"    , ["4"            , "4"            , "21"            , "21"            ]),
            ("ace"    , ["7"            , "7"            , "26"            , "24"            ]),
            ("acf"    , ["9"            , "9"            , "25"            , "26"            ]),
            ("acg"    , ["0"            , "0"            , "29"            , "27"            ]),
            ("ade"    , [","            , "comma"        , "43"            , "36"            ]),
            ("adf"    , ["enter"        , "enter"        , "36"            , "28"            ]),
            ("adg"    , ["."            , "period"       , "47"            , "37"            ]),
            ("aef"    , ["`"            , "grave"        , "50"            , "35"            ]),
            ("aeg"    , ["\\"           , "backslash"    , "42"            , "31"            ]),
            ("afg"    , ["="            , "equal"        , "24"            , "2e"            ]),
            ("bcd"    , ["insert"       , "Insert"       , ""              , "49"            ]),
            ("bce"    , ["["            , "bracketleft"  , "33"            , "2f"            ]),
            ("bcf"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("bcg"    , ["]"            , "bracketright" , "30"            , "30"            ]),
            ("bde"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("bdf"    , ["tab"          , "Tab"          , "48"            , "2b"            ]),
            ("bdg"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("bef"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("beg"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("bfg"    , [""             , ""             , ""              , ""              ]), // unassigned
            ("cde"    , ["1"            , "1"            , "18"            , "1e"            ]),
            ("cdf"    , ["2"            , "2"            , "19"            , "1f"            ]),
            ("cdg"    , ["3"            , "3"            , "20"            , "20"            ]),
            ("cef"    , ["5"            , "5"            , "23"            , "22"            ]),
            ("ceg"    , ["6"            , "6"            , "22"            , "23"            ]),
            ("cfg"    , ["8"            , "8"            , "28"            , "25"            ]),
            ("def"    , ["-"            , "minus"        , "27"            , "2d"            ]),
            ("deg"    , ["/"            , "slash"        , "44"            , "38"            ]),
            ("dfg"    , ["'"            , "apostrophe"   , "39"            , "34"            ]),
            ("efg"    , [";"            , "semicolon"    , "41"            , "33"            ]),

            ("0"      , ["alt"          , "alt"          , "option"        , "04"            ]), // black keys
            ("1"      , [""             , ""             , ""              , ""              ]), // fn
            ("2"      , ["super"        , "super"        , "command"       , "08"            ]),
            ("3"      , ["shift"        , "shift"        , "shift"         , "02"            ]),
            ("4"      , ["ctrl"         , "ctrl"         , "control"       , "01"            ]),
            ("01234"  , ["enable"       , "enable"       , "enable"        , "enable"        ]),
        ])
    };
}
