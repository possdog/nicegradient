//! Shard noise implementation.
//!
//! This is a Rust port of the "Shard Noise" GLSL shader
//! originally created by ENDESGA (https://www.shadertoy.com/view/dlKyWw).
//! It generates a visually striking, crystalline noise pattern.

use crate::shader::Shader;
use std::f32::consts;

const TAU: f64 = consts::TAU as f64;

/// A utility function that behaves like the GLSL `fract()` built-in.
/// Returns the fractional part of `x` (i.e. `x - floor(x)`).
fn fract(x: f64) -> f64 {
    x - x.floor()
}

/// A pseudo-random hash function that maps a 3D coordinate to a 3D vector.
/// It uses sine wave interference and large multipliers to simulate randomness.
fn hash(p: [f64; 3]) -> [f64; 3] {
    // Magic numbers used to scramble the input coordinates
    let x = p[0] * 127.1 + p[1] * 311.7 + p[2] * 74.7;
    let y = p[0] * 269.5 + p[1] * 183.3 + p[2] * 246.1;
    let z = p[0] * 113.5 + p[1] * 271.9 + p[2] * 124.6;

    [
        fract(x.sin() * 43758.5453123),
        fract(y.sin() * 43758.5453123),
        fract(z.sin() * 43758.5453123),
    ]
}

/// The ShardNoise configuration struct.
/// Implements `Shader` to provide the specific evaluate logic.
pub struct ShardNoise {
    /// Controls the contrast and sharpness of the generated shards.
    /// Higher values create more defined edges.
    pub sharpness: f64,
}

impl Shader for ShardNoise {
    /// Evaluates the Shard Noise algorithm at the given 3D coordinate `p`.
    fn evaluate(&self, p: [f64; 3]) -> f64 {
        // Separate the coordinate into integer (cell) and fractional parts.
        let ip = [p[0].floor(), p[1].floor(), p[2].floor()];
        let fp = [fract(p[0]), fract(p[1]), fract(p[2])];

        let mut v = 0.0; // Accumulates the weighted noise values
        let mut t = 0.0; // Accumulates the total weights for normalization

        // Iterate over a 3x3x3 grid of neighboring cells.
        // This is necessary to account for influence from adjacent noise cells.
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    // Cell offset vector
                    let o = [x as f64, y as f64, z as f64];
                    // Absolute integer coordinate of the neighboring cell
                    let io = [ip[0] + o[0], ip[1] + o[1], ip[2] + o[2]];

                    // Generate a pseudo-random point within that cell
                    let h = hash(io);

                    // Vector from our exact coordinate to the random point in the neighbor cell
                    let r = [
                        fp[0] - (o[0] + h[0]),
                        fp[1] - (o[1] + h[1]),
                        fp[2] - (o[2] + h[2]),
                    ];

                    // Squared distance to that point
                    let dot_r = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];

                    // Calculate a weight based on distance using an exponential falloff.
                    // TAU makes the falloff sharper.
                    let w = (-TAU * dot_r).exp2();

                    // Generate a second hash using a shifted coordinate to determine the "slope"
                    // or gradient vector of the noise at this cell.
                    let h2 = hash([io[0] + 11.0, io[1] + 31.0, io[2] + 47.0]);

                    // Dot product between distance vector and the gradient vector.
                    let dot_s = r[0] * (h2[0] - 0.5) + r[1] * (h2[1] - 0.5) + r[2] * (h2[2] - 0.5);

                    // Apply sharpness to the gradient dot product.
                    let s = self.sharpness * dot_s;

                    // Accumulate the value.
                    // `(1.0 + s * s).sqrt().recip()` acts as a normalization factor
                    // to prevent values from exploding when sharpness is high.
                    v += w * s * (1.0 + s * s).sqrt().recip();

                    // Accumulate weight
                    t += w;
                }
            }
        }

        // Normalize accumulated value by total weight, and map from [-1.0, 1.0] to [0.0, 1.0]
        (v / t) * 0.5 + 0.5
    }
}
