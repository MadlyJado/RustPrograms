use std::process::Command;
use std::process::exit;
use std::io;


fn windowsorlinux() {
    let output = if cfg!(target_os = "windows") {
        println!("You are on windows! This program is for installing arch linux only!");
        exit(0);
    } else {
        println!("Welcome to Jadon's Arch Linux Installer!");
    };
}

fn chooseadisk() {
    let fdisk = Command::new("fdisk")
    .arg("-l")
    .output()
    .expect("failed to execute fdisk command!");

    println!("Which Disk would you like to choose\n {}", String::from_utf8_lossy(&fdisk.stdout));

    println!("Type in the path to the disk you would like to use (ex: /dev/sda): ");
    let mut diskpath = String::new();

    io::stdin().read_line(&mut diskpath).expect("failed to read dispath input!");
}

fn main() {
    windowsorlinux();
    chooseadisk();
}