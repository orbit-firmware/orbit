// Compiles build/keymap.orbit (docs/keymap.md) into KEYMAP, COMBOS and COMBO_TERM.
//
//   layer 0                       layers in order, each lists every key once
//   press     | q  w  ---  xxx    starts a row group: what each key sends
//   shift     | Q  !  ---  ---    replaces press while shift is held (--- = no replacement)
//   hold 200  | ml(1) ...         sent instead when held for 200 ms (default: tapping_term)
//   combo j k | esc               j + k pressed together send esc (labels = layer 0 press)
//
// Entries: a keycode name or alias, a modifier wrapper like c(z), `---` (fall through to
// the layer below), `xxx` (nothing), `ml(n)` (layer n while held), `to(n)` (switch base layer).

use crate::generators::keycodes::{self, KeyCode};
use crate::modifiers;
use crate::toml;
use crate::util;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Entry {
  Trough,
  None,
  Code(u16),
  Layer(u8),
  To(u8),
  Toggle(u8),
  Sticky(u16),
  StickyLayer(u8),
  CapsWord,
  Repeat,
  Boot,
}

#[derive(Clone, Copy)]
struct Slot {
  press: Entry,
  shift: Entry,
  hold: Entry,
  hold_ms: u16,
}

pub struct Keymap {
  layers: Vec<Vec<Slot>>,
  combos: Vec<(Vec<usize>, Entry)>,
}

fn fail(line: usize, msg: String) -> ! {
  println!("keymap.orbit line {}: {}", line, msg);
  std::process::exit(1);
}

fn modifier(name: &str) -> Option<fn(u16) -> u16> {
  Some(match name {
    "c" | "lc" => modifiers::lc,
    "s" | "ls" => modifiers::ls,
    "a" | "la" => modifiers::la,
    "g" | "lg" => modifiers::lg,
    "rc" => modifiers::rc,
    "rs" => modifiers::rs,
    "ra" => modifiers::ra,
    "rg" => modifiers::rg,
    _ => return None,
  })
}

fn code(token: &str, codes: &[KeyCode]) -> Option<u16> {
  if let Some((wrap, inner)) = token.strip_suffix(')').and_then(|t| t.split_once('(')) {
    if let Some(apply) = modifier(wrap) {
      return code(inner, codes).map(apply);
    }
  }
  if let Some(hex) = token.strip_prefix("0x") {
    return u16::from_str_radix(hex, 16).ok();
  }
  let lower = token.to_lowercase();
  let found = codes.iter().find(|c| c.alias_list.iter().any(|a| *a == token || *a == lower));
  found.map(|c| c.code)
}

fn entry(token: &str, codes: &[KeyCode], line: usize) -> Entry {
  let arg = |prefix: &str| token.strip_prefix(prefix)?.strip_suffix(')')?.parse::<u8>().ok();
  match token {
    "---" => Entry::Trough,
    "xxx" => Entry::None,
    "cw" => Entry::CapsWord,
    "rep" => Entry::Repeat,
    "boot" => Entry::Boot,
    _ if token.len() > 1 && token.starts_with('"') => fail(line, format!("strings are not supported yet: {}", token)),
    _ => {
      if let Some(n) = arg("ml(") {
        Entry::Layer(n)
      } else if let Some(n) = arg("to(") {
        Entry::To(n)
      } else if let Some(n) = arg("tl(") {
        Entry::Toggle(n)
      } else if let Some(n) = arg("skl(") {
        Entry::StickyLayer(n)
      } else if let Some(c) = token.strip_prefix("sk(").and_then(|t| t.strip_suffix(')')).and_then(|t| code(t, codes)) {
        Entry::Sticky(c)
      } else if let Some(c) = code(token, codes) {
        Entry::Code(c)
      } else {
        fail(line, format!("unknown key `{}`", token))
      }
    }
  }
}

pub fn parse(text: &str, key_count: usize, tapping_term: u16, codes: &[KeyCode]) -> Keymap {
  let mut layers: Vec<Vec<Slot>> = vec![];
  let mut combos = vec![];
  let mut combo_src = vec![];
  let mut group = 0;

  for (i, raw) in text.lines().enumerate() {
    let n = i + 1;
    let line = raw.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with('_') {
      continue;
    }
    if let Some(num) = line.strip_prefix("layer ") {
      if num.trim().parse::<usize>().ok() != Some(layers.len()) {
        fail(n, format!("expected `layer {}`", layers.len()));
      }
      layers.push(vec![]);
      continue;
    }

    let Some((head, tail)) = line.split_once('|') else {
      fail(n, "expected `<row> | <keys>`".to_string());
    };
    let head: Vec<&str> = head.split_whitespace().collect();
    let tokens: Vec<&str> = tail.split_whitespace().collect();

    if head[0] == "combo" {
      if tokens.len() != 1 || head.len() < 3 {
        fail(n, "expected `combo <key> <key>.. | <entry>`".to_string());
      }
      combo_src.push((n, head[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(), entry(tokens[0], codes, n)));
      continue;
    }

    let Some(layer) = layers.last_mut() else {
      fail(n, "rows must follow `layer 0`".to_string());
    };
    match head[0] {
      "press" => {
        group = layer.len();
        for t in tokens {
          let press = entry(t, codes, n);
          layer.push(Slot { press, shift: Entry::Trough, hold: Entry::Trough, hold_ms: tapping_term });
        }
      }
      "shift" | "hold" => {
        if tokens.len() != layer.len() - group {
          fail(n, format!("{} entries, the press row above has {}", tokens.len(), layer.len() - group));
        }
        let ms = match head.get(1) {
          Some(ms) => ms.parse().unwrap_or_else(|_| fail(n, format!("bad hold time `{}`", ms))),
          None => tapping_term,
        };
        for (slot, t) in layer[group..].iter_mut().zip(tokens) {
          let e = entry(t, codes, n);
          if head[0] == "shift" {
            slot.shift = e;
          } else {
            slot.hold = e;
            slot.hold_ms = ms;
          }
        }
      }
      other => fail(n, format!("unsupported row `{}` (press, shift, hold, combo)", other)),
    }
  }

  if layers.is_empty() {
    fail(0, "no `layer 0`".to_string());
  }
  // the engine keeps active layers in a u32
  if layers.len() > 32 {
    fail(0, format!("{} layers, at most 32", layers.len()));
  }
  for (l, layer) in layers.iter().enumerate() {
    if layer.len() != key_count {
      fail(0, format!("layer {} has {} keys, the layout has {}", l, layer.len(), key_count));
    }
  }

  // combo keys are named by their layer 0 press label
  let labels: Vec<String> = text
    .lines()
    .skip_while(|l| !l.trim().starts_with("layer"))
    .skip(1)
    .take_while(|l| !l.trim().starts_with("layer"))
    .filter_map(|l| l.split_once('|').filter(|(h, _)| h.trim() == "press").map(|(_, t)| t.to_string()))
    .flat_map(|t| t.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>())
    .collect();
  for (n, names, e) in combo_src {
    let keys: Vec<usize> = names
      .iter()
      .map(|name| {
        let at: Vec<usize> = labels.iter().enumerate().filter(|(_, l)| *l == name).map(|(i, _)| i).collect();
        if at.len() != 1 {
          fail(n, format!("combo key `{}` must be exactly one layer 0 press label", name));
        }
        at[0]
      })
      .collect();
    combos.push((keys, e));
  }

  let all = layers.iter().flatten().flat_map(|s| [s.press, s.shift, s.hold]).chain(combos.iter().map(|c| c.1));
  for e in all {
    if let Entry::Layer(l) | Entry::To(l) | Entry::Toggle(l) | Entry::StickyLayer(l) = e {
      if l as usize >= layers.len() {
        fail(0, format!("layer {} is used but not defined", l));
      }
    }
  }

  Keymap { layers, combos }
}

fn tokens(e: Entry) -> TokenStream {
  match e {
    Entry::Trough => quote! { Entry::Trough },
    Entry::None => quote! { Entry::None },
    Entry::Code(c) => quote! { Entry::Code(#c) },
    Entry::Layer(l) => quote! { Entry::Layer(#l) },
    Entry::To(l) => quote! { Entry::To(#l) },
    Entry::Toggle(l) => quote! { Entry::Toggle(#l) },
    Entry::Sticky(c) => quote! { Entry::Sticky(#c) },
    Entry::StickyLayer(l) => quote! { Entry::StickyLayer(#l) },
    Entry::CapsWord => quote! { Entry::CapsWord },
    Entry::Repeat => quote! { Entry::Repeat },
    Entry::Boot => quote! { Entry::Boot },
  }
}

pub fn generate(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let root = util::get_root();
  let config = toml::read(&format!("{}/build/keyboard.toml", root), true);
  let layout: Vec<(usize, usize)> = if toml::contains(&config, "matrix") {
    toml::get(&config, "matrix/layout", true)
  } else {
    toml::get(&config, "multiplexers/layout", true)
  };
  let tapping_term: u16 = toml::get(&config, "settings/tapping_term", true);
  let combo_term: u16 = match toml::get(&config, "settings/combo_term", false) {
    0 => 50,
    ms => ms,
  };

  // 0 (unset) = off
  let quick_tap_term: u16 = toml::get(&config, "settings/quick_tap_term", false);

  let path = format!("{}/build/keymap.orbit", root);
  let text = std::fs::read_to_string(&path).unwrap_or_else(|_| {
    println!("Missing keymap: add orbit/keyboards/<keyboard>.orbit or user/keymap.orbit");
    std::process::exit(1);
  });
  let map = parse(&text, layout.len(), tapping_term, &keycodes::load());

  // features are switched on in the keyboard toml; using one that is off is an error
  let cargo = toml::read("Cargo.toml", true);
  let enabled: Vec<&str> = cargo["features"]["default"].as_array().unwrap().iter().filter_map(|f| f.as_str()).collect();
  let slots = || map.layers.iter().flatten();
  let entries = || slots().flat_map(|s| [s.press, s.shift, s.hold]).chain(map.combos.iter().map(|c| c.1));
  let uses = [
    ("behavior_hold_enabled", "[behaviors] hold", slots().any(|s| s.hold != Entry::Trough)),
    ("behavior_combo_enabled", "[behaviors] combo", !map.combos.is_empty()),
    ("action_layers_enabled", "[actions] layers", entries().any(|e| matches!(e, Entry::Layer(_) | Entry::To(_) | Entry::Toggle(_) | Entry::StickyLayer(_)))),
  ];
  for (feature, key, used) in uses {
    if used && !enabled.contains(&feature) {
      println!("keymap.orbit uses {} but it is not enabled in the keyboard toml", key);
      std::process::exit(1);
    }
  }

  let layer_count = map.layers.len();
  let key_count = layout.len();
  let layers = map.layers.iter().map(|layer| {
    let slots = layer.iter().map(|s| {
      let (press, shift, hold, hold_ms) = (tokens(s.press), tokens(s.shift), tokens(s.hold), s.hold_ms);
      quote! { Slot { press: #press, shift: #shift, hold: #hold, hold_ms: #hold_ms } }
    });
    quote! { [#(#slots),*] }
  });
  let combos = map.combos.iter().map(|(keys, e)| {
    let e = tokens(*e);
    quote! { Combo { keys: &[#(#keys),*], entry: #e } }
  });
  let combo_count = map.combos.len();

  quote! {
    pub const LAYER_COUNT: usize = #layer_count;
    pub const KEYMAP: [[Slot; #key_count]; #layer_count] = [#(#layers),*];
    pub const COMBOS: [Combo; #combo_count] = [#(#combos),*];
    pub const COMBO_TERM: u16 = #combo_term;
    pub const QUICK_TAP_TERM: u16 = #quick_tap_term;
  }
  .into()
}

#[cfg(test)]
mod tests {
  use super::*;
  use proc_macro2::{Ident, Span};

  fn codes() -> Vec<KeyCode> {
    [("Q", 0x14, vec!["q"]), ("W", 0x1A, vec!["w"]), ("Escape", 0x29, vec!["escape", "esc"])]
      .into_iter()
      .map(|(name, code, aliases)| KeyCode {
        name: Ident::new(name, Span::call_site()),
        code_str: String::new(),
        code,
        alias_list: aliases.into_iter().map(String::from).collect(),
      })
      .collect()
  }

  #[test]
  fn parses_rows_layers_and_combos() {
    let text = "layer 0\npress | q w\nshift | --- esc\nhold 150 | ml(1) ---\ncombo q w | esc\nlayer 1\npress | c(w) xxx\n";
    let map = parse(text, 2, 200, &codes());
    let l0 = &map.layers[0];
    assert_eq!(l0[0].press, Entry::Code(0x14));
    assert_eq!(l0[0].hold, Entry::Layer(1));
    assert_eq!(l0[0].hold_ms, 150);
    assert_eq!(l0[1].shift, Entry::Code(0x29));
    assert_eq!(l0[1].hold, Entry::Trough);
    assert_eq!(map.layers[1][0].press, Entry::Code(0x011A));
    assert_eq!(map.layers[1][1].press, Entry::None);
    assert_eq!(map.combos, vec![(vec![0, 1], Entry::Code(0x29))]);
  }

  #[test]
  fn thirty_two_layers_fit() {
    let text: String = (0..32).map(|l| format!("layer {}\npress | q\n", l)).collect();
    assert_eq!(parse(&text, 1, 200, &codes()).layers.len(), 32);
  }

  #[test]
  fn parses_qmk_zmk_style_keys() {
    let text = "layer 0\npress | tl(1) sk(c(q)) skl(1) cw rep boot\nlayer 1\npress | --- --- --- --- --- ---\n";
    let l0 = &parse(text, 6, 200, &codes()).layers[0];
    let press: Vec<Entry> = l0.iter().map(|s| s.press).collect();
    let want = [Entry::Toggle(1), Entry::Sticky(0x0114), Entry::StickyLayer(1), Entry::CapsWord, Entry::Repeat, Entry::Boot];
    assert_eq!(press, want);
  }
}
