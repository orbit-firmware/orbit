//
// default mode
//
#[cfg(not(feature = "chip_type_emulator"))]
mod stm32 {
  use embassy_usb::driver::Driver;

  use crate::orbit::keyboard::Keyboard;

  pub async fn run<D: Driver<'static>>(driver: D) {
    Keyboard::new().process(driver).await;
  }
}

#[cfg(not(feature = "chip_type_emulator"))]
pub use stm32::run;

//
// emulator mode
//
#[cfg(feature = "chip_type_emulator")]
mod emulator {
  use crossterm::event::{self, Event, KeyCode as TermKey, KeyModifiers};
  use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
  use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
  use crossterm::{cursor, queue, ExecutableCommand};
  use std::io::Write;
  use std::time::{Duration, Instant};

  use crate::orbit::config as Orbit;
  use crate::orbit::keyboard::Keyboard;
  use crate::orbit::keycodes::KeyCode;
  use crate::orbit::keymap::Entry;
  use crate::orbit::peripherals::{REAL_KEYS, SIMULATED};
  use std::sync::atomic::Ordering;

  // host keys (the physical positions in _emulator.toml) and report contents by keycode name
  enum Step {
    Down(&'static [&'static str]),
    Up(&'static [&'static str]),
    Wait(u64),
    // the latest report holds exactly these keycodes and modifiers
    Expect(&'static [&'static str]),
  }
  use Step::*;

  // written against orbit/keyboards/_emulator.orbit
  const T: u64 = 30; // > debounce_time, < combo_term
  const HOLD: u64 = 260; // > hold 200
  const TESTS: &[(&str, &[Step])] = &[
    ("single key", &[Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Expect(&[])]),
    ("chord across halves", &[Down(&["F", "J"]), Wait(60), Expect(&["F", "J"]), Up(&["F", "J"]), Wait(T), Expect(&[])]),
    ("modifier key", &[Down(&["Tab"]), Wait(T), Down(&["Z"]), Expect(&["LeftShift", "Z"]), Up(&["Z", "Tab"]), Wait(T), Expect(&[])]),
    ("7 keys: boot report keeps 6", &[Down(&["Q", "W", "E", "R", "T", "Y", "U"]), Expect(&["Q", "W", "E", "R", "T", "Y"]), Wait(T), Up(&["Q", "W", "E", "R", "T", "Y", "U"]), Wait(T), Expect(&[])]),
    ("shift row: shift+/ sends \\", &[Down(&["Tab"]), Wait(T), Down(&["Slash"]), Expect(&["Backslash"]), Wait(T), Up(&["Slash"]), Expect(&["LeftShift"]), Wait(T), Up(&["Tab"]), Expect(&[])]),
    ("shift row: shift+bsp sends del", &[Down(&["Tab"]), Wait(T), Down(&["Backspace"]), Expect(&["Delete"]), Wait(T), Up(&["Backspace", "Tab"]), Wait(T), Expect(&[])]),
    ("no shift row: bsp alone", &[Down(&["Backspace"]), Expect(&["Backspace"]), Wait(T), Up(&["Backspace"]), Expect(&[])]),
    ("hold-tap: tap sends space", &[Down(&["Space"]), Wait(T), Expect(&[]), Up(&["Space"]), Expect(&["Space"]), Wait(T), Expect(&[])]),
    ("hold-tap: hold reaches layer 1", &[Down(&["Space"]), Wait(HOLD), Down(&["Q"]), Expect(&["One"]), Wait(T), Up(&["Q"]), Wait(T), Up(&["Space"]), Expect(&[]), Wait(T), Expect(&[])]),
    ("hold-tap: other key decides hold", &[Down(&["Space"]), Wait(T), Down(&["W"]), Expect(&["Two"]), Wait(T), Up(&["W", "Space"]), Wait(T), Expect(&[])]),
    ("layer 1 arrows (j waits out the combo term)", &[Down(&["Space"]), Wait(HOLD), Down(&["J"]), Expect(&[]), Wait(60), Expect(&["Down"]), Wait(T), Up(&["J", "Space"]), Wait(T), Expect(&[])]),
    ("--- falls through to layer 0", &[Down(&["Space"]), Wait(HOLD), Down(&["Z"]), Expect(&["Z"]), Wait(T), Up(&["Z", "Space"]), Wait(T), Expect(&[])]),
    ("held key keeps its layer", &[Down(&["Space"]), Wait(HOLD), Down(&["E"]), Wait(T), Up(&["Space"]), Wait(T), Expect(&["Three"]), Up(&["E"]), Wait(T), Expect(&[])]),
    ("combo j+k sends esc", &[Down(&["J", "K"]), Expect(&["Escape"]), Wait(T), Up(&["J"]), Expect(&[]), Wait(T), Up(&["K"]), Wait(T), Expect(&[])]),
    ("combo d+f pressed apart within term", &[Down(&["D"]), Wait(10), Expect(&[]), Down(&["F"]), Expect(&["Tab"]), Wait(T), Up(&["D", "F"]), Wait(T), Expect(&[])]),
    ("combo key tapped within term", &[Down(&["J"]), Wait(T), Up(&["J"]), Expect(&["J"]), Wait(T), Expect(&[])]),
    ("combo key alone after term", &[Down(&["J"]), Wait(80), Expect(&["J"]), Up(&["J"]), Wait(T), Expect(&[])]),
    ("combo key then other key", &[Down(&["J"]), Wait(10), Down(&["Q"]), Expect(&["J", "Q"]), Wait(T), Up(&["J", "Q"]), Wait(T), Expect(&[])]),
    ("to(3) switches base, to(0) back", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Q"]), Expect(&["F1"]), Wait(T), Up(&["Q"]), Wait(T),
      Down(&["P"]), Wait(T), Up(&["P"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("enter tap still sends enter", &[Down(&["Enter"]), Wait(T), Up(&["Enter"]), Expect(&["Enter"]), Wait(T), Expect(&[])]),
    // layer 2 (enter held): w tl(1), e sk(lsft), r skl(1), t cw, y rep
    ("tl(1) toggles layer 1 on and off", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["W"]), Wait(T), Up(&["W"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Q"]), Expect(&["One"]), Wait(T), Up(&["Q"]), Wait(T),
      Down(&["Enter"]), Wait(HOLD), Down(&["W"]), Wait(T), Up(&["W"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("sk(lsft) tapped shifts the next key only", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["E"]), Wait(T), Up(&["E"]), Wait(T), Up(&["Enter"]), Wait(T), Expect(&[]),
      Down(&["Q"]), Expect(&["LeftShift", "Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
      Down(&["W"]), Expect(&["W"]), Wait(T), Up(&["W"]), Wait(T), Expect(&[]),
    ]),
    ("sk(lsft) held is a normal shift", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["E"]), Wait(T), Down(&["Z"]), Expect(&["LeftShift", "Z"]), Wait(T),
      Up(&["Z", "E", "Enter"]), Wait(T), Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("skl(1) takes the next key from layer 1", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["R"]), Wait(T), Up(&["R"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Q"]), Expect(&["One"]), Wait(T), Up(&["Q"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("caps word shifts letters until space", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["T"]), Wait(T), Up(&["T"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["A"]), Expect(&["LeftShift", "A"]), Wait(T), Up(&["A"]), Wait(T),
      Down(&["Backspace"]), Expect(&["Backspace"]), Wait(T), Up(&["Backspace"]), Wait(T),
      Down(&["Q"]), Expect(&["LeftShift", "Q"]), Wait(T), Up(&["Q"]), Wait(T),
      Down(&["Space"]), Wait(T), Up(&["Space"]), Expect(&["Space"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("sk(lsft) then a tapped hold-tap key", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["E"]), Wait(T), Up(&["E"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Space"]), Wait(T), Up(&["Space"]), Expect(&["LeftShift", "Space"]), Wait(T), Expect(&[]),
    ]),
    ("sk(lsft) then a combo key tapped", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["E"]), Wait(T), Up(&["E"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["J"]), Wait(T), Up(&["J"]), Expect(&["LeftShift", "J"]), Wait(T), Expect(&[]),
    ]),
    ("second sticky key cancels the armed one", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["E"]), Wait(T), Up(&["E"]), Wait(T), Down(&["E"]), Wait(T), Up(&["E"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("combo c+v holds layer 1", &[
      Down(&["C", "V"]), Wait(T), Down(&["Q"]), Expect(&["One"]), Wait(T), Up(&["Q", "C", "V"]), Wait(T), Expect(&[]),
    ]),
    ("combo esc ends caps word, rep repeats it", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["T"]), Wait(T), Up(&["T"]), Wait(T), Up(&["Enter"]), Wait(T),
      Down(&["J", "K"]), Expect(&["Escape"]), Wait(T), Up(&["J", "K"]), Wait(T),
      Down(&["Q"]), Expect(&["Q"]), Wait(T), Up(&["Q"]), Wait(T),
      Down(&["Enter"]), Wait(HOLD), Down(&["Y"]), Expect(&["Q"]), Wait(T), Up(&["Y", "Enter"]), Wait(T), Expect(&[]),
    ]),
    ("quick tap: tap then hold space holds space", &[
      Down(&["Space"]), Wait(T), Up(&["Space"]), Wait(T),
      Down(&["Space"]), Wait(HOLD), Expect(&["Space"]), Down(&["Q"]), Expect(&["Space", "Q"]), Wait(T),
      Up(&["Q", "Space"]), Wait(T), Expect(&[]),
    ]),
    ("tap dance: double tap b sends esc", &[
      Down(&["B"]), Wait(T), Up(&["B"]), Expect(&[]), Wait(T), Down(&["B"]), Expect(&["Escape"]), Wait(T),
      Up(&["B"]), Wait(T), Expect(&[]),
    ]),
    ("tap dance: single tap b goes out alone, then the next key", &[
      Down(&["B"]), Wait(T), Up(&["B"]), Wait(T), Expect(&[]), Down(&["Q"]), Expect(&["B"]), Wait(0), Expect(&["Q"]), Wait(T),
      Up(&["Q"]), Wait(T), Expect(&[]),
    ]),
    ("tap dance: b held past the term is b", &[
      Down(&["B"]), Wait(HOLD), Expect(&["B"]), Up(&["B"]), Wait(T), Expect(&[]),
    ]),
    ("string types one character per report", &[
      Down(&["Enter"]), Wait(HOLD), Down(&["U"]), Expect(&["A"]), Wait(0), Expect(&[]),
      Wait(0), Expect(&["Space"]), Wait(0), Expect(&[]), Wait(0), Expect(&["B"]), Wait(0), Expect(&[]),
      Up(&["U", "Enter"]), Wait(T), Expect(&[]),
    ]),
    ("rep sends the last key again", &[
      Down(&["X"]), Wait(T), Up(&["X"]), Wait(T),
      Down(&["Enter"]), Wait(HOLD), Down(&["Y"]), Expect(&["X"]), Wait(T), Up(&["Y", "Enter"]), Wait(T), Expect(&[]),
    ]),
  ];
  const SHOW_MS: u64 = 600;
  const MODIFIERS: [&str; 8] = ["LeftCtrl", "LeftShift", "LeftAlt", "LeftGui", "RightCtrl", "RightShift", "RightAlt", "RightGui"];

  // what a screen shows: pressed positions, the report, the status line, expected names
  struct Frame {
    pressed: Vec<bool>,
    // key names of what each key sends now, matched against the report
    labels: Vec<String>,
    // the same with held modifiers and caps word applied, as drawn
    display: Vec<String>,
    // active layers, sticky key, caps word, modifiers
    state: String,
    report: [u8; 8],
    status: String,
    expect: Option<Vec<String>>,
  }

  pub async fn run() -> ! {
    let mut keyboard = Keyboard::new();

    // ORBIT_EMULATOR_TEST=1: run the tests without a screen and exit 1 on failure
    if std::env::var_os("ORBIT_EMULATOR_TEST").is_some() {
      // the lower board's labels: a / A, 1 / ! under shift, caps word, shift row, ctrl prefix
      let (a, one, lsft, lctl) = (Entry::Code(0x04), Entry::Code(0x1E), 0x02, 0x01);
      let shown_ok = shown(a, false, 0, false) == "a"
        && shown(a, false, lsft, false) == "A"
        && shown(a, false, 0, true) == "A"
        && shown(one, false, lsft, false) == "!"
        && shown(one, false, 0, true) == "1"
        && shown(Entry::Code(0x31), true, lsft, false) == "\\"
        && shown(a, false, lctl, false) == "c-a";
      println!("display labels: {}", if shown_ok { "PASS" } else { "FAIL" });
      if !shown_ok {
        std::process::exit(1);
      }
      let failed = run_tests(&mut keyboard, &mut |f| println!("{}", f.status));
      std::process::exit(if failed.is_empty() { 0 } else { 1 });
    }

    let mut out = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(EnterAlternateScreen).unwrap();
    out.execute(cursor::Hide).unwrap();

    let mut summary = play_tests(&mut keyboard);
    let mut last: Option<(Vec<bool>, Vec<String>, String, [u8; 8], (u16, u16))> = None;
    loop {
      // drain terminal input so typed keys don't leak into the shell; ctrl+c quits
      while event::poll(Duration::ZERO).unwrap_or(false) {
        if let Ok(Event::Key(k)) = event::read() {
          if k.code == TermKey::Char('c') && k.modifiers.contains(KeyModifiers::CONTROL) {
            out.execute(cursor::Show).unwrap();
            out.execute(LeaveAlternateScreen).unwrap();
            terminal::disable_raw_mode().unwrap();
            std::process::exit(0);
          }
          if k.code == TermKey::Char('r') && k.modifiers.contains(KeyModifiers::CONTROL) {
            summary = play_tests(&mut keyboard);
            last = None;
          }
        }
      }

      let report = keyboard.tick().serialize();
      let size = terminal::size().unwrap_or((80, 24));
      let frame = snapshot(&keyboard, report, summary.clone(), None);
      let state = (frame.pressed.clone(), frame.display.clone(), frame.state.clone(), report, size);
      if last.as_ref() != Some(&state) {
        draw(&mut out, &frame, size);
        last = Some(state);
      }
      std::thread::sleep(Duration::from_millis(1));
    }
  }

  // runs the tests on screen and returns the line shown afterwards
  fn play_tests(keyboard: &mut Keyboard) -> String {
    let failed = run_tests(keyboard, &mut |f| {
      draw(&mut std::io::stdout(), &f, terminal::size().unwrap_or((80, 24)));
      std::thread::sleep(Duration::from_millis(SHOW_MS));
    });
    // key presses made while the tests ran are not commands
    while event::poll(Duration::ZERO).unwrap_or(false) {
      let _ = event::read();
    }
    if failed.is_empty() {
      format!("tests: {0}/{0} passed. ctrl+r reruns, ctrl+c quits", TESTS.len())
    } else {
      format!("tests: {} failed ({}). ctrl+r reruns, ctrl+c quits", failed.len(), failed.join(", "))
    }
  }

  // plays each test's steps in real time; `show` gets one frame per test: its first failing
  // expectation, else its last one that expects keys
  fn run_tests(keyboard: &mut Keyboard, show: &mut dyn FnMut(Frame)) -> Vec<&'static str> {
    let mut failed = vec![];
    REAL_KEYS.store(false, Ordering::Relaxed);
    for (i, (name, steps)) in TESTS.iter().enumerate() {
      let mut report = settle(keyboard, 60);
      let mut shown: Option<Frame> = None;
      for (s, step) in steps.iter().enumerate() {
        match step {
          Down(keys) => {
            SIMULATED.lock().unwrap().extend(keys.iter().map(|k| k.to_string()));
            report = settle(keyboard, 0);
          }
          Up(keys) => {
            SIMULATED.lock().unwrap().retain(|k| !keys.contains(&k.as_str()));
            report = settle(keyboard, 0);
          }
          Wait(ms) => report = settle(keyboard, *ms),
          Expect(want) => {
            // in report order: modifiers, then keys in the order they went down (hosts
            // type newly pressed keys in array order)
            let got = names(&report);
            let want: Vec<String> = want.iter().map(|w| w.to_string()).collect();
            let ok = got == want;
            let verdict = if ok { "PASS".to_string() } else { format!("FAIL: got [{}]", got.join(" ")) };
            let status = format!("test {}/{}: {}  step {}: expect [{}]  {}", i + 1, TESTS.len(), name, s + 1, want.join(" "), verdict);
            let keep = shown.as_ref().is_some_and(|f| f.status.contains("FAIL") || (want.is_empty() && f.expect.as_ref().is_some_and(|w| !w.is_empty())));
            if !keep {
              shown = Some(snapshot(keyboard, report, status, Some(want)));
            }
          }
        }
      }
      SIMULATED.lock().unwrap().clear();
      settle(keyboard, 60);
      let frame = shown.expect("every test has an Expect step");
      if frame.status.contains("FAIL") {
        failed.push(*name);
      }
      show(frame);
    }
    REAL_KEYS.store(true, Ordering::Relaxed);
    failed
  }

  fn settle(keyboard: &mut Keyboard, ms: u64) -> [u8; 8] {
    let end = Instant::now() + Duration::from_millis(ms);
    let mut report = keyboard.tick().serialize();
    while Instant::now() < end {
      std::thread::sleep(Duration::from_millis(1));
      report = keyboard.tick().serialize();
    }
    report
  }

  fn names(report: &[u8; 8]) -> Vec<String> {
    let mods = (0..8).filter(|b| report[0] & 1 << b != 0).map(|b| MODIFIERS[b].to_string());
    let keys = report[2..].iter().filter(|&&c| c != 0).map(|&c| format!("{:?}", KeyCode::from_u16(c as u16)));
    mods.chain(keys).collect()
  }

  fn label(e: Entry) -> String {
    match e {
      Entry::Code(c) => {
        let name = format!("{:?}", KeyCode::from_u16(c));
        if name == "None" { format!("{:#06x}", c) } else { name }
      }
      Entry::Layer(l) => format!("ml{}", l),
      Entry::To(l) => format!("to{}", l),
      Entry::Toggle(l) => format!("tl{}", l),
      Entry::Sticky(c) => format!("sk {}", label(Entry::Code(c))),
      Entry::StickyLayer(l) => format!("skl{}", l),
      Entry::CapsWord => "cw".to_string(),
      Entry::Repeat => "rep".to_string(),
      Entry::Boot => "boot".to_string(),
      Entry::Str(_) => "str".to_string(),
      Entry::Trough | Entry::None => String::new(),
    }
  }

  // what a key shows under the held modifiers: shifted symbols by name, letters upper case
  // when shifted, other modifiers as c- a- g- prefixes
  fn shown(entry: Entry, replaced: bool, mods: u8, caps_word: bool) -> String {
    let Entry::Code(c) = entry else { return short(label(entry)) };
    let key = c as u8;
    if (0xE0..=0xE7).contains(&key) {
      return short(label(entry));
    }
    // a shift row entry is sent without the held shift
    let held = if replaced { mods & !0x22 } else { mods };
    let mut mods = (c >> 8) as u8 | held;
    if caps_word && matches!(key, 0x04..=0x1D | 0x2D) {
      mods |= 0x02;
    }
    let shifted = format!("{:?}", KeyCode::from_u16(key as u16 | 0x2200));
    let mut name = short(label(Entry::Code(key as u16)));
    if mods & 0x22 != 0 && shifted != "None" {
      name = short(shifted);
    } else if name.len() == 1 && mods & 0x22 == 0 {
      name = name.to_lowercase();
    } else if mods & 0x22 != 0 && name.len() > 1 {
      name = format!("s-{}", name);
    }
    for (bits, prefix) in [(0x11, "c-"), (0x44, "a-"), (0x88, "g-")] {
      if mods & bits != 0 {
        name = format!("{}{}", prefix, name);
      }
    }
    name
  }

  fn describe(keyboard: &Keyboard, mods: u8) -> String {
    let (layers, base, toggled, sticky, caps_word) = keyboard.engine().state();
    let list = |bits: u32| (0..32).filter(|l| bits & 1 << l != 0).map(|l| l.to_string()).collect::<Vec<_>>().join(" ");
    let top = (0..32).rev().find(|l| layers & 1 << l != 0).unwrap_or(0);
    let mut s = format!("layer {}  (active: {}, base {}", top, list(layers), base);
    if toggled != 0 {
      s += &format!(", toggled {}", list(toggled));
    }
    s += ")";
    if let Some(e) = sticky {
      s += &format!("  sticky {}", short(label(e)));
    }
    if caps_word {
      s += "  CAPS WORD";
    }
    let held: Vec<&str> = (0..8).filter(|b| mods & 1 << b != 0).map(|b| MODIFIERS[b]).collect();
    if !held.is_empty() {
      s += &format!("  mods: {}", held.join("+"));
    }
    s
  }

  fn snapshot(keyboard: &Keyboard, report: [u8; 8], status: String, expect: Option<Vec<String>>) -> Frame {
    let caps_word = keyboard.engine().state().4;
    let view: Vec<(Entry, bool)> = (0..Orbit::KEY_COUNT).map(|k| keyboard.engine().view(k)).collect();
    Frame {
      pressed: (0..Orbit::KEY_COUNT).map(|k| keyboard.is_pressed(k)).collect(),
      labels: view.iter().map(|(e, _)| label(*e)).collect(),
      display: view.iter().map(|(e, r)| shown(*e, *r, report[0], caps_word)).collect(),
      state: describe(keyboard, report[0]),
      report,
      status,
      expect,
    }
  }

  // 34-key split: key k < 30 sits at row k / 10, column k % 10 (5 left, 5 right);
  // the 4 thumb keys sit on row 3 under the inner columns
  fn position(k: usize) -> (usize, usize) {
    if k < 30 {
      (k / 10, k % 10)
    } else {
      (3, k - 27)
    }
  }

  fn short(name: String) -> String {
    let s = match name.as_str() {
      "Semicolon" => ";",
      "Comma" => ",",
      "Dot" => ".",
      "Slash" => "/",
      "Space" => "spc",
      "Enter" => "ent",
      "Backspace" => "bsp",
      "Tab" => "tab",
      "One" => "1",
      "Two" => "2",
      "Three" => "3",
      "Four" => "4",
      "Five" => "5",
      "Six" => "6",
      "Seven" => "7",
      "Eight" => "8",
      "Nine" => "9",
      "Zero" => "0",
      "LeftShift" => "lsft",
      "Escape" => "esc",
      "Delete" => "del",
      "Backslash" => "\\",
      "Right" => "rght",
      "None" => "",
      "Exlm" => "!",
      "At" => "@",
      "Hash" => "#",
      "Dollar" => "$",
      "Percent" => "%",
      "Circumflex" => "^",
      "Ampersand" => "&",
      "Asterisk" => "*",
      "LeftParenthesis" => "(",
      "RightParenthesis" => ")",
      "Underscore" => "_",
      "Plus" => "+",
      "Colon" => ":",
      "QuestionMark" => "?",
      "LeftAngleBracket" => "<",
      "RightAngleBracket" => ">",
      "Pipe" => "|",
      "Tilde" => "~",
      "DoubleQuote" => "\"",
      "Minus" => "-",
      _ => return name,
    };
    s.to_string()
  }

  // top: physical keys; middle: the test; bottom: what each key sends on the current layer.
  // with an expectation, sent keys are green when expected and red when not; expected
  // names that match no key on this layer are listed in red
  fn draw(out: &mut std::io::Stdout, f: &Frame, (w, h): (u16, u16)) {
    let sent = names(&f.report);
    let input: Vec<(String, Option<Color>)> = (0..Orbit::KEY_COUNT)
      .map(|k| (short(format!("{:?}", Orbit::LAYOUT[k][0])), f.pressed[k].then_some(Color::DarkBlue)))
      .collect();
    let output: Vec<(String, Option<Color>)> = f
      .labels
      .iter()
      .zip(&f.display)
      .map(|(name, text)| {
        let is_sent = sent.contains(name);
        let color = match &f.expect {
          None => is_sent.then_some(Color::DarkGreen),
          Some(want) => match (want.contains(name), is_sent) {
            (true, true) => Some(Color::DarkGreen),
            (true, false) | (false, true) => Some(Color::DarkRed),
            _ => None,
          },
        };
        (text.clone(), color)
      })
      .collect();
    let mut status = f.status.clone();
    if let Some(want) = &f.expect {
      let off_board: Vec<&String> = want.iter().chain(sent.iter()).filter(|n| !f.labels.contains(n)).collect();
      if !off_board.is_empty() {
        let list: Vec<String> = off_board.iter().map(|n| format!("{}{}", if sent.contains(n) { "+" } else { "-" }, n)).collect();
        status = format!("{}  [{}]", status, list.join(" "));
      }
    }

    queue!(out, ResetColor, terminal::Clear(terminal::ClearType::All)).unwrap();
    let half = h / 2;
    board(out, "physical keys (host key that presses each position)", &input, 0, half, w);
    let status: String = status.chars().take(w as usize).collect();
    let sx = (w as usize).saturating_sub(status.chars().count()) / 2;
    let color = if status.contains("FAIL") || status.contains("failed") { Color::Red } else { Color::Yellow };
    queue!(out, cursor::MoveTo(sx as u16, half), SetForegroundColor(color), Print(status), ResetColor).unwrap();
    board(out, &format!("firmware output: {}", f.state), &output, half + 1, h - half - 1, w);
    out.flush().unwrap();
  }

  // draws the split board as [label] cells, centered in the rectangle and as big as fits
  fn board(out: &mut std::io::Stdout, title: &str, keys: &[(String, Option<Color>)], top: u16, height: u16, width: u16) {
    const COLS: usize = 11; // 5 + gap + 5
    const ROWS: usize = 4;
    let cw = (width as usize / COLS).clamp(5, 14);
    let rh = ((height as usize).saturating_sub(2) / ROWS).clamp(1, 4);
    let left = (width as usize).saturating_sub(cw * COLS) / 2;
    let y0 = top as usize + ((height as usize).saturating_sub(rh * ROWS + 2)) / 2;

    let tx = (width as usize).saturating_sub(title.len()) / 2;
    queue!(out, cursor::MoveTo(tx as u16, y0 as u16), SetForegroundColor(Color::DarkGrey), Print(title), ResetColor).unwrap();

    for (k, (label, bg)) in keys.iter().enumerate() {
      let (row, col) = position(k);
      let col = if col >= 5 { col + 1 } else { col };
      let inner = cw - 3;
      let label: String = label.chars().take(inner).collect();
      let text = format!("[{:^inner$}]", label, inner = inner);
      let (x, y) = (left + col * cw, y0 + 2 + row * rh);
      queue!(out, cursor::MoveTo(x as u16, y as u16)).unwrap();
      match bg {
        Some(bg) => queue!(out, SetBackgroundColor(*bg), SetForegroundColor(Color::White)).unwrap(),
        None => queue!(out, SetForegroundColor(Color::Grey)).unwrap(),
      }
      queue!(out, Print(text), ResetColor).unwrap();
    }
  }
}

#[cfg(feature = "chip_type_emulator")]
pub use emulator::run;
