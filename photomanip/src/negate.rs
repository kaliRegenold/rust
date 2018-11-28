/*
@fileName: negate.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use image::Image;
use rayon::prelude::*;

impl Image
{
    pub fn negate(&mut self) {
        self.pixels.par_iter_mut().for_each(|p|{
            p.r = 255 - p.r;
            p.g = 255 - p.g;
            p.b = 255 - p.b;
        });
    }
}
