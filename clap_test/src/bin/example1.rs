use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Optional name to operate on
  pub name: Option<String>,
  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<PathBuf>,
  /// Turn debugging informatin on
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub debug: u8,
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// does testing things
  Test {
    /// list test values
    #[arg(short, long)]
    list: bool,
  },
}

fn main() {
  let cli = Cli::parse();

  if let Some(name) = cli.name.as_deref() {
    println!("Value for name: {name}");
  }
  if let Some(config_path) = cli.config.as_deref() {
    println!("Value for name: {}", config_path.display());
  }

  match cli.debug {
    0 => println!("Debug mode is off"),
    1 => println!("Debug mode is kind of on"),
    2 => println!("Debug mode is on"),
    _ => println!("Don't be crazy"),
  }

  match &cli.command {
    Some(Commands::Test {list}) => {
      if *list {
        println!("Printing testing lists...");
      } else {
        println!("Not printing testing lists...");
      }
    }
    None => {}
  }
}
