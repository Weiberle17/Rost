use std::{thread::sleep, time::Duration};

use notify_rust::Notification;

fn main() {
  const TIMER: Duration = Duration::from_secs(600);
  loop {
    Notification::new()
      .summary("Sit up Straight!")
      .hint(notify_rust::Hint::Transient(true))
      .body("You are supposed to sit up straight!")
      .show()
      .expect("Failed to show Notification")
      .on_close(|| println!("Notification closed"));
    
    sleep(TIMER);
  }
}
