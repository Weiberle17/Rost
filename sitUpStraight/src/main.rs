use notify_rust::{CloseReason, Notification, Timeout};
use std::{thread::sleep, time::Duration};

fn main() {
  const TIMER: Duration = Duration::from_secs(600);
  loop {
    Notification::new()
      .summary("Sit up Straight!")
      .hint(notify_rust::Hint::Transient(true))
      .body("You are supposed to sit up straight!")
      .icon("/home/weiberle/.local/bin/sitting.png")
      .timeout(Timeout::Never)
      .show()
      .expect("Failed to show Notification")
      .on_close(|reason: CloseReason| println!("Notification closed: {reason:?}"));

    sleep(TIMER);
  }
}
