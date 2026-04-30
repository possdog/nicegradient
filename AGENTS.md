# OpenCode Agent Instructions

This is a single-crate Rust CLI application that generates gradient images.

## Architecture & Conventions
- **Entrypoints & Configuration:** `src/main.rs` is a thin wrapper. CLI parsing (`argh`) and configuration is handled in `src/cli.rs`.
- **Rendering:** `src/render.rs` contains the main image generation loop. It uses the `image` crate to create `Rgb32FImage` buffers, processed in parallel using `rayon`.
- **Shaders:** Procedural noise logic is abstracted behind the `Shader` trait (`src/shader.rs`). 
  - Implementations like `src/shard_noise.rs` (a port of a GLSL shader) provide specific effects.
  - To add a new shader, implement the `Shader` trait and add a new subcommand to `ShaderCommand` in `src/cli.rs`.

## Verification & Workflow
- Use standard Rust tooling: `cargo check`, `cargo test`, `cargo fmt`.
- **Visual Verification:** To test output changes, run the binary and inspect the generated image. 
  - **Important:** The CLI *requires* a shader subcommand to run.
  - Example: `cargo run -- -w 500 -h 500 -o test.png shard --sharpness 33.0`
