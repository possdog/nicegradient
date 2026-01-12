use colors_transform::{Color, Rgb};

pub fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    // Add '#' prefix if not present
    let hex_with_prefix = if hex.starts_with('#') {
        hex.to_string()
    } else {
        format!("#{}", hex)
    };

    let rgb = Rgb::from_hex_str(&hex_with_prefix).unwrap();
    (
        rgb.get_red() / 255.0,
        rgb.get_green() / 255.0,
        rgb.get_blue() / 255.0,
    )
}
