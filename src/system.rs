use std::fmt;

pub struct System {
  pub name: String,
  pub version: String
}


impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}


pub trait SystemInfo {
  fn new() -> System {
    System {
      name: System::get_name(),
      version: System::get_version()
    }
  }

  fn get_name() -> String;

  fn get_version() -> String;
}
