/*
@fileName: imageStl.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file is the entrypoint to the photomainpulation program.
*/

use std::io;
use std::io::Write;
use std::fs::File;
use std::error::Error;

mod image;
mod negate;
mod greyscale;
mod brighten;

use image::Image;

fn handle(_choice: &str, img: & mut Image) {
    let mut value : String = String::new();
    match _choice {
        "1" => println!("Do sharpen."),
        "2" => println!("Do smooth."),
        "3" => println!("Do greyscale."),
        "4" => img.negate(),
        "5" => {
                get_input("Value (-255, 255): ", & mut value); 
                let value = value.parse::<i16>().unwrap();
                println!("here!");
                let _ = io::stdout().flush();
                img.brighten(value)
            },
        "6" => println!("Do contrast."),
        "7" => println!("Quit."),
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
}

fn get_input(text: &str, input : & mut String) {
    print!("{}", text);
    let _ = io::stdout().flush();
    io::stdin().read_line(input).expect("Failed to read line");
    input.pop();
}

fn main() {
    let mut input_imp: String;
    let mut in_filename: String = String::new();
    let mut out_filename: String = String::new();
    let mut do_compare: String = String::new();
    let mut in_file: File;
    let mut out_file: File;
    let mut choice: String = String::new();

    get_input("Enter input filename: ", & mut in_filename);
    in_filename = "../../images/puppiesA.ppm".to_string();
    in_file = File::open(in_filename).unwrap();

    get_input("Enter output filename: ", & mut out_filename);
    out_filename = "../../images/out.ppm".to_string();

    out_file = match File::create(& out_filename) {
        Err(why) => panic!("couldn't create output file: {}", why.description()),
        Ok(file) => file,
    };


    get_input("Compare to STL? (y/N) ", & mut do_compare);
    // do something about this or whatever
    menu();

    get_input("Choose a function: ", & mut choice);

    let mut img = image::Image::new(& mut in_file);
    
    handle(&choice, & mut img);

    img.write(& mut out_file);
}