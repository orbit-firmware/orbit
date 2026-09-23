# Keymap

A keymap defines how each key on a keyboard behaves across various layers and states.  
It acts as a flexible configuration tool that maps key presses to behaviors and actions.  

The keymap lives next to the keyboard config as `orbit/keyboards/<keyboard>.orbit`
(or `user/keymap.orbit`, which wins) and is compiled into the firmware.
Loading it from the keyboard's flash drive is planned.

Supported today: `layer N`, `press`, `shift` and `hold [ms]` rows, `---` (fall through to the
layer below), `xxx` (nothing), modifier wrappers like `c(z)`, `ml(n)`, `to(n)`, and combos.
Also:

- `tl(n)` toggles layer n on until pressed again (no modifier mask yet).
- `sk(key)` is a sticky key: tapped, it applies to the next key press only; held, it is a
  normal key. `skl(n)` does the same for layer n. Pressing a sticky key while one is armed
  cancels it.
- `cw` is caps word: letters are shifted and `-` becomes `_` until a key other than a
  letter, digit, `-`, backspace or delete is pressed, or `cw` again.
- `rep` sends the last keycode again.
- A `tap 2` row gives each key's double-tap entry (needs `[behaviors] tap`): a second
  press within `tapping_term` sends it; a single tap is sent when the term ends or the next
  key goes down, and a key held past the term sends its press.
- `boot` restarts an STM32 into its ROM USB bootloader (flash with `dfu-util`).

Combos:

```orbit
combo j k | esc   # keys named by their layer 0 press label, within settings.combo_term ms
```

Not yet: strings (`"::"`), `os` rows, mouse, rgb and `boot`/`stats`/`return`.
A `hold` key becomes the hold after its time or as soon as another key is pressed.


## Structure

```
Layer NUMBER
_____________________________  # optional row separator
press    | key1 key2 key3 key4 # minimum requirement for a row and also denotes the start of a row
behavior | ---  ---  ---  key4 # behavior for each key 
_____________________________  # optional row separator
press    | key1 key2 key3 key4 # new row
```








```orbit
# none    xxx
# trough  ---

layer 0
____________________________________________________________________________________________________
press      | esc    q      w      e      r      t      y      u      i      o      p      =
shift      | `      ---    ---    ---    ---    ---    ---    ---    ---    ---    ---    "::"
____________________________________________________________________________________________________
press      | tab    a      s      d      f      g      h      j      k      l             ent
shift      | ---    ---    ---    ---    ---    ---    ---    ---    ---    ---           bspc
____________________________________________________________________________________________________
press      | lsft   z      x      c      v      b      n      m      ,             .      del
shift      | ---    ---    ---    ---    ---    ---    ---    ---    !             ?      ---
____________________________________________________________________________________________________
press      | lctl   lgui   lalt          space         space                /      -      _
shift      | ---    ---    ---           ---           ---                  \      +      _
hold 200   | ---    ---    ---           ml(1)         ml(2)                ---    ---    ---
os mac     | lgui   lctl   ---           ---           ---                  ---    ---    ---


layer 1
____________________________________________________________________________________________________
press      | ---    home   up     end    [      ]      ---    ---    &      |      "->"   ---
shift      | ---    ---    ---    ---    <      >      ---    ---    "&&"   "||"   ---    ---
____________________________________________________________________________________________________
press      | ---    left   down   right  (      )      ---    ---    ;      "             bspc
shift      | ---    ---    ---    ---    {      }      ---    ---    :      '             ---
____________________________________________________________________________________________________
press      | ---    c(z)   c(x)   c(c)   c(v)   ---    ---    ---    ---           ---    ---
____________________________________________________________________________________________________
press      | ---    ---    ---           ---           ml(3)                ---    ---    ---


layer 2
____________________________________________________________________________________________________
press      | ---    1      2      3      ---    ---    ---    ---    ---    ---    ---    ---
____________________________________________________________________________________________________
press      | 0      4      5      6      ---    ---    ---    ---    ;      "             bspc
shift      | ---    ---    ---    ---    ---    ---    ---    ---    :      '             ---
____________________________________________________________________________________________________
press      | ---    7      8      9      ---    ---    ---    ---    ---           ---    ---
shift      | ---    ---    ---    ~      ---    ---    ---    ---    ---           ---    ---
____________________________________________________________________________________________________
press      | ---    ---    ---           ml(3)         ---                  ---    ---    ---


layer 3
____________________________________________________________________________________________________
press      | os     mb1    mcu    mb2    f1     f2     f3     ---    ---    ---    ---    boot
____________________________________________________________________________________________________
press      | ---    mcl    mcd    mcr    f4     f5     f6     ---    ---    ---           return
____________________________________________________________________________________________________
press      | ---    mwhu   mb3    mwhd   f7     f8     f9     f10    f11           f12    to(4) 
____________________________________________________________________________________________________
press      | ---    ---    ---           ---           ---                  rgb    rgbi   rgbt


layer 4
____________________________________________________________________________________________________
press      | 1      c      w      e      r      t      f5     f4     f3     f2     f1     esc
____________________________________________________________________________________________________
press      | 2      a      s      d      f      g      xxx    xxx    xxx    s(ent)        ent
____________________________________________________________________________________________________
press      | lsft   q      x      c      v      b      b      m      to(0)         up     lsft
____________________________________________________________________________________________________
press      | lctl   lalt   3             space         stats                left   down   right
```