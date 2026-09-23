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
  use crate::orbit::peripherals::{REAL_KEYS, SIMULATED};
  use std::sync::atomic::Ordering;

  // (what is tested, host keys held down, keycodes the report must hold)
  // ponytail: written against the qwerty keymap in _emulator.toml, move into the toml when
  // other keyboards need their own tests
  const TESTS: &[(&str, &[&str], &[&str])] = &[
    ("single key", &["Q"], &["Q"]),
    ("right pinky", &["Semicolon"], &["Semicolon"]),
    ("thumb", &["Space"], &["Space"]),
    ("chord on one half", &["Q", "W"], &["Q", "W"]),
    ("chord across halves", &["F", "J"], &["F", "J"]),
    ("both outer thumbs", &["Tab", "Backspace"], &["Tab", "Backspace"]),
    ("7 keys: boot report keeps 6", &["Q", "W", "E", "R", "T", "Y", "U"], &["Q", "W", "E", "R", "T", "Y"]),
  ];
  const SETTLE_MS: u64 = 40; // > debounce_time
  const SHOW_MS: u64 = 700;

  type Show<'a> = &'a mut dyn FnMut(&[bool], &[u8; 8], &str, Option<&[&str]>);

  pub async fn run() -> ! {
    let mut keyboard = Keyboard::new();

    // ORBIT_EMULATOR_TEST=1: run the tests without a screen and exit 1 on failure
    if std::env::var_os("ORBIT_EMULATOR_TEST").is_some() {
      let failed = run_tests(&mut keyboard, &mut |_, _, status, _| println!("{}", status));
      std::process::exit(if failed.is_empty() { 0 } else { 1 });
    }

    let mut out = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(EnterAlternateScreen).unwrap();
    out.execute(cursor::Hide).unwrap();

    let mut summary = play_tests(&mut keyboard);
    let mut last: Option<(Vec<bool>, [u8; 8], (u16, u16))> = None;
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

      let report = keyboard.tick();
      let size = terminal::size().unwrap_or((80, 24));
      let state = (pressed(&mut keyboard), report, size);
      if last.as_ref() != Some(&state) {
        draw(&mut out, &state.0, &state.1, size, &summary, None);
        last = Some(state);
      }
      std::thread::sleep(Duration::from_millis(1));
    }
  }

  // runs the tests on screen and returns the line shown afterwards
  fn play_tests(keyboard: &mut Keyboard) -> String {
    let failed = run_tests(keyboard, &mut |pressed, report, status, expect| {
      let size = terminal::size().unwrap_or((80, 24));
      draw(&mut std::io::stdout(), pressed, report, size, status, expect);
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

  // holds each test's keys, checks the report, releases, checks the report is empty again
  fn run_tests(keyboard: &mut Keyboard, show: Show) -> Vec<&'static str> {
    let mut failed = vec![];
    REAL_KEYS.store(false, Ordering::Relaxed);
    for (i, (name, keys, expect)) in TESTS.iter().enumerate() {
      let head = format!("test {}/{}: {}  press [{}]  expect [{}]", i + 1, TESTS.len(), name, keys.join(" "), expect.join(" "));
      *SIMULATED.lock().unwrap() = keys.iter().map(|k| k.to_string()).collect();
      let report = settle(keyboard, SETTLE_MS);
      let mut sent = sent_names(&report);
      let mut want: Vec<String> = expect.iter().map(|k| k.to_string()).collect();
      sent.sort();
      want.sort();
      let held_ok = sent == want && report[0] == 0;

      SIMULATED.lock().unwrap().clear();
      let released = settle(keyboard, SETTLE_MS);
      let ok = held_ok && released == [0; 8];

      let verdict = match (held_ok, ok) {
        (true, true) => "PASS".to_string(),
        (false, _) => format!("FAIL: sent [{}]", sent.join(" ")),
        (true, false) => format!("FAIL: stuck after release {:02x?}", released),
      };
      if !ok {
        failed.push(*name);
      }

      // show the held state with its verdict
      *SIMULATED.lock().unwrap() = keys.iter().map(|k| k.to_string()).collect();
      let report = settle(keyboard, SETTLE_MS);
      let status = format!("{}  {}", head, verdict);
      show(&pressed(keyboard), &report, &status, Some(expect));
      std::thread::sleep(Duration::from_millis(SHOW_MS));
      SIMULATED.lock().unwrap().clear();
      settle(keyboard, SETTLE_MS);
    }
    REAL_KEYS.store(true, Ordering::Relaxed);
    failed
  }

  fn settle(keyboard: &mut Keyboard, ms: u64) -> [u8; 8] {
    let end = Instant::now() + Duration::from_millis(ms);
    let mut report = keyboard.tick();
    while Instant::now() < end {
      std::thread::sleep(Duration::from_millis(1));
      report = keyboard.tick();
    }
    report
  }

  fn pressed(keyboard: &mut Keyboard) -> Vec<bool> {
    (0..Orbit::KEY_COUNT).map(|k| keyboard.key(k).is_pressed()).collect()
  }

  fn sent_names(report: &[u8; 8]) -> Vec<String> {
    report[2..].iter().filter(|&&c| c != 0).map(|&c| format!("{:?}", KeyCode::from_u16(c as u16))).collect()
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
      "None" => "",
      _ => return name,
    };
    s.to_string()
  }

  // top: physical keys; middle: the test; bottom: firmware output.
  // with an expectation, output keys are green when right and red when wrong
  fn draw(out: &mut std::io::Stdout, pressed: &[bool], report: &[u8; 8], (w, h): (u16, u16), status: &str, expect: Option<&[&str]>) {
    let sent = |code: u16| {
      let key = code as u8;
      let mods = (code >> 8) as u8;
      (key != 0 && report[2..].contains(&key)) || (mods != 0 && report[0] & mods == mods)
    };

    let input: Vec<(String, Option<Color>)> = (0..Orbit::KEY_COUNT)
      .map(|k| (short(format!("{:?}", Orbit::LAYOUT[k][0])), pressed[k].then_some(Color::DarkBlue)))
      .collect();
    let output: Vec<(String, Option<Color>)> = (0..Orbit::KEY_COUNT)
      .map(|k| {
        let name = format!("{:?}", Orbit::KEYMAP[k]);
        let is_sent = sent(Orbit::KEYMAP[k] as u16);
        let color = match expect {
          None => is_sent.then_some(Color::DarkGreen),
          Some(expect) => {
            let wanted = expect.contains(&name.as_str());
            if wanted && is_sent {
              Some(Color::DarkGreen)
            } else if wanted != is_sent {
              Some(Color::DarkRed)
            } else {
              None
            }
          }
        };
        (short(name), color)
      })
      .collect();

    queue!(out, ResetColor, terminal::Clear(terminal::ClearType::All)).unwrap();
    let half = h / 2;
    board(out, "physical keys (host key that presses each position)", &input, 0, half, w);
    let status: String = status.chars().take(w as usize).collect();
    let sx = (w as usize).saturating_sub(status.chars().count()) / 2;
    let color = if status.contains("FAIL") || status.contains("failed") { Color::Red } else { Color::Yellow };
    queue!(out, cursor::MoveTo(sx as u16, half), SetForegroundColor(color), Print(status), ResetColor).unwrap();
    board(out, "firmware output (qwerty keymap)", &output, half + 1, h - half - 1, w);
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
