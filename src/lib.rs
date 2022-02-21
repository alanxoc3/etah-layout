extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;
use clap::{Parser, ArgEnum};

#[derive(Parser, ArgEnum, Debug, Clone, Copy)]
pub enum KeyEmulationType {
    Echo = 0, Xdotool   = 1, Osascript = 2,
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
			// midi      echo             xdotool         osascript
            ("a"      , ["k"            , "k"            , "k"            ]),
            ("b"      , ["l"            , "l"            , "l"            ]),
            ("c"      , ["h"            , "h"            , "h"            ]),
            ("d"      , ["j"            , "j"            , "j"            ]),
            ("e"      , ["a"            , "a"            , "a"            ]),
            ("f"      , ["e"            , "e"            , "e"            ]),
            ("g"      , ["t"            , "t"            , "t"            ]),

            ("ab"     , ["z"            , "z"            , "z"            ]),
            ("ac"     , ["u"            , "u"            , "u"            ]),
            ("ad"     , ["space"        , "space"        , "space"        ]),
            ("ae"     , ["r"            , "r"            , "r"            ]),
            ("af"     , ["m"            , "m"            , "m"            ]),
            ("ag"     , ["f"            , "f"            , "f"            ]),
            ("bc"     , ["backspace"    , "BackSpace"    , "BackSpace"    ]),
            ("bd"     , ["n"            , "n"            , "n"            ]),
            ("be"     , ["d"            , "d"            , "d"            ]),
            ("bf"     , ["p"            , "p"            , "p"            ]),
            ("bg"     , ["g"            , "g"            , "g"            ]),
            ("cd"     , ["v"            , "v"            , "v"            ]),
            ("ce"     , ["c"            , "c"            , "c"            ]),
            ("cf"     , ["w"            , "w"            , "w"            ]),
            ("cg"     , ["y"            , "y"            , "y"            ]),
            ("de"     , ["x"            , "x"            , "x"            ]),
            ("df"     , ["o"            , "o"            , "o"            ]),
            ("dg"     , ["i"            , "i"            , "i"            ]),
            ("ef"     , ["q"            , "q"            , "q"            ]),
            ("eg"     , ["s"            , "s"            , "s"            ]),
            ("fg"     , ["b"            , "b"            , "b"            ]),

            ("abc"    , ["delete"       , "Delete"       , "Delete"       ]), // - = Key.insert Key.delete
            ("abd"    , [""             , ""             , ""             ]), // unassigned
            ("abe"    , [""             , ""             , ""             ]), // unassigned
            ("abf"    , [""             , ""             , ""             ]), // unassigned
            ("abg"    , ["escape"       , "Escape"       , "Escape"       ]),
            ("acd"    , ["4"            , "4"            , "4"            ]),
            ("ace"    , ["7"            , "7"            , "7"            ]),
            ("acf"    , ["9"            , "9"            , "9"            ]),
            ("acg"    , ["0"            , "0"            , "0"            ]),
            ("ade"    , [","            , "comma"        , "comma"        ]),
            ("adf"    , ["enter"        , "enter"        , "enter"        ]),
            ("adg"    , ["."            , "period"       , "period"       ]),
            ("aef"    , ["`"            , "grave"        , "grave"        ]),
            ("aeg"    , ["\\"           , "backslash"    , "backslash"    ]),
            ("afg"    , ["="            , "equal"        , "equal"        ]),
            ("bcd"    , ["insert"       , "Insert"       , "Insert"       ]),
            ("bce"    , ["["            , "bracketleft"  , "bracketleft"  ]),
            ("bcf"    , [""             , ""             , ""             ]), // unassigned
            ("bcg"    , ["]"            , "bracketright" , "bracketright" ]),
            ("bde"    , [""             , ""             , ""             ]), // unassigned
            ("bdf"    , ["tab"          , "Tab"          , "Tab"          ]),
            ("bdg"    , [""             , ""             , ""             ]), // unassigned
            ("bef"    , [""             , ""             , ""             ]), // unassigned
            ("beg"    , [""             , ""             , ""             ]), // unassigned
            ("bfg"    , [""             , ""             , ""             ]), // unassigned
            ("cde"    , ["1"            , "1"            , "1"            ]),
            ("cdf"    , ["2"            , "2"            , "2"            ]),
            ("cdg"    , ["3"            , "3"            , "3"            ]),
            ("cef"    , ["5"            , "5"            , "5"            ]),
            ("ceg"    , ["6"            , "6"            , "6"            ]),
            ("cfg"    , ["8"            , "8"            , "8"            ]),
            ("def"    , ["-"            , "minus"        , "minus"        ]),
            ("deg"    , ["/"            , "slash"        , "slash"        ]),
            ("dfg"    , ["'"            , "apostrophe"   , "apostrophe"   ]),
            ("efg"    , [";"            , "semicolon"    , "semicolon"    ]),

            ("0"      , ["alt"          , "alt"          , "alt"          ]), // black keys
            ("1"      , [""             , ""             , ""             ]), // fn
            ("2"      , ["super"        , "super"        , "super"        ]),
            ("3"      , ["shift"        , "shift"        , "shift"        ]),
            ("4"      , ["ctrl"         , "ctrl"         , "ctrl"         ]),
            ("01234"  , ["enable"       , "enable"       , "enable"       ]),
        ])
    };
}
