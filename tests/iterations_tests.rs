use parallel_worker::calculate_iterations;

#[test]
fn test_vector() {
    let input = vec![1u64, 2, 3, 100, 100, 3, 2, 1, 1, 2, 3, 100];
    let output = vec![0u64, 1, 7, 88, 88, 7, 1, 0, 0, 1, 7, 88];

    assert_eq!(calculate_iterations(&input), output);
}

#[test]
fn empty_vector() {
  let input = vec![];
  let output = vec![];

  assert_eq!(calculate_iterations(&input), output);
}

#[test]
#[should_panic]
fn has_zeros() {
  let input = vec![1u64, 0, 2, 3, 100];

  calculate_iterations(&input);
}
