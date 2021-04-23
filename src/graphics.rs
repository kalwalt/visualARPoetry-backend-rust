use image::{ Rgb, RgbImage };
use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;

pub struct Graphics;

impl Graphics {
    pub fn draw_rects(canvas: &mut RgbImage, pos_x: i32, pos_y: i32, width: u32, height: u32, num: i32, color: Rgb<u8>) {
        for a in 0..num {
            draw_hollow_rect_mut(canvas, Rect::at(pos_x, pos_y + (a * 4)).of_size(width, height), color);
        }
    }
}
