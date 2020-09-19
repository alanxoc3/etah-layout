#!/usr/bin/env python3

keyboard_export_layout = {
        "a":  0x04,
        "b":  0x05,
        "c":  0x06,
        "d":  0x07,
        "e":  0x08,
        "f":  0x09,
        "g":  0x0a,
        "h":  0x0b,
        "i":  0x0c,
        "j":  0x0d,
        "k":  0x0e,
        "l":  0x0f,
        "m":  0x10,
        "n":  0x11,
        "o":  0x12,
        "p":  0x13,
        "q":  0x14,
        "r":  0x15,
        "s":  0x16,
        "t":  0x17,
        "u":  0x18,
        "v":  0x19,
        "w":  0x1a,
        "x":  0x1b,
        "y":  0x1c,
        "z":  0x1d,

        "1":  0x1e,
        "2":  0x1f,
        "3":  0x20,
        "4":  0x21,
        "5":  0x22,
        "6":  0x23,
        "7":  0x24,
        "8":  0x25,
        "9":  0x26,
        "0":  0x27,

#define KEY_ENTER 0x28 // Keyboard Return (ENTER)
#define KEY_ESC 0x29 // Keyboard ESCAPE
#define KEY_BACKSPACE 0x2a // Keyboard DELETE (Backspace)
#define KEY_TAB 0x2b // Keyboard Tab
#define KEY_SPACE 0x2c // Keyboard Spacebar
#define KEY_MINUS 0x2d // Keyboard - and _
#define KEY_EQUAL 0x2e // Keyboard =3D and +
#define KEY_LEFTBRACE 0x2f // Keyboard [ and {
#define KEY_RIGHTBRACE 0x30 // Keyboard ] and }
#define KEY_BACKSLASH 0x31 // Keyboard \ and |
#define KEY_HASHTILDE 0x32 // Keyboard Non-US # and ~
#define KEY_SEMICOLON 0x33 // Keyboard ; and :
#define KEY_APOSTROPHE 0x34 // Keyboard ' and "
#define KEY_GRAVE 0x35 // Keyboard ` and ~
#define KEY_COMMA 0x36 // Keyboard , and &lt;
#define KEY_DOT 0x37 // Keyboard . and &gt;
#define KEY_SLASH 0x38 // Keyboard / and ?
#define KEY_CAPSLOCK 0x39 // Keyboard Caps Lock

#define KEY_F1 0x3a // Keyboard F1
#define KEY_F2 0x3b // Keyboard F2
#define KEY_F3 0x3c // Keyboard F3
#define KEY_F4 0x3d // Keyboard F4
#define KEY_F5 0x3e // Keyboard F5
#define KEY_F6 0x3f // Keyboard F6
#define KEY_F7 0x40 // Keyboard F7
#define KEY_F8 0x41 // Keyboard F8
#define KEY_F9 0x42 // Keyboard F9
#define KEY_F10 0x43 // Keyboard F10
#define KEY_F11 0x44 // Keyboard F11
#define KEY_F12 0x45 // Keyboard F12



        "1"     : ,
        "2"     : ,
        "3"     : ,
        "4"     : ,
        "5"     : ,
        "6"     : ,
        "7"     : ,
        "8"     : ,
        "9"     : ,
        "0"     : ,

        "space"     : "p",
        "backspace"     : "p",

        # c#: shift
        # d#: func
        # f#: ctrl
        # g#: super
        # a#: meta

        # numbers
        "abf"    : "-",
        "abg"    : "=",

        # arrows
        # "def"    : Key.left,
        # "deg"    : Key.down,
        # "ade"    : Key.up,
        # "bde"    : Key.right,

        # punctuation
        "adf"    : "`",
        "adg"    : "[",
        "bdg"    : "]",
        "efg"    : "/",
        "abd"    : ";",
        "aef"    : "'",
        "bef"    : "\\",
        "aeg"    : ",",
        "ceg"    : ".",
        "ceg"    : "/",

        # Other keys
        "afg"    : Key.tab,
        "bfg"    : Key.enter,
        "abe"    : Key.insert,
        "abf"    : Key.delete, # opposite backspace
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
        "bcd"    : "f1",
        "bce"    : "f2",
        "bcf"    : "f3",
        "bcg"    : "f4",
        "bde"    : "f5",
        "bdf"    : "f6",
        "bdg"    : "f7",
        "bef"    : "f8",
        "beg"    : "f9",
        "bfg"    : "f10",
        "abf"    : "f11",
        "abg"    : "f12",

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

