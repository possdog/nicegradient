# nicegradient

> 🤖 This project was made by a human with LLM assistance.

CLI for making nice gradients.

```
❯ ./target/release/nicegradient --help
Usage: nicegradient -w <width> -h <height> [-a <color-a>] [-b <color-b>] [--sharpness <sharpness>] [--scale <scale>] [--noise <noise>] [-o <output>]

Create a nice gradient.

Options:
  -w, --width       horizontal resolution, in pixels
  -h, --height      vertical resolution, in pixels
  -a, --color-a     starting color for the gradient (default: #000000)
  -b, --color-b     ending color for the gradient (default: #FFFFFF)
  --sharpness       sharpness of the gradient (default: 33.0)
  --scale           scaling factor (default: 3.0)
  --noise           amount of noise (default: 0.2)
  -o, --output      output filename (default: gradient.png)
  --help, help      display usage information
```