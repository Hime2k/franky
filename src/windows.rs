

use std::process::Command;


use std::fs::File;


async fn check_winget() {
    let winget = Command::new("winget")
        .output()
        .expect("failed to execute process");
    if winget.status.success() {
        println!("winget is installed");
    } else {
        println!("winget is not installed");
    }
}

async fn install_packages() {
    let apps = File::open("apps.json").expect("file not found");
    let apps: serde_json::Value = serde_json::from_reader(apps).expect("error while reading file");
    let apps = apps["windows"].as_object().unwrap();
    for (key, value) in apps {
        println!("Installing {}...", key);
        let mut command = Command::new("winget");
        command.arg("install");
        command.arg(value.as_str().unwrap());
        command.arg("-e");
        command.arg("-q");
        command.output().expect("failed to execute process");
    }

}

pub async fn setup_windows() {
    check_winget().await;
    install_packages().await;
}


