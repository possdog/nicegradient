// by ENDESGA: https://www.shadertoy.com/view/dlKyWw
use std::f32::consts;

const TAU: f64 = consts::TAU as f64;

// GLSL-like fract() implementation
fn fract(x: f64) -> f64 {
    x - x.floor()
}

fn hash(p: [f64; 3]) -> [f64; 3] {
    let x = p[0] * 127.1 + p[1] * 311.7 + p[2] * 74.7;
    let y = p[0] * 269.5 + p[1] * 183.3 + p[2] * 246.1;
    let z = p[0] * 113.5 + p[1] * 271.9 + p[2] * 124.6;

    [
        fract(x.sin() * 43758.5453123),
        fract(y.sin() * 43758.5453123),
        fract(z.sin() * 43758.5453123),
    ]
}

pub fn shard_noise(p: [f64; 3], sharpness: f64) -> f64 {
    let ip = [p[0].floor(), p[1].floor(), p[2].floor()];
    let fp = [fract(p[0]), fract(p[1]), fract(p[2])];

    let mut v = 0.0;
    let mut t = 0.0;

    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let o = [x as f64, y as f64, z as f64];
                let io = [ip[0] + o[0], ip[1] + o[1], ip[2] + o[2]];
                let h = hash(io);
                let r = [
                    fp[0] - (o[0] + h[0]),
                    fp[1] - (o[1] + h[1]),
                    fp[2] - (o[2] + h[2]),
                ];

                let dot_r = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
                let w = (-TAU * dot_r).exp2();

                let h2 = hash([io[0] + 11.0, io[1] + 31.0, io[2] + 47.0]);
                let dot_s = r[0] * (h2[0] - 0.5) + r[1] * (h2[1] - 0.5) + r[2] * (h2[2] - 0.5);
                let s = sharpness * dot_s;

                v += w * s * (1.0 + s * s).sqrt().recip();
                t += w;
            }
        }
    }

    (v / t) * 0.5 + 0.5
}
