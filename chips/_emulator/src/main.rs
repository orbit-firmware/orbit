use futures::executor::block_on;
mod orbit;

fn main() {
  let processor = orbit::orbit::run();
  block_on(processor);
}
