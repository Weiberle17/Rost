fn main() {
  let mut v = vec!['h', 'a', 'l', 'l', 'o'];
  ascii_capitalize(&mut v);
  recap_1();
  recap_slice();
  recap_moving_strings();
}

fn ascii_capitalize(v: &mut Vec<char>) {
  let c = &v[0];
  if c.is_ascii_lowercase() {
    v[0] = c.to_ascii_uppercase();
    println!("I capitalized it for you: {:?}", v);
  } else {
    println!("Already capitalized: {:?}", v);
  }
}

fn recap_1() {
  let mut a_num = 0;
  recap_inner(&mut a_num);
  println!("{a_num}");
}

fn recap_inner(x: &mut i32) {
  let another_num = 1;
  let a_stack_ref = &another_num;

  let a_box = Box::new(2);
  let a_box_stack_ref = &a_box;
  let a_box_heap_ref = &*a_box;

  *x += 5;
}

fn recap_slice() {
  let s = String::from("abcdefg");
  let s_slice = &s[2..5];
  println!("{s}, {s_slice}");
}

fn recap_moving_strings() {
  let s = String::from("Hello World");
  let cs = &s;

  println!("{cs}");
  println!("{s}");
}
