use std::{thread, time::{Duration, Instant}};
use parallel_worker::worker;

fn heavy_func(number: i32) -> i32 {
  thread::sleep(Duration::from_millis(100));
  number
}

#[test]
fn is_parallel() {
  let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  let batch_size = (input.len() / num_cpus::get()).max(1);
  let now = Instant::now();
  worker(&input, heavy_func);
  assert!(now.elapsed().as_millis() < ((batch_size + 1) * 100).try_into().unwrap());
}

#[test]
fn is_sequential() {
  let input = vec![1, 2, 3];
  let now = Instant::now();
  worker(&input, heavy_func);
  assert!(now.elapsed().as_millis() >= 3 * 100);
}
