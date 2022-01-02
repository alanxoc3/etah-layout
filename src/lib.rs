extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref NUM_TO_NOTE: HashMap<u8, &'static str> = {
        HashMap::from([
            (9 , "a"),
            (11, "b"),
            (0 , "c"),
            (2 , "d"),
            (4 , "e"),
            (5 , "f"),
            (7 , "g"),
            (10, "0"),
            (1 , "1"),
            (3 , "2"),
            (6 , "3"),
            (8 , "4"),
        ])
    };

    pub static ref LAYOUT: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("a"      , "k"           ),
            ("b"      , "l"           ),
            ("c"      , "h"           ),
            ("d"      , "j"           ),
            ("e"      , "a"           ),
            ("f"      , "e"           ),
            ("g"      , "t"           ),

            ("ab"     , "z"           ),
            ("ac"     , "u"           ),
            ("ad"     , "space"       ),
            ("ae"     , "r"           ),
            ("af"     , "m"           ),
            ("ag"     , "f"           ),
            ("bc"     , "BackSpace"   ),
            ("bd"     , "n"           ),
            ("be"     , "d"           ),
            ("bf"     , "p"           ),
            ("bg"     , "g"           ),
            ("cd"     , "v"           ),
            ("ce"     , "c"           ),
            ("cf"     , "w"           ),
            ("cg"     , "y"           ),
            ("de"     , "x"           ),
            ("df"     , "o"           ),
            ("dg"     , "i"           ),
            ("ef"     , "q"           ),
            ("eg"     , "s"           ),
            ("fg"     , "b"           ),

            // Some things to assign...
            // - = Key.insert Key.delete
            ("abc"    , "Delete"      ),
            ("abd"    , ""            ), // unassigned
            ("abe"    , ""            ), // unassigned
            ("abf"    , ""            ), // unassigned
            ("abg"    , "Escape"      ),
            ("acd"    , "4"           ),
            ("ace"    , "7"           ),
            ("acf"    , "9"           ),
            ("acg"    , "0"           ),
            ("ade"    , "comma"       ),
            ("adf"    , "enter"       ),
            ("adg"    , "period"      ),
            ("aef"    , "grave"       ),
            ("aeg"    , "backslash"   ),
            ("afg"    , "equal"       ),
            ("bcd"    , "Insert"      ),
            ("bce"    , "bracketleft" ),
            ("bcf"    , ""            ), // unassigned
            ("bcg"    , "bracketright"),
            ("bde"    , ""            ), // unassigned
            ("bdf"    , "Tab"         ),
            ("bdg"    , ""            ), // unassigned
            ("bef"    , ""            ), // unassigned
            ("beg"    , ""            ), // unassigned
            ("bfg"    , ""            ), // unassigned
            ("cde"    , "1"           ),
            ("cdf"    , "2"           ),
            ("cdg"    , "3"           ),
            ("cef"    , "5"           ),
            ("ceg"    , "6"           ),
            ("cfg"    , "8"           ),
            ("def"    , "minus"       ),
            ("deg"    , "slash"       ),
            ("dfg"    , "apostrophe"  ),
            ("efg"    , "semicolon"   ),

            // 5 black keys
            ("0"      , "alt"         ),
            ("1"      , ""            ), // fn
            ("2"      , "super"       ),
            ("3"      , "shift"       ),
            ("4"      , "ctrl"        ),
            ("01234"  , "enable"      ),
        ])
    };
}
