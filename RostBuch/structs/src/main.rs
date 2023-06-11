fn main() {
  user_test();
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
