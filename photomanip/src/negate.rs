/*
@fileName: negate.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use image::Image;

impl Image
{
    pub fn negate(&mut self) {
        for p in &mut self.pixels {
            p.r = 255 - p.r;
            p.g = 255 - p.g;
            p.b = 255 - p.b;
        }
    }
}
