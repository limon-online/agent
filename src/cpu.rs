use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub struct Cpu {
  pub id: String,
  pub physical_id: String,
  pub core_id: String,
  pub cores: String,
  pub name: String,
  pub model: String,
  pub vendor: String,
  pub family: String,
  pub mhz: String,
  pub cache: String,
  pub flags: String,
  pub bugs: String
}


impl fmt::Display for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.id)
  }
}


pub trait CpuInfo {
  fn new() -> Cpu {
    Cpu {
      id: String::from("Unknown"),
      physical_id: String::from("Unknown"),
      core_id: String::from("Unknown"),
      cores: String::from("Unknown"),
      name: String::from("Unknown"),
      model: String::from("Unknown"),
      vendor: String::from("Unknown"),
      family: String::from("Unknown"),
      mhz: String::from("Unknown"),
      cache: String::from("Unknown"),
      flags: String::from("Unknown"),
      bugs: String::from("Unknown")
    }
  }

  fn get_cpu_list() -> Vec<Cpu>;
}
