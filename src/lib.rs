extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;
use clap::{Parser, ArgEnum};

#[derive(Parser, ArgEnum, Debug, Clone, Copy)]
pub enum KeyEmulationType {
    Xdotool = 0,
    Other = 1,
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

    pub static ref LAYOUT: HashMap<&'static str, [&'static str; 2]> = {
        HashMap::from([
			// midi      xdotool         ????
            ("a"      , ["k"           , "k"           ]),
            ("b"      , ["l"           , "l"           ]),
            ("c"      , ["h"           , "h"           ]),
            ("d"      , ["j"           , "j"           ]),
            ("e"      , ["a"           , "a"           ]),
            ("f"      , ["e"           , "e"           ]),
            ("g"      , ["t"           , "t"           ]),

            ("ab"     , ["z"           , "z"           ]),
            ("ac"     , ["u"           , "u"           ]),
            ("ad"     , ["space"       , "space"       ]),
            ("ae"     , ["r"           , "r"           ]),
            ("af"     , ["m"           , "m"           ]),
            ("ag"     , ["f"           , "f"           ]),
            ("bc"     , ["BackSpace"   , "BackSpace"   ]),
            ("bd"     , ["n"           , "n"           ]),
            ("be"     , ["d"           , "d"           ]),
            ("bf"     , ["p"           , "p"           ]),
            ("bg"     , ["g"           , "g"           ]),
            ("cd"     , ["v"           , "v"           ]),
            ("ce"     , ["c"           , "c"           ]),
            ("cf"     , ["w"           , "w"           ]),
            ("cg"     , ["y"           , "y"           ]),
            ("de"     , ["x"           , "x"           ]),
            ("df"     , ["o"           , "o"           ]),
            ("dg"     , ["i"           , "i"           ]),
            ("ef"     , ["q"           , "q"           ]),
            ("eg"     , ["s"           , "s"           ]),
            ("fg"     , ["b"           , "b"           ]),

            // Some things to assign...
            // - = Key.insert Key.delete
            ("abc"    , ["Delete"      , "Delete"      ]),
            ("abd"    , [""            , ""            ]), // unassigned
            ("abe"    , [""            , ""            ]), // unassigned
            ("abf"    , [""            , ""            ]), // unassigned
            ("abg"    , ["Escape"      , "Escape"      ]),
            ("acd"    , ["4"           , "4"           ]),
            ("ace"    , ["7"           , "7"           ]),
            ("acf"    , ["9"           , "9"           ]),
            ("acg"    , ["0"           , "0"           ]),
            ("ade"    , ["comma"       , "comma"       ]),
            ("adf"    , ["enter"       , "enter"       ]),
            ("adg"    , ["period"      , "period"      ]),
            ("aef"    , ["grave"       , "grave"       ]),
            ("aeg"    , ["backslash"   , "backslash"   ]),
            ("afg"    , ["equal"       , "equal"       ]),
            ("bcd"    , ["Insert"      , "Insert"      ]),
            ("bce"    , ["bracketleft" , "bracketleft" ]),
            ("bcf"    , [""            , ""            ]), // unassigned
            ("bcg"    , ["bracketright", "bracketright"]),
            ("bde"    , [""            , ""            ]), // unassigned
            ("bdf"    , ["Tab"         , "Tab"         ]),
            ("bdg"    , [""            , ""            ]), // unassigned
            ("bef"    , [""            , ""            ]), // unassigned
            ("beg"    , [""            , ""            ]), // unassigned
            ("bfg"    , [""            , ""            ]), // unassigned
            ("cde"    , ["1"           , "1"           ]),
            ("cdf"    , ["2"           , "2"           ]),
            ("cdg"    , ["3"           , "3"           ]),
            ("cef"    , ["5"           , "5"           ]),
            ("ceg"    , ["6"           , "6"           ]),
            ("cfg"    , ["8"           , "8"           ]),
            ("def"    , ["minus"       , "minus"       ]),
            ("deg"    , ["slash"       , "slash"       ]),
            ("dfg"    , ["apostrophe"  , "apostrophe"  ]),
            ("efg"    , ["semicolon"   , "semicolon"   ]),

            // 5 black keys
            ("0"      , ["alt"         , "alt"         ]),
            ("1"      , [""            , ""            ]), // fn
            ("2"      , ["super"       , "super"       ]),
            ("3"      , ["shift"       , "shift"       ]),
            ("4"      , ["ctrl"        , "ctrl"        ]),
            ("01234"  , ["enable"      , "enable"      ]),
        ])
    };
}
