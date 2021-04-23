use image::{ Rgb, RgbImage };
use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;

pub struct Graphics;

impl Graphics {
    pub fn draw_rects(canvas: &mut RgbImage,  num: i32, color: Rgb<u8>) {
        let white = Rgb([255u8, 255u8, 255u8]);
        for a in 0..num {
            draw_hollow_rect_mut(canvas, Rect::at(60, 10 + (a * 4)).of_size(250, 200), color);
        }
    }
}
