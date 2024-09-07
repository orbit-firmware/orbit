---
layout: home

# https://vitepress.dev/reference/default-theme-home-page
hero:
  name: "RMK Documentation"
  tagline: Rust Mechanical Keyboard Firmware
  actions:
    - theme: brand
      text: Documentation
      link: /concepts
    - theme: alt
      text: Github
      link: https://github.com/rmk-firmware

features:
  - title: Easy
    details: The keymaps are incredibly easy compared to other firmwares.
  - title: Fast
    details: Built on Rust, its fast and reliable.
  - title: Github Actions
    details: Build online, no need to mess with developer environments



--- 

<div style="display: none">

# Docs

</div>

<div style="padding-bottom: 50px"></div>

## Example Keymaps

<div class="code-showcase-container">

<div class="code-showcase-0 visible">

```
keycodes german

feature tap
type keypress
delay 120

feature held
type keypress
trigger after
delay 200

feature shifted
type modifier
modifier_mask shift

feature mac
type os
identifier MacOS

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

</div>

<div class="code-showcase-1">

```
keycodes arabic

feature shifted
type modifier
modifier_mask shift

layer 0
____________________________________________________________________________________________________
press      | esc    غ      س      ء      آ      آ     آب      u      i      o      p      =
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

</div>

</div>

<client-only>

<script>
  if (typeof window !== 'undefined') {
    const max = 2;
    let idx = 0;
    const sec = 1000

    const cycle = (time = 3 * sec) => {
      window.clearTimeout(window.home_cycle_timer);
      window.home_cycle_timer = window.setTimeout(() => {
        const cur_el = document.querySelectorAll(".code-showcase-"+idx)[0];
        idx += 1;
        if (idx > max - 1) idx = 0;
        const el = document.querySelectorAll(".code-showcase-"+idx)[0];

        cur_el.classList.remove("visible")
        window.setTimeout(() => {
          el.classList.add("visible")
        }, 450);
        
        cycle(10 * sec);
      }, time);
    };

    cycle();
  }
 
</script>

</client-only>
