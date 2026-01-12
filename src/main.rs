use argh::FromArgs;
use image::{DynamicImage, Rgb, Rgb32FImage};
use rand::prelude::*;
use std::cmp::min;
use std::io::{self, Write};
mod shard_noise;
use shard_noise::shard_noise;
mod parse_color;
use parse_color::hex_to_rgb;

const SHARPNESS: f32 = 33.0;
const SCALE: f32 = 3.0;
const NOISE: f32 = 0.2;

#[derive(FromArgs)]
/// Create a nice gradient.
struct GradientSettings {
    /// horizontal resolution, in pixels
    #[argh(option, short = 'w')]
    width: u32,

    /// vertical resolution, in pixels
    #[argh(option, short = 'h')]
    height: u32,

    /// starting color for the gradient (default: #7A76DA)
    #[argh(option, short = 'a', default = "String::from(\"#7A76DA\")")]
    color_a: String,

    /// ending color for the gradient (default: #A8C3E9)
    #[argh(option, short = 'b', default = "String::from(\"A8C3E9\")")]
    color_b: String,

    /// output filename (default: gradient.png)
    #[argh(option, short = 'o', default = "String::from(\"gradient.png\")")]
    output: String
}

fn lerp(v: f32, a: f32, b: f32) -> f32 {
    a + v * (b - a)
}

fn main() {
    let gradient: GradientSettings = argh::from_env();

    let color_a = hex_to_rgb(&gradient.color_a);
    let color_b = hex_to_rgb(&gradient.color_b);

    print!("Generating... ");
    io::stdout().flush().unwrap();

    let mut rng = rand::rng();

    let entropy = rng.random::<f32>() * 100.0;

    let mut img = Rgb32FImage::new(gradient.width, gradient.height);

    let min_dimension = min(gradient.width, gradient.height) as f32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x = (x as f32) / min_dimension * SCALE;
        let y = (y as f32) / min_dimension * SCALE;

        let shard_noise = (shard_noise([x, y, entropy], SHARPNESS)
            + (rng.random::<f32>() * 2.0 - 1.0) * NOISE)
            .clamp(0.0, 1.0);

        *pixel = Rgb([
            lerp(shard_noise, color_a.0, color_b.0),
            lerp(shard_noise, color_a.1, color_b.1),
            lerp(shard_noise, color_a.2, color_b.2),
        ]);
    }

    DynamicImage::ImageRgb32F(img)
        .to_rgb8()
        .save(gradient.output)
        .unwrap();

    println!("Done!");
}
