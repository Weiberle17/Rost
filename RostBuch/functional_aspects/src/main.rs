use std::{thread, time::Duration};

fn main() {
  example1();
  example2();
  example3();
  example4();
  example5();
  example6();
}

fn example1() {
  #[derive(Debug, PartialEq, Copy, Clone)]
  enum ShirtColor {
    Red,
    Blue,
  }

  struct Inventory {
    shirts: Vec<ShirtColor>,
  }

  impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
      user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
      let mut num_red = 0;
      let mut num_blue = 0;

      for color in &self.shirts {
        match color {
          ShirtColor::Red => num_red += 1,
          ShirtColor::Blue => num_blue += 1,
        }
      }
      if num_red > num_blue {
        ShirtColor::Red
      } else {
        ShirtColor::Blue
      }
    }
  }

  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref1, giveaway1
  );

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref2, giveaway2
  );
}

fn example2() {
  fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
    };

    if intensity < 25 {
      println!("Today, do {} pushups!", expensive_closure(intensity));
      println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
      if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
      } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
      }
    }
  }

  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn example3() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let only_borrows = || println!("From closure: {:?}", list);

  println!("Before calling closure: {:?}", list);
  only_borrows();
  println!("After calling closure: {:?}", list);
}

fn example4() {
  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7);

  borrows_mutably();
  println!("After calling closure: {:?}", list);
}

fn example5() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
}

fn example6() {
  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }

  let mut list = [
    Rectangle { width: 10, height:1},
    Rectangle { width: 3, height:5},
    Rectangle { width: 7, height:12},
  ];

  list.sort_by_key(|r| r.width);
  println!("{:#?}", list);
}
