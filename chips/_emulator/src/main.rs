use futures::executor::block_on;
mod orbit;

fn main() {
  let processor = orbit::processor::emulate();
  block_on(processor);
}
