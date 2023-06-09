use std::io;

fn main() {
  println!("Please input the Fibonacci number you want calculated:");
  let mut nth_fibonacci = String::new();
  io::stdin().read_line(&mut nth_fibonacci).expect("User input failed");
  let nth_fibonacci: u32 = nth_fibonacci.trim().parse().expect("Please input a number");
  println!("The {nth_fibonacci}th Fibonacci number is: {}", fib(nth_fibonacci));
}

fn fib(n: u32) -> u32 {
  if n <= 2 {
    1
  } else {
    fib(n - 1) + fib(n - 2)
  }
}
