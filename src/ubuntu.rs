// install packages from apps.json using apt in ubuntu

pub async fn setup_ubuntu() {
    println!("Setting up Ubuntu");

    let uname = std::process::Command::new("uname")
        .arg("-m")
        .output()
        .expect("failed to execute process");
    println!("architecture: {}", String::from_utf8_lossy(&uname.stdout));

    // print lsb_release -a

    let lsb_release = std::process::Command::new("lsb_release")
        .arg("-a")
        .output()
        .expect("failed to execute process");
    
    println!("lsb_release -a: {}", String::from_utf8_lossy(&lsb_release.stdout));


}
