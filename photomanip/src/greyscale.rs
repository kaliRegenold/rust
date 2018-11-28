/*
@fileName: greyscale.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use image::Image;
use rayon::prelude::*;

// We need to handle grey pixels (like writing them out and what not...)
impl Image
{
    pub fn greyscale(&mut self) {
        self.pixels.par_iter_mut().for_each(|p|{
            p.r = 255 - p.r;
            p.g = 255 - p.g;
            p.b = 255 - p.b;
            let grey = 0.3*(p.r as f64) + 0.6*(p.g as f64) + 0.1*(p.b as f64);
        });
    }
}