use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  println!("This is a guessing game");

  let number_to_guess = rand::thread_rng().gen_range(1..=100);

  loop {
    println!("Guess a number:");

    let mut guessed_number = String::new();

    io::stdin()
      .read_line(&mut guessed_number)
      .expect("User input failed!");

    let guessed_number: u32 = match guessed_number.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {guessed_number}");

    match guessed_number.cmp(&number_to_guess) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
