use crate::config;

pub use embedded_hal;
use embedded_hal::digital::{InputPin as DIPin, OutputPin as DOPin};

pub struct Pinout {
  matrix_rows: [u8; config::MATRIX_ROW_COUNT],
  matrix_cols: [u8; config::MATRIX_COL_COUNT],
  multiplexer_sel: [u8; config::MULTIPLEXER_SEL_COUNT],
  multiplexer_com: [u8; config::MULTIPLEXER_COM_COUNT],
}

impl Pinout {
  pub fn new() -> Self {
    let mut matrix_rows = [0; config::MATRIX_ROW_COUNT];
    let mut matrix_cols = [0; config::MATRIX_COL_COUNT];
    let mut multiplexer_sel = [0; config::MULTIPLEXER_SEL_COUNT];
    let mut multiplexer_com = [0; config::MULTIPLEXER_COM_COUNT];

    Self {
      matrix_rows,
      matrix_cols,
      multiplexer_sel,
      multiplexer_com,
    }
  }
}
