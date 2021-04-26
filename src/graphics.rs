use image::{ Rgb, RgbImage };
use imageproc::rect::Rect;
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};

pub struct Graphics;

impl Graphics {
    pub fn draw_rects(canvas: &mut RgbImage, pos_x: i32, pos_y: i32, width: u32, height: u32, num: i32, color: Rgb<u8>) {
        for a in 0..num {
            draw_hollow_rect_mut(canvas, Rect::at(pos_x, pos_y + (a * 4)).of_size(width, height), color);
        }
    }

    pub fn recursive_lines_x(canvas: &mut RgbImage, start: f32, line_width: u32, start_x: u32, spacing: u32, number_lines: u32, color: Rgb<u8>) {
        let factor = number_lines * spacing;
        for x in (start_x..factor).step_by(spacing as usize) {
            draw_line_segment_mut(canvas, (x as f32, start as f32), (x as f32, start as f32 + line_width as f32), color);
        };
    }

    pub fn recursive_lines_y(canvas: &mut RgbImage, start: f32, line_width: u32, start_y: u32, spacing: u32, number_lines: u32, color: Rgb<u8>) {
        let factor = number_lines * spacing;
        for y in (start_y..factor).step_by(spacing as usize) {
            draw_line_segment_mut(canvas, (start as f32, y as f32), (start as f32 + line_width as f32, y as f32), color);
        };
    }
}
