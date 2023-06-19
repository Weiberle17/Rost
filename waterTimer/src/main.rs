use notify_rust::{CloseReason, Notification, Timeout};
use std::{env, thread::sleep, time::Duration};

fn main() {
  let timer: Duration = Duration::from_secs(
    env::args()
      .nth(1)
      .unwrap_or("9000".to_string())
      .parse()
      .expect("Please input a valid Integer"),
  );
  
  println!("Der Timer wurde auf {timer:?} gesetzt.");

  loop {
    Notification::new()
      .summary("Time to drink some water!")
      .hint(notify_rust::Hint::Transient(true))
      .body("You are supposed to drink some Water!")
      .icon("/home/weiberle/.local/bin/waterIcon.png")
      .timeout(Timeout::Never)
      .show()
      .expect("Failed to show Notification")
      .on_close(|reason: CloseReason| println!("WaterTimer notification closed: {reason:?}"));

    sleep(timer);
  }
}
