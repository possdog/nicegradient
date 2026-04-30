//! Utility for parsing color strings.

use colors_transform::{Color, Rgb};

/// Parses a hex color string (e.g. `#FF0000` or `FF0000`) into a normalized RGB tuple.
///
/// Returns `(r, g, b)` where each channel is a float between 0.0 and 1.0.
///
/// Panics if the hex string is invalid.
pub fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    // Gracefully handle strings missing the leading '#' symbol
    // which is common in CLI inputs.
    let hex_with_prefix = if hex.starts_with('#') {
        hex.to_string()
    } else {
        format!("#{}", hex)
    };

    let rgb = Rgb::from_hex_str(&hex_with_prefix).unwrap();

    // Normalize channels from [0, 255] down to [0.0, 1.0]
    // as our rendering math expects normalized floats.
    (
        rgb.get_red() / 255.0,
        rgb.get_green() / 255.0,
        rgb.get_blue() / 255.0,
    )
}
