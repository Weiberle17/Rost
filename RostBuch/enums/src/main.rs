fn main() {
  enums1();
  enums2();
  enums3();
  enums4();
  option1();
  match1();
  match2();
  match3();
  if_let1();
  if_let2();
}

fn enums1() {
  #[derive(Debug)]
  enum IpAddrKind {
    V4,
    V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  dbg!(four);
  dbg!(six);
}

fn enums2() {
  #[derive(Debug)]
  enum IpAddrKind {
    V4,
    V6,
  }

  #[derive(Debug)]
  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  dbg!(home);
  dbg!(loopback);
}

fn enums3() {
  #[derive(Debug)]
  enum IpAddr {
    V4(String),
    V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));

  let loopback = IpAddr::V6(String::from("::1"));

  dbg!(home);
  dbg!(loopback);
}

fn enums4() {
  #[derive(Debug)]
  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  let home = IpAddr::V4(127, 0, 0, 1);

  let loopback = IpAddr::V6(String::from("::1"));

  dbg!(home);
  dbg!(loopback);
}

fn option1() {
  let some_number = Some(5);
  let some_char = Some('e');

  let absent_number: Option<i32> = None;

  dbg!(some_number, some_char, absent_number);
}

fn match1() {
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  impl Coin {
    fn value_in_cents(coin: Coin) -> u8 {
      match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
      }
    }
  }

  let test_coin = Coin::Dime;

  dbg!(Coin::value_in_cents(test_coin));
}

fn match2() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  dbg!(five, six, none);
}

fn match3() {
  fn plus_one(x: Option<i32>) -> i32 {
    match x {
      None => 0,
      Some(i) => i + 1,
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  dbg!(five, six, none);
}

fn if_let1() {
  let config_max = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
  }
}

fn if_let2() {
  let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }
}
