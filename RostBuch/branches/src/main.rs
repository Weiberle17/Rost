fn main() {
  if_true();
  if_false();
  other_than_zero();
  if_in_let();
  retry_with_loop();
  nested_loops();
  while_loops();
  while_collection();
  for_collection();
  for_loop_countdown();
  question_2();
}

fn if_true() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false")
  }
}

fn if_false() {
  let number = 7;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false")
  }
}

fn other_than_zero() {
  let number = 3;

  if number != 0 {
    println!("number was something other than zero.")
  }
}

fn if_in_let() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");
}

fn retry_with_loop() {
  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  println!("The result is {result}")
}

fn nested_loops() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count")
}

fn while_loops() {
  let mut number = 3;

  while number != 0 {
    println!("{number}");

    number -= 1;
  }
  println!("LIFTOFF!!!");
}

fn while_collection() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("The value is: {}", a[index]);
    index += 1;
  }
}

fn for_collection() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("The value is: {element}");
  }
}

fn for_loop_countdown() {
  for number in (1..4).rev() {
    println!("{number}");
  }
  println!("LIFTOFF!!!");
}

fn question_2() {
  let a = [5; 10];
  let mut sum = 0;
  for x in a {
    sum += x;
  }
  println!("{sum}");
}
