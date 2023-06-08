use std::fs;

fn main() {
  let input = fs::read_to_string(std::env::args().nth(1).expect("No file provided")).unwrap();
  let split_input = input.split("\n");
  let mut sum = 0;
  let mut max = 0;
  for i in split_input {
    if i.is_empty() {
      if sum > max {
        max = sum;
      }
      sum = 0;
    } else {
      let i: u32 = i.parse().expect("Not a number");
      sum += i;
    }
  }
  println!("{:?}", max);
  reference_fn(input);
}

// This answer was taken from @timvisee's Github advent-of-code-2022 repo
// in order to understand it.

fn reference_fn(input: String) {
  println!(
    "{}",
    input
      .split("\n\n")
      .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
      .max()
      .unwrap(),
  );
}
