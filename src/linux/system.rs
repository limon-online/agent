use std::fs;

use async_std::task;
use async_trait::async_trait;
use futures::join;
use sentry;

use crate::cpu::CpuInfo;
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


#[async_trait]
impl SystemInfo for System {
  fn get_name() -> String {
    get_system_info(InfoType::Name)
  }

  fn get_version() -> String {
    get_system_info(InfoType::Version)
  }

  fn update_information(&self) {
    task::block_on(self.update_information_future());
  }

  async fn update_information_future(&self) {
    // TODO: Need to pass &mut self info update_cpu()
    // for cpu in &self.cpu_list {
    //   futures::join!(cpu.update_usage());
    // }

    futures::join!(
      self.cpu_list[0].update_usage(),
      self.cpu_list[1].update_usage(),
      self.cpu_list[2].update_usage(),
      self.cpu_list[3].update_usage()
    );
  }
}
