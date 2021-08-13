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
