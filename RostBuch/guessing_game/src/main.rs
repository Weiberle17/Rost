use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  pub struct Guess {
    value: i32,
  }

  impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
      if value < 1 || value > 100 {
        Err(format!("Guess must be between 1 and 100, got {}", value))?;
      }
      Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
      self.value
    }
  }
  println!("This is a guessing game");

  let number_to_guess = rand::thread_rng().gen_range(1..=100);

  loop {
    println!("Guess a number:");

    let mut guessed_number = String::new();

    io::stdin()
      .read_line(&mut guessed_number)
      .expect("User input failed!");

    let guessed_number: Guess = match guessed_number.trim().parse() {
      Ok(num) => {
        match Guess::new(num) {
          Ok(guess) => guess,
          Err(e) => {
            println!("{e}");
            continue;
          }
        }
      },
      Err(_) => continue,
    };

    println!("You guessed: {}", guessed_number.value());

    match guessed_number.value().cmp(&number_to_guess) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
