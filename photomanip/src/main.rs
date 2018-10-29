/*
@fileName: main.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file is the entrypoint to the photomainpulation program.
*/

extern crate image;

use std::io;
use std::io::Write;
use std::fs::File;

fn handle(_choice: i32) {
    match _choice {
        1 => println!("Do sharpen."),
        2 => println!("Do smooth."),
        3 => println!("Do greyscale."),
        4 => println!("Do negate."),
        5 => println!("Do brighten."),
        6 => println!("Do contrast."),
        7 => println!("Quit."),
        _ => println!("WRONG."),
    }
}

fn menu() {
    println!("1\tSharpen");
    println!("2\tSmooth");
    println!("3\tGreyscale");
    println!("4\tNegate");
    println!("5\tBrighten");
    println!("6\tContrast");
    println!("7\tQuit");
    print!("Choose a function: ");
    let _ = io::stdout().flush();
}

fn main() {
    let mut input = String::new();
    let mut filename = String::new();
    let mut choice: i32 = 0;

    // Prompt for file name
    print!("Enter filename: ");
    let _ = io::stdout().flush();

    // Read in filename
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    // Remove trailing newline
    filename.pop();

    // Replace this with a hookup to the Image class
    // Open file
    //let mut _f = File::open(filename).expect("file not found");

    // Make do while
    while choice != 7 {
        // Print menu
        menu();

        // Read in user input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Remove trailing newline
        input.pop();
        // Convert to int
        choice = input.parse::<i32>().unwrap();
        // Clear input string
        input = "".to_string();

        handle(choice);
    }
}