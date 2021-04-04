use std::fs;

use sentry;

use crate::cpu::{Cpu, CpuInfo};


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
    let contents = fs::read_to_string("/proc/cpuinfo");

    let contents = match contents {
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
}
