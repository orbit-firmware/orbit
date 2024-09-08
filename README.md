# ![logo](https://github.com/rmk-firmware/rmk/blob/master/docs/public/logo-64x64.png?raw=true) RMK Firmware

📖 [Docs](https://rmk-firmware.github.io/rmk)


RMK is a rust keyboard firmware built for ease.  
The main selling pints are:
  1. that you can configure your keyboard trough the keyboards flash drive.
  2. that its fast and reliable since its built on rust.
  3. that adding your own keyboard is just a single config file.
  4. that it supports a lot of chipsets trough [embassy](https://github.com/embassy-rs/embassy)


Here is an example keymap:  
```rmk
layer 0
____________________________________________________________________________________________________
press      | esc    q      w      e      r      t      y      u      i      o      p      =
shifted    | `      ---    ---    ---    ---    ---    ---    ---    ---    ---    ---    "::"
____________________________________________________________________________________________________
press      | tab    a      s      d      f      g      h      j      k      l             ent
shifted    | ---    ---    ---    ---    ---    ---    ---    ---    ---    ---           bspc
____________________________________________________________________________________________________
press      | lsft   z      x      c      v      b      n      m      ,             .      del
shifted    | ---    ---    ---    ---    ---    ---    ---    ---    !             ?      ---
____________________________________________________________________________________________________
press      | lctl   lgui   lalt          space         space                /      -      _
shifted    | ---    ---    ---           ---           ---                  \      +      _
held       | ---    ---    ---           ml(1)         ml(2)                ---    ---    ---
mac        | lgui   lctl   ---           ---           ---                  ---    ---    ---
```
  
  
To get started, check out the [docs](https://rmk-firmware.github.io/rmk/getting-started.html)