use std::process::Command;

fn main() {
    // The name of the process you want to kill
    let process_name = "chrome.exe";

    // Terminate the process by name
    Command::new("taskkill")
        .args(&["/IM", process_name, "/F"])
        .status()
        .expect("Failed to execute taskkill");

    println!("Process terminated");
}
