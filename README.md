# Hatel
Hatel is a keyboard layout that maps keys on a piano to keys on a keyboard in
an efficient and somewhat pleasant sounding way.

Here is the specification so far:

| Piano Chord | US Keyboard Mapping |
| :-:         | :-:                 |
| A           | k                   |
| B           | l                   |
| C           | e                   |
| D           | t                   |
| E           | a                   |
| F           | h                   |
| G           | j                   |
| AB          | \<backspace\>       |
| AC          | n                   |
| AD          | d                   |
| AE          | p                   |
| AF          | g                   |
| AG          | z                   |
| BC          | v                   |
| BD          | c                   |
| BE          | w                   |
| BF          | y                   |
| BG          | u                   |
| CD          | x                   |
| CE          | o                   |
| CF          | i                   |
| CG          | \<space\>           |
| DE          | q                   |
| DF          | s                   |
| DG          | r                   |
| EF          | b                   |
| EG          | m                   |
| FG          | f                   |
| ABC         | \<insert\>          |
| ABD         | [                   |
| ABF         | ]                   |
| ABG         | \<delete\>          |
| ACE         | \<tab\>             |
| AFG         | \<esc\>             |
| BCD         | 1                   |
| BCE         | 2                   |
| BCF         | 3                   |
| BCG         | 4                   |
| BDE         | 5                   |
| BDF         | 6                   |
| BDG         | 7                   |
| BEF         | 8                   |
| BEG         | 9                   |
| BFG         | 0                   |
| CDE         | -                   |
| CDF         | /                   |
| CDG         | ,                   |
| CEF         | '                   |
| CEG         | \<enter\>           |
| CFG         | .                   |
| DEF         | ;                   |
| DEG         | `                   |
| DFG         | \\                  |
| EFG         | =                   |

`D#` represents the shift key. The remaining black keys will be reserved for:
- CMD/Super/Windows
- Alt
- Ctrl
- Fn (programmable mappings)

Some other mappings:
- Releasing four different white key letters will cancel the current modifer.
- Releasing 5, 6, or all 7 white keys will disable hatel.
- If hatel is disabled, pressing all 5 black keys will re-enable hatel.

Disabling hatel is useful if you need to record midi, you want to play an
actual piano in the middle of your typing session.

The function key is used to switch modes. Hitting a white key after the
function key switches to that mode. To leave any mode, you can hit the function
key then the previous mode, hit four white keys, hit five or more white keys
(disabling hatel too), or you can hit all black keys. Pressing a modifier
before pressing the function key will result in doing nothing special.

Some modes that will probably be implemented:
- Mouse mode (left, middle, right, grid-like movement similar to [keynav](https://github.com/jordansissel/keynav))
- Navigation mode (arrows, page down/up, home, end)
- Other modes probably wouldn't be official, but it should be configurable.
