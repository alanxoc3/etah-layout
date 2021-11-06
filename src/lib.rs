extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LAYOUT: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("a"      , "k"           ),
            ("b"      , "l"           ),
            ("c"      , "e"           ),
            ("d"      , "t"           ),
            ("e"      , "a"           ),
            ("f"      , "h"           ),
            ("g"      , "j"           ),

            ("ab"     , "BackSpace"   ),
            ("ac"     , "n"           ),
            ("ad"     , "d"           ),
            ("ae"     , "p"           ),
            ("af"     , "g"           ),
            ("ag"     , "z"           ),
            ("bc"     , "v"           ),
            ("bd"     , "c"           ),
            ("be"     , "w"           ),
            ("bf"     , "y"           ),
            ("bg"     , "u"           ),
            ("cd"     , "x"           ),
            ("ce"     , "o"           ),
            ("cf"     , "i"           ),
            ("cg"     , "space"       ),
            ("de"     , "q"           ),
            ("df"     , "s"           ),
            ("dg"     , "r"           ),
            ("ef"     , "b"           ),
            ("eg"     , "m"           ),
            ("fg"     , "f"           ),

            // Some things to assign...
            // - = Key.insert Key.delete
            ("abc"    , "Insert"      ),
            ("abd"    , "bracketleft" ),
            ("abe"    , ""            ), // unassigned
            ("abf"    , "bracketright"),
            ("abg"    , "Delete"      ),
            ("acd"    , ""            ), // unassigned
            ("ace"    , "Tab"         ),
            ("acf"    , ""            ), // unassigned
            ("acg"    , ""            ), // unassigned
            ("ade"    , ""            ), // unassigned
            ("adf"    , ""            ), // unassigned
            ("adg"    , ""            ), // unassigned
            ("aef"    , ""            ), // unassigned
            ("aeg"    , ""            ), // unassigned
            ("afg"    , "Escape"      ),
            ("bcd"    , "1"           ),
            ("bce"    , "2"           ),
            ("bcf"    , "3"           ),
            ("bcg"    , "4"           ),
            ("bde"    , "5"           ),
            ("bdf"    , "6"           ),
            ("bdg"    , "7"           ),
            ("bef"    , "8"           ),
            ("beg"    , "9"           ),
            ("bfg"    , "0"           ),
            ("cde"    , "minus"       ),
            ("cdf"    , "slash"       ),
            ("cdg"    , "comma"       ),
            ("cef"    , "apostrophe"  ),
            ("ceg"    , "enter"       ),
            ("cfg"    , "period"      ),
            ("def"    , "semicolon"   ),
            ("deg"    , "grave"       ),
            ("dfg"    , "backslash"   ),
            ("efg"    , "equal"       ),

            // 5 black keys
            ("0"      , "super"       ),
            ("1"      , ""            ), // fn
            ("2"      , "shift"       ),
            ("3"      , "ctrl"        ),
            ("4"      , "alt"         ),
            ("01234"  , "enable"      ),
        ])
    };
}
