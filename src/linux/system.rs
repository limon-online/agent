use std::fs;

use sentry;

use crate::system::{System, SystemInfo};

enum InfoType {
  Name,
  Version
}


fn get_system_info(info: InfoType) -> String {
  let info_str = match info {
    InfoType::Name => "NAME=",
    InfoType::Version => "VERSION_ID=",
  };

  let contents = fs::read_to_string("/etc/os-release");

  let contents = match contents {
    Ok(contents) => contents,
    Err(err) => {
      sentry::capture_error(&err);
      return String::from("Unknown");
    }
  };

  for line in contents.split("\n") {
    if line.starts_with(info_str) {
      return line.replace(info_str, "").replace("\"", "");
    }
  }

  String::from("Unknown")
}


impl SystemInfo for System {
  fn get_name() -> String {
    get_system_info(InfoType::Name)
  }

  fn get_version() -> String {
    get_system_info(InfoType::Version)
  }
}
