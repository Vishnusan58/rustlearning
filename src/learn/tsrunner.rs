
use  std::process::Command; 
pub fn run() {
    let output = Command::new("node").arg("src/learn/loops.js").output().expect("Fail to execute command");
    println!("The Output is {:?}", String::from_utf8_lossy(&output.stdout));
    
}