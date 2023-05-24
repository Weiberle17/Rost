use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
  user_id: i32,
  user_name: String,
  sur_name: String,
  first_name: String,
  age: i32,
  password: String,
  reg_date: String,
  email: String,
}

fn main() {
  let url = "mysql://api1:password1@localhost/chatroom";
  let pool = Pool::new(url).unwrap();
  let mut conn = pool.get_conn().unwrap();

  let res = conn
    .query_map(
      "select * from users;",
      |(user_id, user_name, sur_name, first_name, age, password, reg_date, email)| User {
        user_id: user_id,
        user_name: user_name,
        sur_name: sur_name,
        first_name: first_name,
        age: age,
        password: password,
        reg_date: reg_date,
        email: email,
      },
    )
    .expect("Query failed.");

  for r in res {
    println!("{:?}", r);
  }
}
