//! Main entry point for the nicegradient application.
//!
//! This module ties together the CLI parsing (`cli.rs`) and the
//! image rendering logic (`render.rs`). It acts as a lightweight
//! wrapper that simply passes configuration from one to the other.

use image::DynamicImage;
use std::io::{self, Write};

mod cli;
use cli::GradientSettings;

mod parse_color;
mod render;
mod shader;
mod shard_noise;

fn main() {
    // Parse command line arguments from the environment using `argh`.
    let settings: GradientSettings = argh::from_env();

    // Convert the CLI-specific struct into a rendering-specific configuration.
    // This extracts the parsed logic (like converting string colors to RGB)
    // and returns the target output path.
    let (render_config, output_path) = settings.into_render_config();

    print!("Generating... ");
    io::stdout().flush().unwrap();

    // Execute the rendering process using the parsed configuration.
    let img = render::generate_image(&render_config);

    // Convert the generated 32-bit float RGB image to a standard 8-bit RGB image
    // and save it to the specified output path.
    DynamicImage::ImageRgb32F(img)
        .to_rgb8()
        .save(output_path)
        .unwrap();

    println!("Done!");
}
