use std::path::Path;
use rand::Rng;
use image::{ DynamicImage, RgbImage };

pub struct Glitch;

fn clamp(val: i16, min: i16, max: i16) -> i16 {
    if val <= min { return min; }
    if val >= max { return max; }
    return val;
}

impl Glitch {
    pub fn glitching_file(filename: String, x_factor: u16, y_factor: u16) {
        let mut buf = image::open(&Path::new(&filename)).unwrap().to_rgb8();
        println!("dimensions {:?}", buf.dimensions());
    
        let (w, h) = buf.dimensions();
        let mut xoff: i16 = 0;
        let mut yoff: i16 = 0;
        let mut rng = rand::thread_rng();
    
        for y in 0..h {
            for x in 0..w {
                if rand::random::<u16>() < x_factor {
                    xoff += rng.gen_range(-1..2);
                }
                if rand::random::<u16>() < y_factor {
                    yoff += rng.gen_range(-1..2);
                }
                if rand::random::<u16>() < 10 {
                    xoff /= 2;
                    yoff /= 2;
                }
            
                let srcx = clamp((x as i16) + xoff, 0, (w - 1) as i16);
                let srcy = clamp((y as i16) + yoff, 0, (h - 1) as i16);
                let src_pixel = buf[(srcx as u32, srcy as u32)];
                buf.put_pixel(x, y, src_pixel);
            }
        }
        let out_filename = format!("{}.rg.png", filename);
        let _ = DynamicImage::ImageRgb8(buf).save(&out_filename);
        println!("Saved => {0}", out_filename);
    }
    pub fn glitching_buf(buf: &mut RgbImage, x_factor: u16, y_factor: u16) -> &mut RgbImage {
        let (w, h) = buf.dimensions();
        let mut xoff: i16 = 0;
        let mut yoff: i16 = 0;
        let mut rng = rand::thread_rng();
    
        for y in 0..h {
            for x in 0..w {
                if rand::random::<u16>() < x_factor {
                    xoff += rng.gen_range(-1..2);
                }
                if rand::random::<u16>() < y_factor {
                    yoff += rng.gen_range(-1..2);
                }
                if rand::random::<u16>() < 10 {
                    xoff /= 2;
                    yoff /= 2;
                }
            
                let srcx = clamp((x as i16) + xoff, 0, (w - 1) as i16);
                let srcy = clamp((y as i16) + yoff, 0, (h - 1) as i16);
                let src_pixel = buf[(srcx as u32, srcy as u32)];
                buf.put_pixel(x, y, src_pixel);
            }
        }
        buf
    }
}