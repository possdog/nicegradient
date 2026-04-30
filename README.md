# nicegradient

> 🤖 This project was made by a human with LLM assistance.

CLI for making nice gradients.

## Building

Simply clone this repository and `cargo build --release`.

## Usage

```
❯ ./target/release/nicegradient --help
Usage: nicegradient -w <width> -h <height> [-a <color-a>] [-b <color-b>] [--scale <scale>] [--noise <noise>] [-o <output>] <command> [<args>]

Create a nice gradient.

Options:
  -w, --width       horizontal resolution, in pixels
  -h, --height      vertical resolution, in pixels
  -a, --color-a     starting color for the gradient (default: #000000)
  -b, --color-b     ending color for the gradient (default: #FFFFFF)
  --scale           scaling factor, controls how "zoomed in" the noise is
                    (default: 3.0)
  --noise           amount of uniform random noise applied to the final image
                    (default: 0.2)
  -o, --output      output filename (default: gradient.png)
  --help, help      display usage information

Commands:
  shard             Shard noise shader by ENDESGA


❯ ./target/release/nicegradient --help shard
Usage: nicegradient shard [--sharpness <sharpness>]

Shard noise shader by ENDESGA

Options:
  --sharpness       sharpness of the gradient (default: 33.0)
  --help, help      display usage information
```