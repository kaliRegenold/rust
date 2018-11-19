/*
@fileName: image.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use std::fs::File;
use std::io::Read;
use std::io::Write;



pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    max_color_value: u8,
    pixels: Vec<Pixel>,
}

impl Image {
    
    pub fn new(file: &mut File) -> Image {

        // to hold contents of the file in string form
        let mut contents = String::new();

        // put contents of file into contents string
        file.read_to_string(&mut contents).expect("could not read file");

        // vector to hold contents of the file, split on newlines
        let contents : Vec<&str> = contents.split("\n").collect();

        // counter to the beginning of the color values
        let mut counter = 0;

        // MAGIC NUMBER is at contents[0] so skip it
        counter += 1;

        // COMMENTS, ignored
        while contents[counter].as_bytes()[0] == 35 {      // 35 = # in ascii
            counter += 1;
        }

        // WIDTH and HEIGHT of image
        let w_h : Vec<&str> = contents[counter].split(" ").collect();
        let width = w_h[0].parse::<u32>().unwrap();
        let height = w_h[1].parse::<u32>().unwrap();
        counter += 1;

        // MAX COLOR VALUE
        let max_color_value = contents[counter].parse::<u8>().unwrap();
        counter += 1;

        // number of lines of color values
        let limit : usize = (width * height * 3) as usize;

        // vector to hold pixels as we receive them
        let mut pixels : Vec<Pixel> = Vec::new();

        // COLOR VALUES into pixel vector
        for i in (0..limit).step_by(3) {
            pixels.push(
                Pixel {
                    r: contents[counter + i + 0].parse::<u8>().unwrap(),
                    g: contents[counter + i + 1].parse::<u8>().unwrap(),
                    b: contents[counter + i + 2].parse::<u8>().unwrap(),
                }
            )
        }

        // make and return the image
        Image {
            width: width,
            height: height,
            max_color_value: max_color_value,
            pixels: pixels,
        }
    }

    pub fn write(&self, file: & mut File) {
        file.write_all(b"P3\n");
        let w_h_line = format!("{} {}\n", self.width, self.height);
        file.write_all(w_h_line.as_bytes());
        file.write_all((self.max_color_value.to_string() + "\n").as_bytes());
        for p in &self.pixels {
            file.write_all((p.r.to_string() + "\n").as_bytes());
            file.write_all((p.g.to_string() + "\n").as_bytes());
            file.write_all((p.b.to_string() + "\n").as_bytes());
        }
    }

}
