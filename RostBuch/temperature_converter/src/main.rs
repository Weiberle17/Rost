use std::io::{self};

fn main() {
  loop {
    let convertion = get_convertion();

    println!("Please enter the value you want converted:");

    let mut input_number = String::new();

    io::stdin()
      .read_line(&mut input_number)
      .expect("User input failed");

    let input_number: i32 = match input_number.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    if convertion == "F\n" {
      println!("{input_number} Fahrenheit equals {} Celsius.", (input_number - 32) * 5 / 9);
    } else {
      println!("{input_number} Celsius equals {} Fahrenheit.", input_number *  9 / 5 + 32);
    }
    break;
  }
}

fn get_convertion() -> String {
  loop {
    println!("Please choose which way you want to convert the temperatures:");
    println!("If you want to convert from Fahrenheit to Celsius please enter F");
    println!("If you want to convert from Celsius to Fahrenheit please enter C");

    let mut convertion = String::new();
    match io::stdin().read_line(&mut convertion) {
      Ok(s) => s,
      Err(_) => continue,
    };

    if convertion == "F\n" || convertion == "C\n" {
      break convertion;
    }
  }
}
