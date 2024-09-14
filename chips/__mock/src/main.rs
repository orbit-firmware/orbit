use futures::executor::block_on;
mod orbit;

fn main() {
  let processor = orbit::processor::test();
  block_on(processor);
}
