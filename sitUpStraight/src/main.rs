use notify_rust::{Notification, Timeout};

fn main() {
  Notification::new()
    .summary("Sit up Straight!")
    .body("You are supposed to sit up straight!")
    .icon("sitting.png")
    .timeout(Timeout::Never)
    .show()
    .expect("Failed to show Notification");
}
