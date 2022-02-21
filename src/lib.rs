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

            ("0"      , ["alt"          , "alt"          , "alt"          ]), // black keys
            ("1"      , [""             , ""             , ""             ]), // fn
            ("2"      , ["super"        , "super"        , "super"        ]),
            ("3"      , ["shift"        , "shift"        , "shift"        ]),
            ("4"      , ["ctrl"         , "ctrl"         , "ctrl"         ]),
            ("01234"  , ["enable"       , "enable"       , "enable"       ]),
        ])
    };
}
