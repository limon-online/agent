pub struct System {
  pub name: String,
  pub version: String
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
