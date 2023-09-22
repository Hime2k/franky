// write a function to check brew existence in Mac OSX and install it if it doesn't exist. then install packages from apps.json

use std::process::Command;

async fn check_brew() {
    let brew = Command::new("brew")
        .output()
        .expect("failed to execute process");
    if brew.status.success() {
        println!("brew is installed");
    } else {
        println!("brew is not installed");
    }
}


pub async fn setup_osx() {
    println!("Setting up Mac OSX");
    let uname = std::process::Command::new("uname")
        .arg("-m")
        .output()
        .expect("failed to execute process");
    println!("architecture: {}", String::from_utf8_lossy(&uname.stdout));
check_brew().await;


}