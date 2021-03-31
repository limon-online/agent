use std::fs;
use std::fs::File;
use std::io::BufReader;

use crate::system::{System, SystemInfo};

impl SystemInfo for System {
  fn get_name() -> String {
    String::from("Ubuntu")
  }

  fn get_version() -> String {
    String::from("20.02")
  }
}
