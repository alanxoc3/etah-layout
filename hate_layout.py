from pynput.keyboard import Key

hate_layout = {
        "b"      : "l",
        "c"      : "e",
        "d"      : "t",
        "e"      : "a",
        "f"      : "h",
        "g"      : "j",
        "a"      : "k",

        "bc"     : "v",
        "bd"     : "c",
        "be"     : "w",
        "bf"     : "y",
        "bg"     : "u",
        "ab"     : Key.backspace,
        "cd"     : "x",
        "ce"     : "o",
        "cf"     : "i",
        "cg"     : Key.space,
        "ac"     : "n",
        "de"     : "q",
        "df"     : "s",
        "dg"     : "r",
        "ad"     : "d",
        "ef"     : "b",
        "eg"     : "m",
        "ae"     : "p",
        "fg"     : "f",
        "af"     : "g",
        "ag"     : "z",

        "bcd"    : "1",
        "bce"    : "2",
        "bcf"    : "3",
        "bcg"    : "4",
        "abc"    : "", # unassigned
        "bde"    : "5",
        "bdf"    : "6",
        "bdg"    : "7",
        "abd"    : "", # unassigned
        "bef"    : "8",
        "beg"    : "9",
        "abe"    : "", # unassigned
        "bfg"    : "0",
        "abf"    : "-",
        "abg"    : "=",
        "cde"    : "`",
        "cdf"    : "[",
        "cdg"    : "]",
        "acd"    : "/",
        "cef"    : ";",
        "ace"    : "'",
        "cfg"    : "\\",
        "acf"    : ",",
        "cge"    : ".",
        "acg"    : "", # Key.esc
        "def"    : "", # Key.tab
        "deg"    : "", # Key.enter
        "ade"    : "", # Key.insert
        "dfg"    : "", # Key.delete
        "adf"    : "", # unassigned
        "adg"    : "", # unassigned
        "efg"    : "", # unassigned
        "aef"    : "", # unassigned
        "aeg"    : "", # unassigned
        "afg"    : "", # unassigned

        "bcde"   : "", # unassigned
        "bcdf"   : "", # unassigned
        "bcdg"   : "", # unassigned
        "abcd"   : "", # unassigned
        "bcef"   : "", # unassigned
        "bceg"   : "", # unassigned
        "abce"   : "", # unassigned
        "bcfg"   : "", # unassigned
        "abcf"   : "", # unassigned
        "abcg"   : "", # unassigned
        "bdef"   : "", # unassigned
        "bdeg"   : "", # unassigned
        "abde"   : "", # unassigned
        "bdfg"   : "", # unassigned
        "abdf"   : "", # unassigned
        "abdg"   : "", # unassigned
        "befg"   : "", # unassigned
        "abef"   : "", # unassigned
        "abeg"   : "", # unassigned
        "abfg"   : "", # unassigned
        "cdef"   : "", # unassigned
        "cdeg"   : "", # unassigned
        "acde"   : "", # unassigned
        "cdfg"   : "", # unassigned
        "acdf"   : "", # unassigned
        "acdg"   : "", # unassigned
        "cefg"   : "", # unassigned
        "acef"   : "", # unassigned
        "aceg"   : "", # unassigned
        "acfg"   : "", # unassigned
        "defg"   : "", # unassigned
        "adef"   : "", # unassigned
        "adeg"   : "", # unassigned
        "adfg"   : "", # unassigned
        "aefg"   : "", # unassigned

        # 5 black keys
        "0"      : Key.alt,
        "1"      : Key.ctrl,
        "2"      : Key.shift,
        "3"      : Key.cmd,
        "4"      : "", # Key.fn
}
