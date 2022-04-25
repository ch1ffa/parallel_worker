use crate::worker;

fn test_function(number: u64) -> u64 {
  if number == 0 {
    panic!("Expected positive numbers only!");
  }

  let k = 8;
  let mut result = number;
  let mut i = 0;

  while result != 1 && i < k {
    result = match result % 2 {
      0 => result / 2,
      _ => result * 3 + 1,
    };
    i = i + 1;
  }

  match result {
    1 => i,
    _ => result,
  }
}

pub fn calculate_iterations(list: &[u64]) -> Vec<u64> {
  worker(list, test_function)
}
