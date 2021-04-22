use image::Rgb;
use imageproc::rect::Rect;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::drawing::Canvas;

pub struct Graphics;

impl Graphics {
    pub fn draw_rects<C: Canvas>(canvas: &mut C,  num: i32) {
        let white = Rgb([255u8, 255u8, 255u8]);
        for a in 0..num {
            draw_filled_rect_mut(canvas, Rect::at(60, 10 + a).of_size(250, 200), white);
        }
    }
}
