fn main() {
  another_function(5, 'h');
  expression_test();
  fn_test();
  f_test();
}

fn another_function(x: i32, unit_label: char) {
  println!("The value of x is: {x}{unit_label}");
}

fn expression_test() {
  let y = {
    let x = 3;
    x + 1
  };
  println!("The Value of y is: {y}");
}

fn fn_test() {
  let five = five();
  println!("The value of five is: {five}");
  let plus_one = plus_one(5);
  println!("The value of plus_one is: {plus_one}");
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn f_test() {
  println!(
    "{}",
    f({
      let y = 1;
      y + 1
    })
  )
}

fn f(x: i32) -> i32 {
  x + 1
}
