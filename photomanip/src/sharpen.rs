/*
@fileName: sharpen.rs
@programAuthors: Joe Ceritelli, Kali Regenold
@fileDescription:
 This file contains ...
*/

use image::Image;
use image::Pixel;

impl Image
{
    pub fn sharpen(&mut self) {
        let mut new_pixels : Vec<Pixel> = Vec::new();
        let limit : usize = (self.width * self.height) as usize;

        let width : usize = self.width as usize;
        let height : usize = self.height as usize;

        for i in 0..(width*height) {
            let mut r: u32 = 5 * self.pixels[i].r as u32;
            let mut g: u32 = 5 * self.pixels[i].g as u32;
            let mut b: u32 = 5 * self.pixels[i].b as u32;

            if i / width != 0 {
                r = r.saturating_sub(self.pixels[i-width].r as u32);
                g = g.saturating_sub(self.pixels[i-width].g as u32);
                b = b.saturating_sub(self.pixels[i-width].b as u32);
            }

            if i / width != height-1 {
                r = r.saturating_sub(self.pixels[i+width].r as u32);
                g = g.saturating_sub(self.pixels[i+width].g as u32);
                b = b.saturating_sub(self.pixels[i+width].b as u32);
            }

            if i % width != 0 {
                r = r.saturating_sub(self.pixels[i-1].r as u32);
                g = g.saturating_sub(self.pixels[i-1].g as u32);
                b = b.saturating_sub(self.pixels[i-1].b as u32);
            }

            if i % width != width-1 {
                r = r.saturating_sub(self.pixels[i+1].r as u32);
                g = g.saturating_sub(self.pixels[i+1].g as u32);
                b = b.saturating_sub(self.pixels[i+1].b as u32);
            }

            if r > 255 {r = 255;}
            if g > 255 {g = 255;}
            if b > 255 {b = 255;}

            new_pixels.push(
                Pixel {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                }
            )
        }

        self.pixels = new_pixels;
    }
}
