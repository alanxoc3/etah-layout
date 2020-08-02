from pynput.keyboard import Key

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

        # modifiers
        # "cde"    : function on,
        # "cdf"    : ctrl on,
        # "cdg"    : alt on,
        # "acd"    : mod on,
        "bcd"    : Key.shift,

        # numbers
        "cef"    : "1",
        "ceg"    : "2",
        "ace"    : "3",
        "bce"    : "4",
        "cfg"    : "5",
        "acf"    : "6",
        "bcf"    : "7",
        "acg"    : "8",
        "bcg"    : "9",
        "abc"    : "0",

        # arrows
        "def"    : Key.left,
        "deg"    : Key.down,
        "ade"    : Key.up,
        "bde"    : Key.right,

        # punctuation
        "dfg"    : "-",
        "adf"    : "`",
        "bdf"    : "=",
        "adg"    : "[",
        "bdg"    : "]",
        "abd"    : ";",
        "efg"    : "/",
        "aef"    : "'",
        "bef"    : "\\",
        "aeg"    : ",",
        "beg"    : ".",

        # Other keys
        "afg"    : Key.tab,
        "bfg"    : Key.enter,
        "abe"    : Key.insert,
        "abf"    : Key.delete,
        "abg"    : Key.esc,

        # Meta direction
        "cdef"   : Key.home,
        "cdeg"   : Key.page_down,
        "acde"   : Key.page_up,
        "bcde"   : Key.end,

        # Modifier combos
        # "cdfg"   : ctrl alt,
        # "acdf"   : ctrl mod,
        # "bcdf"   : ctrl shift,
        # "acdg"   : alt mod,
        # "bcdg"   : alt shift,
        # "abcd"   : mod shift,

        # Function keys
        # "cefg"   : <f1>,
        # "acef"   : <f2>,
        # "bcef"   : <f3>,
        # "aceg"   : <f4>,
        # "bceg"   : <f5>,
        # "abce"   : <f6>,
        # "defg"   : <f7>,
        # "adef"   : <f8>,
        # "bdef"   : <f9>,
        # "adeg"   : <f10>,
        # "bdeg"   : <f11>,
        # "abde"   : <f12>,

        # Don't know yet.
        # "acfg"   : ,
        # "bcfg"   : ,
        # "abcf"   : ,
        # "abcg"   : ,
        # "adfg"   : ,
        # "bdfg"   : ,
        # "abdf"   : ,
        # "abdg"   : ,
        # "aefg"   : ,
        # "befg"   : ,
        # "abef"   : ,
        # "abeg"   : ,

        # "abfg"   : cancel everything,
}

# Notes: (TODO: delete?)
# So, to change to a different key, press 7 notes at the same time. The lowest
# note will be interpreted as the starting note (which note the letter 'E' would
# end up as). Also, if more than 7 notes are played, the first unique 7 notes
# starting from the bottom are chosen.

# If you mess up typing a note, just make sure you press 5, or 6 unique notes.
# This will cancel out your current chord. Or you can press a note that is not in
# your key. That will also cancel out the chord. Remember that 7 unique notes at
# the same time will change the key.

# If you hold down the pedal, this means that every single note you press will be
# part of the next chord, the notes won't release until you release the pedal.

# Or, do I want the chord to cancel if you press play note that is not a part of
# your key? That could be good. It would make it so if you mess up, you are bad
# at piano, so it won't do anything, or you did it on purpose because you messed
# up your keystroke.

# Oh, pressing multiple notes or a wrong note (not in key) will also stop all
# modifiers.

# total: 119
