use crate::system::SystemInfo;
use crate::cpu::CpuInfo;

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

use std::time::Duration;
use std::thread;


fn main() {
  let system = system::System::new();

  while (true) {
    for cpu in &system.cpu_list {
      println!("{}", cpu.get_usage());
    }
    thread::sleep(Duration::from_millis(1000));
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
  }
  let json = serde_json::to_string(&system).unwrap();

  println!("{}", json);
}
