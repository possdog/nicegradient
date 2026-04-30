//! Shader trait definition.
//!
//! This module defines the common interface that all procedural noise algorithms
//! must implement to be used by the rendering engine.

/// A trait that defines a generalized 3-dimensional procedural noise function.
///
/// The `Sync + Send` bounds are required because the rendering engine evaluates
/// shaders in parallel across multiple threads using Rayon.
pub trait Shader: Sync + Send {
    /// Evaluates the noise function at a given 3D coordinate.
    ///
    /// The input `p` represents the `[x, y, z]` coordinates. In the context of
    /// a 2D image, `x` and `y` are the pixel coordinates, and `z` acts as a seed
    /// or time slice to ensure unique patterns on each run.
    ///
    /// Should ideally return a value normalized to `[0.0, 1.0]`.
    fn evaluate(&self, p: [f64; 3]) -> f64;
}
