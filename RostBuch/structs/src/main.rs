fn main() {
  user_test();
  default_example();
  tuple_example();
  struct_example();
  printing_structs();
  debug_structs();
  impl_test();
  rect_hold();
  square_test();
}

#[derive(Debug)]
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn user_test() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  let user2 = User {
    email: String::from("testmail"),
    username: String::from("testusername2"),
    active: false,
    sign_in_count: 2,
  };

  println!("{user1:#?}");
  println!("{user2:#?}");
}

fn default_example() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area_default(width1, height1)
  );
}

fn area_default(width: u32, height: u32) -> u32 {
  width * height
}

fn tuple_example() {
  let rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area_tuple(rect1)
  );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn struct_example() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area_struct(&rect1)
  );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

fn printing_structs() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!("rect1 is {:#?}", rect1);
}

fn debug_structs() {
  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };
  dbg!(&rect1);
}

fn impl_test() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
}

fn rect_hold() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn square_test() {
  let rect1 = Rectangle::square(30);
  println!("{rect1:#?}");
}
