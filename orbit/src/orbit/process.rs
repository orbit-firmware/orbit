//
// default mode
//
#[cfg(not(feature = "chip_type_emulator"))]
mod stm32 {
  use embassy_usb::driver::Driver;

  use crate::orbit::config as Orbit;
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
  use std::time::Duration;

  use crate::orbit::config as Orbit;
  use crate::orbit::keyboard::Keyboard;
  use crate::orbit::keycodes::KeyCode;

  // input keys per row in the top grid
  const INPUT_COLS: usize = 10;
  const MODIFIERS: [&str; 8] = ["LCtrl", "LShift", "LAlt", "LGui", "RCtrl", "RShift", "RAlt", "RGui"];

  struct Cell {
    label: String,
    on: bool,
  }

  pub async fn run() -> ! {
    let mut out = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(EnterAlternateScreen).unwrap();
    out.execute(cursor::Hide).unwrap();

    let mut keyboard = Keyboard::new();
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
        }
      }

      let report = keyboard.tick();
      let pressed: Vec<bool> = (0..Orbit::KEY_COUNT).map(|k| keyboard.key(k).is_pressed()).collect();
      let size = terminal::size().unwrap_or((80, 24));
      let state = (pressed, report, size);
      if last.as_ref() != Some(&state) {
        draw(&mut out, &state.0, &state.1, size);
        last = Some(state);
      }
      std::thread::sleep(Duration::from_millis(1));
    }
  }

  fn draw(out: &mut std::io::Stdout, pressed: &[bool], report: &[u8; 8], (w, h): (u16, u16)) {
    let inputs: Vec<Cell> = (0..Orbit::KEY_COUNT)
      .map(|k| Cell {
        label: format!("{:?}", Orbit::LAYOUT[k][0]),
        on: pressed[k],
      })
      .collect();

    let mods = MODIFIERS.iter().enumerate().map(|(i, name)| Cell {
      label: name.to_string(),
      on: report[0] & (1 << i) != 0,
    });
    let keys = report[2..].iter().map(|&code| Cell {
      label: if code == 0 { String::new() } else { format!("{:?}", KeyCode::from_u16(code as u16)) },
      on: code != 0,
    });
    let outputs: Vec<Cell> = mods.chain(keys).collect();

    queue!(out, ResetColor, terminal::Clear(terminal::ClearType::All)).unwrap();
    let half = h / 2;
    grid(out, "input: pressed keys", &inputs, INPUT_COLS, 0, half, w);
    grid(out, "output: hid report (modifiers, then 6 key slots)", &outputs, 8, half, h - half, w);
    out.flush().unwrap();
  }

  // draws cells as a grid filling the rectangle (0, top) .. (width, top + height)
  fn grid(out: &mut std::io::Stdout, title: &str, cells: &[Cell], cols: usize, top: u16, height: u16, width: u16) {
    queue!(out, cursor::MoveTo(0, top), SetForegroundColor(Color::DarkGrey), Print(title), ResetColor).unwrap();
    let cols = cols.min(cells.len()).max(1);
    let rows = cells.len().div_ceil(cols).max(1);
    let cw = width as usize / cols;
    let ch = (height.saturating_sub(1) as usize / rows).max(1);

    for (i, cell) in cells.iter().enumerate() {
      let (x, y) = ((i % cols) * cw, top as usize + 1 + (i / cols) * ch);
      let bg = if cell.on { Color::Green } else { Color::DarkGrey };
      let fg = if cell.on { Color::Black } else { Color::White };
      let inner = cw.saturating_sub(1);
      let mut label: String = cell.label.chars().take(inner).collect();
      let pad = inner - label.chars().count();
      label = format!("{}{}{}", " ".repeat(pad / 2), label, " ".repeat(pad - pad / 2));
      for row in 0..ch.saturating_sub(1).max(1) {
        let text = if row == (ch.saturating_sub(1).max(1) - 1) / 2 { label.clone() } else { " ".repeat(inner) };
        queue!(out, cursor::MoveTo(x as u16, (y + row) as u16), SetBackgroundColor(bg), SetForegroundColor(fg), Print(text)).unwrap();
      }
    }
    queue!(out, ResetColor).unwrap();
  }
}

#[cfg(feature = "chip_type_emulator")]
pub use emulator::run;
