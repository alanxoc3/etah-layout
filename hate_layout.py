from pynput.keyboard import Key

# c#: shift
# d#: func
# f#: ctrl
# g#: super
# a#: meta

hate_layout = {
        "c"      : "e",
        "d"      : "t",
        "e"      : "a",
        "f"      : "h",
        "g"      : "j",
        "a"      : "k",
        "b"      : "l",
        "cd"     : "x",
        "ce"     : "o",
        "cf"     : "i",
        "cg"     : Key.space,
        "ac"     : "n",
        "bc"     : "v",
        "de"     : "s",
        "df"     : "q",
        "dg"     : "r",
        "ad"     : "d",
        "bd"     : "c",
        "ef"     : "u",
        "eg"     : "m",
        "ae"     : "z",
        "be"     : "w",
        "fg"     : "f",
        "af"     : "g",
        "bf"     : "y",
        "ag"     : "p",
        "bg"     : "b",
        "ab"     : Key.backspace,

        # numbers
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
        "abf"    : "-",
        "abg"    : "=",

        # punctuation
        "adf"    : "`",
        "adg"    : "[",
        "acg"    : "]",
        "efg"    : "/",
        "abd"    : ";",
        "aef"    : "'",
        "acf"    : "\\",
        "aeg"    : ",",
        "ceg"    : ".",
        "ceg"    : "/",

        # Other keys
        "afg"    : Key.tab,
        "abfg"   : Key.enter,
        "abe"    : Key.insert,
        "abcf"   : Key.delete, # opposite backspace
        "abcg"   : Key.esc,
}
