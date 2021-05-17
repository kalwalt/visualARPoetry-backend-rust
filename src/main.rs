use std::env;
use image::Rgb;
use imageproc::drawing::draw_filled_circle_mut;

mod graphics;
use crate::graphics::Graphics;

mod utils;
use crate::utils::Utils;

mod deserialize_json;
use crate::deserialize_json::Deserializer;

mod glitch;
use crate::glitch::Glitch;

fn main() {
    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "imgs/fishes.jpg".to_owned()
        }
    };
    let mut img = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgb8();
    let red   = Rgb([255u8, 0u8, 0u8]);
    let white = Rgb([255u8, 255u8, 255u8]);
    println!("Date is: {:}", Utils::get_date());
    Deserializer::deserialize_poem_test();
    Deserializer::deserialize_poem("./poems/poems.json".to_string());
    draw_filled_circle_mut(&mut img, (150, 150), 150, red);
    println!("Circle drawn!");
    Graphics::draw_rects(&mut img, 60, 10, 250, 200, 40, white);
    println!("Rects drawn!");
    Graphics::recursive_lines_x(&mut img, 20.0, 320, 20, 10, 200, red);
    Graphics::recursive_lines_y(&mut img, 20.0, 320, 20, 10, 200, red);
    Utils::get_random_int_inclusive(0, 100);
    img.save("imgs/visual_poetry.jpg").ok();
    println!("Image visual_poetry.jpg saved");
    Glitch::glitching_file("imgs/fishes.jpg".to_string(), 100, 500);
    Glitch::glitching_buf(&mut img, 20, 200).save("imgs/visual_poetry_glitch.jpg").ok();
    println!("Image visual_poetry_glitch.jpg saved");
}
