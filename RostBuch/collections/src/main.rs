fn main() {
  vec1();
  vec2();
  vec3();
  vec4();
  strings1();
  strings2();
  strings3();
}

fn vec1() {
  let v: Vec<i32> = Vec::new();
  let b = vec![1, 2, 3];

  dbg!(v, b);
}

fn vec2() {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  dbg!(v);
}

fn vec3() {
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {third}");

  let third: Option<&i32> = v.get(2);
  match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element"),
  }

  dbg!(third);
}

fn vec4() {
  #[derive(Debug)]
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  dbg!(row);
}

fn strings1() {
  let data = "initial contents";
  let s = data.to_string();

  dbg!(data, s);

  let s = "initial contents".to_string();

  dbg!(data, s);
  
  let s = String::from("initial contents");

  dbg!(data, s);
}

fn strings2() {
  let mut s = String::from("foo");
  s.push_str("bar");

  dbg!(s);

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);

  dbg!(s1, s2);
}

fn strings3() {
  let mut s = String::from("tes");
  s.push('t');

  dbg!(s);

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;

  dbg!(s2, s3);
}
