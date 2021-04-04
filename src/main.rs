use crate::system::SystemInfo;

mod cpu;
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
  let json = serde_json::to_string(&system).unwrap();

  println!("{}", json);
}
