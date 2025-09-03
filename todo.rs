use std::fs;
use std::process::Command;

fn main() {
    //system paths for todo list
    let path = "/home/raven/Obsidian/vault/Calendar/TODO LIST.md"; 
    
    //reads and writes todo list
    let content = fs::read_to_string(path).expect("failed reading");

    //sends notification
    if content==""{
        let _ = Command::new("notify-send")
            .arg("TODO LIST")
            .arg("Nothing to do :)")
            .output();
    }
    else {
        let _ = Command::new("notify-send")
            .arg("TODO LIST")
            .arg(&content)
            .output();
    }
    
}
