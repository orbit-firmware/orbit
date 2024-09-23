mod orbit;

use futures::executor::block_on;

fn main() {
  let processor = orbit::process::run();
  block_on(processor);
}
