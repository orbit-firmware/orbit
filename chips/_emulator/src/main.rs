use futures::executor::block_on;
mod orbit;

fn main() {
  let processor = orbit::process::run();
  block_on(processor);
}
