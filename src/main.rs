use std::io;
use std::fs::File;
use std::io::prelude::*;



fn main() {
    // How to Read a file
    let mut read = File::open("read.txt").expect("Can;t open the file");

    let mut contents = String::new();
    read.read_to_string(&mut contents)
        .expect("Oops! Can not read the file...");
    
    println!("File Contents: \n\n{}", contents);

    // How to write into a file
    let mut write = File::create("output.txt")
        .expect("Could not create file!");
    // user input in Rust. Python example (user = input("What is your name? "))
    println!("Please write down your todays journal log:\n");
    let mut journal = String::new();
    io::stdin().read_line(&mut journal)
        .expect("Faild to read input"); 

    write.write_all(journal.as_bytes())
        .expect("Cannot write to the file, sorry mate.");
}

