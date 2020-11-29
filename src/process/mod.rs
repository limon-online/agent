use std::vec;


#[cfg(target_os = "linux")]
pub mod linux;


#[derive(Debug)]
pub struct Process {
    pid: u32
}


#[derive(Debug)]
pub struct StateProcess {
    pub processes: vec::Vec<Process>,
    pub timestamp: i64
}
