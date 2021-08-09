from pynput.keyboard import Key

hate_layout = {
        "a"      : "k",
        "b"      : "l",
        "c"      : "e",
        "d"      : "t",
        "e"      : "a",
        "f"      : "h",
        "g"      : "j",

        "ab"     : Key.backspace,
        "ac"     : "n",
        "ad"     : "d",
        "ae"     : "p",
        "af"     : "g",
        "ag"     : "z",
        "bc"     : "v",
        "bd"     : "c",
        "be"     : "w",
        "bf"     : "y",
        "bg"     : "u",
        "cd"     : "x",
        "ce"     : "o",
        "cf"     : "i",
        "cg"     : Key.space,
        "de"     : "q",
        "df"     : "s",
        "dg"     : "r",
        "ef"     : "b",
        "eg"     : "m",
        "fg"     : "f",

        # Some things to assign...
        # - = Key.insert Key.delete
        "abc"    : Key.insert,
        "abd"    : "-",
        "abe"    : "", # unassigned
        "abf"    : "=",
        "abg"    : Key.delete,
        "acd"    : "", # unassigned
        "ace"    : Key.tab,
        "acf"    : "", # unassigned
        "acg"    : "", # unassigned
        "ade"    : "", # unassigned
        "adf"    : "", # unassigned
        "adg"    : "", # unassigned
        "aef"    : "", # unassigned
        "aeg"    : "", # unassigned
        "afg"    : Key.esc,
        "bcd"    : "1",
        "bce"    : "2",
        "bcf"    : "3",
        "bcg"    : "4",
        "bde"    : "5",
        "bdf"    : "6",
        "bdg"    : "7",
        "bef"    : "8",
        "beg"    : "9",
        "bfg"    : "0",
        "cde"    : "[",
        "cdf"    : "/",
        "cdg"    : ",",
        "cef"    : "'",
        "ceg"    : Key.enter,
        "cfg"    : ".",
        "def"    : ";",
        "deg"    : "`",
        "dfg"    : "\\",
        "efg"    : "]",

        # 5 black keys
        "0"      : Key.alt,
        "1"      : Key.ctrl,
        "2"      : Key.shift,
        "3"      : Key.cmd,
        "4"      : "", # Key.fn

        # arrow keys
        # pg up, pg dwn, home, end
        # prnt scrn

        # 5  black keys = midi mode
        # 5+ white keys = type mode
}
