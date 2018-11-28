/*
@fileName: brighten.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use image::Image;
use rayon::prelude::*;

impl Image
{
    pub fn brighten(&mut self, value : i16) {
        if value > 0
        {
            self.pixels.par_iter_mut().for_each(|p|{
                p.r = p.r.saturating_add(value as u8);
                p.g = p.g.saturating_add(value as u8);
                p.b = p.b.saturating_add(value as u8);
            });
        }
        else if value < 0 {
            self.pixels.par_iter_mut().for_each(|p|{
                p.r = p.r.saturating_sub(-value as u8);
                p.g = p.g.saturating_sub(-value as u8);
                p.b = p.b.saturating_sub(-value as u8);
            });
        }
    }
}