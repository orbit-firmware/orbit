#![allow(dead_code)]
pub struct Keyboard<const KEY_COUNT: usize> {
  key_count: usize,
  layout: [[usize; 2]; KEY_COUNT],
  matrix_state: [[bool; 2]; KEY_COUNT],
}

impl<const KEY_COUNT: usize> Keyboard<KEY_COUNT> {
  pub const fn new() -> Self {
    Keyboard {
      key_count: KEY_COUNT,
      layout: [[0; 2]; KEY_COUNT],
      matrix_state: [[false; 2]; KEY_COUNT],
    }
  }

  // Use a matrix layout
  pub fn use_matrix(&mut self, layout: [[usize; 2]; KEY_COUNT]) {
    self.layout = layout;
  }

  pub fn set_matrix_row_state(&self, row: usize, state: bool) -> bool {
    self.matrix_state[row][0] == state
  }

  pub fn set_matrix_col_state(&self, col: usize, state: bool) -> bool {
    self.matrix_state[col][0] == state
  }
}
