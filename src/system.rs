use std::fmt;

use async_trait::async_trait;
use serde::Serialize;

use crate::cpu::Cpu;
use crate::cpu::CpuInfo;

#[derive(Serialize)]
pub struct System {
  pub name: String,
  pub version: String,
  pub cpu_list: Vec<Cpu>
}


impl fmt::Display for System {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {}", self.name, self.version)
  }
}


#[async_trait]
pub trait SystemInfo {
  fn new() -> System {
    System {
      name: System::get_name(),
      version: System::get_version(),
      cpu_list: Cpu::get_cpu_list()
    }
  }

  fn get_name() -> String;

  fn get_version() -> String;

  fn update_information(&self);

  async fn update_information_future(&self);
}
