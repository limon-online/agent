use crate::system::SystemInfo;

mod system;

cfg_if::cfg_if! {
  if #[cfg(target_os = "linux")] {
    mod linux;
  }
  else {
    mod unknown;
  }
}


fn main() {
  let system = system::System::new();
  println!("{} {}", system.name, system.version);
}
