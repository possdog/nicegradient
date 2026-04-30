//! Command-line interface definition and parsing.
//!
//! This module uses `argh` to define the expected arguments and subcommands
//! for the CLI. It also provides the logic to convert these raw CLI inputs
//! into a clean `RenderConfig` that the rendering engine can use without
//! needing to know about CLI specific traits or strings.

use argh::FromArgs;

use crate::parse_color::hex_to_rgb;
use crate::render::RenderConfig;
use crate::shader::Shader;
use crate::shard_noise::ShardNoise;

/// Create a nice gradient.
#[derive(FromArgs)]
pub struct GradientSettings {
    /// horizontal resolution, in pixels
    #[argh(option, short = 'w')]
    pub width: u32,

    /// vertical resolution, in pixels
    #[argh(option, short = 'h')]
    pub height: u32,

    /// starting color for the gradient (default: #000000)
    #[argh(option, short = 'a', default = "String::from(\"#000000\")")]
    pub color_a: String,

    /// ending color for the gradient (default: #FFFFFF)
    #[argh(option, short = 'b', default = "String::from(\"#FFFFFF\")")]
    pub color_b: String,

    /// scaling factor, controls how "zoomed in" the noise is (default: 3.0)
    #[argh(option, default = "3.0")]
    pub scale: f64,

    /// amount of uniform random noise applied to the final image (default: 0.2)
    #[argh(option, default = "0.2")]
    pub noise: f64,

    /// output filename (default: gradient.png)
    #[argh(option, short = 'o', default = "String::from(\"gradient.png\")")]
    pub output: String,

    /// the specific shader to use for generating the gradient.
    /// Required as a subcommand.
    #[argh(subcommand)]
    pub shader: ShaderCommand,
}

/// Enum representing the available shaders and their specific configurations.
#[derive(FromArgs)]
#[argh(subcommand)]
pub enum ShaderCommand {
    Shard(ShardConfig),
}

// Configuration options specific to the Shard noise shader.
#[derive(FromArgs)]
#[argh(subcommand, name = "shard")]
/// Shard noise shader by ENDESGA
pub struct ShardConfig {
    /// sharpness of the gradient (default: 33.0)
    #[argh(option, default = "33.0")]
    pub sharpness: f64,
}

impl ShaderCommand {
    /// Consumes the enum variant and builds the corresponding trait object
    /// that implements `Shader`. This allows the renderer to evaluate the
    /// shader without knowing its concrete type.
    pub fn build_shader(&self) -> Box<dyn Shader> {
        match self {
            ShaderCommand::Shard(config) => Box::new(ShardNoise {
                sharpness: config.sharpness,
            }),
        }
    }
}

impl GradientSettings {
    /// Transforms the raw CLI strings and settings into a strongly-typed `RenderConfig`.
    ///
    /// This parses hex color strings into float RGB tuples and builds the requested shader.
    /// Returns a tuple containing the `RenderConfig` and the target output path string.
    pub fn into_render_config(self) -> (RenderConfig, String) {
        let config = RenderConfig {
            width: self.width,
            height: self.height,
            color_a: hex_to_rgb(&self.color_a),
            color_b: hex_to_rgb(&self.color_b),
            scale: self.scale,
            noise: self.noise,
            shader: self.shader.build_shader(),
        };
        (config, self.output)
    }
}
