use argh::FromArgs;
use image::{DynamicImage, Rgb, Rgb32FImage};
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::io::{self, Write};
mod shard_noise;
use shard_noise::shard_noise;
mod parse_color;
use parse_color::hex_to_rgb;

#[derive(FromArgs)]
/// Create a nice gradient.
struct GradientSettings {
    /// horizontal resolution, in pixels
    #[argh(option, short = 'w')]
    width: u32,

    /// vertical resolution, in pixels
    #[argh(option, short = 'h')]
    height: u32,

    /// starting color for the gradient (default: #000000)
    #[argh(option, short = 'a', default = "String::from(\"#000000\")")]
    color_a: String,

    /// ending color for the gradient (default: #FFFFFF)
    #[argh(option, short = 'b', default = "String::from(\"#FFFFFF\")")]
    color_b: String,

    /// sharpness of the gradient (default: 33.0)
    #[argh(option, default = "33.0")]
    sharpness: f32,

    /// scaling factor (default: 3.0)
    #[argh(option, default = "3.0")]
    scale: f32,

    /// amount of noise (default: 0.2)
    #[argh(option, default = "0.2")]
    noise: f32,

    /// output filename (default: gradient.png)
    #[argh(option, short = 'o', default = "String::from(\"gradient.png\")")]
    output: String,
}

// This could be replaced with a perceptual color blending function,
// but this is good enough for now.
fn lerp(v: f32, a: f32, b: f32) -> f32 {
    a + v * (b - a)
}

fn main() {
    let gradient: GradientSettings = argh::from_env();

    let min_dimension = min(gradient.width, gradient.height) as f32;

    let color_a = hex_to_rgb(&gradient.color_a);
    let color_b = hex_to_rgb(&gradient.color_b);

    print!("Generating... ");
    io::stdout().flush().unwrap();

    let mut rng = rand::rng();
    let entropy = rng.random::<f32>() * 100.0; // Arbitrary multiplier

    let mut img = Rgb32FImage::new(gradient.width, gradient.height);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            // This seems wasteful, but it turns out that this returns a handle
            // to a pre-initialized ThreadRng. Harmless!
            let mut rng = rand::rng();

            let x = (x as f32) / min_dimension * gradient.scale;
            let y = (y as f32) / min_dimension * gradient.scale;

            let shard_noise = (shard_noise([x, y, entropy], gradient.sharpness)
                + (rng.random::<f32>() * 2.0 - 1.0) * gradient.noise)
                .clamp(0.0, 1.0);

            *pixel = Rgb([
                lerp(shard_noise, color_a.0, color_b.0),
                lerp(shard_noise, color_a.1, color_b.1),
                lerp(shard_noise, color_a.2, color_b.2),
            ]);
        });

    DynamicImage::ImageRgb32F(img)
        .to_rgb8()
        .save(gradient.output)
        .unwrap();

    println!("Done!");
}
