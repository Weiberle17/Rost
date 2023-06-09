fn main() {
  let mut v = vec!['h', 'a', 'l', 'l', 'o'];
  ascii_capitalize(&mut v);
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
