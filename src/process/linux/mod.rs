use std::vec;
use std::fmt;

use chrono::Utc;

use crate::process::{Process, StateProcess};


impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pid: {}", self.pid)
    }
}


impl fmt::Display for StateProcess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} processes ({})", self.processes.len(), self.timestamp)
    }
}


impl StateProcess {
    pub fn new() -> StateProcess {
        StateProcess {
            processes: vec::Vec::new(),
            timestamp: Utc::now().timestamp()
        }
    }
}
