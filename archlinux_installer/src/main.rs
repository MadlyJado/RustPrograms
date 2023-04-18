use std::io;
use std::process::Command;
use std::process::exit;


fn windowsorlinux() -> () {
    let output = if cfg!(target_os = "windows") {
        println!("You are on windows! This program is for installing arch linux only!");
        exit(1);
    } else {
        println!("Welcome to Jadon's Arch Linux Installer!");
    };

    output
}

fn main() {
    windowsorlinux();
}