use std::fs;
use std::thread;
use std::time::Duration;

use sentry;

use crate::cpu::{Cpu, CpuInfo};

struct CpuUsage {
  user: u64,
  nice: u64,
  system: u64,
  idle: u64,
  iowait: u64,
  irq: u64,
  softirq: u64,
  steal: u64,
  guest: u64,
  guest_nice: u64,
}


enum CpuInfoType {
  Id,
  PhysicalId,
  CoreId,
  Cores,
  Name,
  Model,
  Vendor,
  Family,
  Mhz,
  Cache,
  Flags,
  Bugs,
  None
}


fn define_cpu_info(line: &str) -> CpuInfoType {
  if line.starts_with("processor") { CpuInfoType::Id }
  else if line.starts_with("physical id") { CpuInfoType::PhysicalId }
  else if line.starts_with("core id") { CpuInfoType::CoreId }
  else if line.starts_with("cpu cores") { CpuInfoType::Cores }
  else if line.starts_with("model name") { CpuInfoType::Name }
  else if line.starts_with("model") { CpuInfoType::Model }
  else if line.starts_with("vendor_id") { CpuInfoType::Vendor }
  else if line.starts_with("cpu family") { CpuInfoType::Family }
  else if line.starts_with("cpu MHz") { CpuInfoType::Mhz }
  else if line.starts_with("cache size") { CpuInfoType::Cache }
  else if line.starts_with("flags") { CpuInfoType::Flags }
  else if line.starts_with("bugs") { CpuInfoType::Bugs }
  else { CpuInfoType::None }
}


fn get_value_from_cpu_info(line: &str) -> String {
  let index = match line.trim().find(": ") {
    Some(index) => index + 2,
    None => {
      return String::from("Unknown");
    }
  };

  String::from(&line[index..])
}


fn parse_cpu(cpu_info: &str) -> Cpu {
  let mut cpu = Cpu::new();

  for line in cpu_info.split("\n") {
    match define_cpu_info(line) {
      CpuInfoType::Id => { cpu.id = get_value_from_cpu_info(line); }
      CpuInfoType::PhysicalId => { cpu.physical_id = get_value_from_cpu_info(line); }
      CpuInfoType::CoreId => { cpu.core_id = get_value_from_cpu_info(line); }
      CpuInfoType::Cores => { cpu.cores = get_value_from_cpu_info(line); }
      CpuInfoType::Name => { cpu.name = get_value_from_cpu_info(line); }
      CpuInfoType::Model => { cpu.model = get_value_from_cpu_info(line); }
      CpuInfoType::Vendor => { cpu.vendor = get_value_from_cpu_info(line) }
      CpuInfoType::Family => { cpu.family = get_value_from_cpu_info(line); }
      CpuInfoType::Mhz => { cpu.mhz = get_value_from_cpu_info(line); }
      CpuInfoType::Cache => { cpu.cache = get_value_from_cpu_info(line); }
      CpuInfoType::Flags => { cpu.flags = get_value_from_cpu_info(line); }
      CpuInfoType::Bugs => { cpu.bugs = get_value_from_cpu_info(line); }
      CpuInfoType::None => {}
    }
  }

  cpu
}


impl CpuInfo for Cpu {
  fn get_cpu_list() -> Vec<Cpu> {
    let mut cpu_list: Vec<Cpu> = Vec::new();

    let contents = match fs::read_to_string("/proc/cpuinfo") {
      Ok(contents) => contents,
      Err(err) => {
        sentry::capture_error(&err);
        return cpu_list;
      }
    };

    for line in contents.split("\n\n") {
      if !line.is_empty() {
        cpu_list.push(parse_cpu(line));
      }
    }

    cpu_list
  }

  fn get_usage(&self) -> u64 {
    // TODO: Need to spawn thread for each get_usage
    let t0 = CpuUsage::new(&self.id);
    thread::sleep(Duration::from_millis(1000));
    let t1 = CpuUsage::new(&self.id);

    ((t1.usage() - t0.usage()) / (t1.total() - t0.total()) * 100 as f32) as u64
  }
}


impl CpuUsage {
  fn new(cpu_id: &String) -> CpuUsage {
    let mut usage = CpuUsage {
      user: 0,
      nice: 0,
      system: 0,
      idle: 0,
      iowait: 0,
      irq: 0,
      softirq: 0,
      steal: 0,
      guest: 0,
      guest_nice: 0
    };

    let contents = match fs::read_to_string("/proc/stat") {
      Ok(contents) => contents,
      Err(err) => {
        sentry::capture_error(&err);
        return usage;
      }
    };

    for line in contents.split("\n") {
      let target_cpu = "cpu".to_owned() + cpu_id;

      if line.starts_with(&target_cpu) {
        let values: Vec<String> = line
          .replace(&target_cpu, "")
          .trim()
          .split(" ")
          .map(|s| s.to_string())
          .collect();

        // TODO: In linux < 2.6 not all rows are allow
        usage.user = values[0].parse().unwrap();
        usage.nice = values[1].parse().unwrap();
        usage.system = values[2].parse().unwrap();
        usage.idle = values[3].parse().unwrap();
        usage.iowait = values[4].parse().unwrap();
        usage.irq = values[5].parse().unwrap();
        usage.softirq = values[6].parse().unwrap();
        usage.steal = values[7].parse().unwrap();
        usage.guest = values[8].parse().unwrap();
        usage.guest_nice = values[9].parse().unwrap();
      }
    }

    usage
  }

  fn total(&self) -> f32 {
    let result = (
      self.user + self.nice + self.system + self.idle +
      self.iowait + self.irq + self.softirq + self.steal
    );

    if result > 0 { result as f32 } else { 1 as f32 }
  }

  fn usage(&self) -> f32 {
    self.total() - (self.idle + self.iowait) as f32
  }
}
