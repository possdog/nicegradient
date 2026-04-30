//! Image generation and rendering logic.
//!
//! This module handles the parallel execution of the selected shader across
//! all pixels of the target image. It iterates over every coordinate, scales
//! it, evaluates the noise function, and blends the resulting value between
//! the two target colors.

use image::{Rgb, Rgb32FImage};
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::min;

use crate::shader::Shader;

/// Strongly-typed configuration needed to render a gradient image.
/// This acts as the boundary between the CLI parsing and the rendering logic.
pub struct RenderConfig {
    /// Width of the output image in pixels
    pub width: u32,
    /// Height of the output image in pixels
    pub height: u32,
    /// The starting color of the gradient, normalized to 0.0 - 1.0 range
    pub color_a: (f32, f32, f32),
    /// The ending color of the gradient, normalized to 0.0 - 1.0 range
    pub color_b: (f32, f32, f32),
    /// Controls how "zoomed in" the coordinate space passed to the shader is
    pub scale: f64,
    /// Amount of random noise to apply post-shader (0.0 means no noise)
    pub noise: f64,
    /// Trait object representing the selected shader implementation
    pub shader: Box<dyn Shader>,
}

/// Linear interpolation between `a` and `b` by factor `v`.
///
/// This could be replaced with a perceptual color blending function
/// (e.g. OkLCH) for more natural looking gradients in the future,
/// but standard RGB lerp is sufficient for a baseline.
fn lerp(v: f32, a: f32, b: f32) -> f32 {
    a + v * (b - a)
}

/// Generates an image buffer based on the provided configuration.
///
/// This leverages Rayon's `par_bridge()` to divide the work of calculating
/// pixel colors across all available CPU threads.
pub fn generate_image(config: &RenderConfig) -> Rgb32FImage {
    // We use min_dimension to maintain aspect ratio scaling when the image isn't square.
    // If we scaled X and Y independently, the noise would appear stretched.
    let min_dimension = min(config.width, config.height) as f64;

    let mut rng = rand::rng();
    // Add a random offset to the Z coordinate to ensure the generated gradient
    // looks different on every run, even if using the exact same arguments.
    let entropy = rng.random::<f64>() * 1000.0; // Arbitrary multiplier to shift the sampling space

    // Initialize an empty 32-bit floating point RGB image
    let mut img = Rgb32FImage::new(config.width, config.height);

    // Iterate over every pixel mutably and process in parallel
    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            // Re-initialize thread-local rng.
            // This looks inefficient but rand::rng() inside a thread is very fast
            // as it returns a handle to a pre-initialized ThreadRng.
            let mut rng = rand::rng();

            // Normalize and scale coordinates based on image dimensions
            let x = (x as f64) / min_dimension * config.scale;
            let y = (y as f64) / min_dimension * config.scale;

            // Evaluate the shader for this specific coordinate point.
            // The shader returns a value roughly between 0.0 and 1.0.
            // We then add the optional uniform noise jitter, and strictly
            // clamp the final result to [0.0, 1.0] so color blending doesn't overflow.
            let noise_val = (config.shader.evaluate([x, y, entropy])
                + (rng.random::<f64>() * 2.0 - 1.0) * config.noise)
                .clamp(0.0, 1.0);

            // Lerp each RGB channel individually based on the evaluated noise value
            *pixel = Rgb([
                lerp(noise_val as f32, config.color_a.0, config.color_b.0),
                lerp(noise_val as f32, config.color_a.1, config.color_b.1),
                lerp(noise_val as f32, config.color_a.2, config.color_b.2),
            ]);
        });

    img
}
