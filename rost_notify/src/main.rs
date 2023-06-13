use notify_rust::Notification;
use pulldown_cmark::{html, Options, Parser};

fn main() {
  let markdown = std::fs::read_to_string("test.md").expect("Failed to read file");

  let parser = Parser::new_ext(&markdown, Options::all());
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  Notification::new()
    .summary("Markdown Notification")
    .body(&html_output)
    .show()
    .expect("Failed to show Notification");
}
