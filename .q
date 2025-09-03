use std::fs;
use std::process::Command;

fn main() {
    //system paths for todo list
    let path = "/home/raven/Obsidian/vault/Calendar/TODO LIST.md"; 
    
    //reads and writes todo list
    let content = fs::read_to_string(path).expect("failed reading");

    //sends notification
    Command::new("notify-send")
        .arg(&content).output().expect("error notification");



}
