mod process;

use process::StateProcess;


fn main() {
    let state_process = StateProcess::new();
    println!("{}", state_process);
}
