# DTEK-V Sprite Packer

Rust CLI tool for packing sprite textures of any [supported format](https://docs.rs/image/latest/image/codecs/index.html#supported-formats) into a C file of 2D arrays in the native format of the DTEK-V's VGA output. The textures are converted to the right color format automatically, and **the alpha channel is discarded**. The arrays are named after the files, ignoring extensions, so the file names must be legal variable names in C.

It takes in 2 paths as CLI arguments, the first one being the source directory containing all sprites and the second one being the target file that will contain the arrays. The target file is not overwritten if it already exists.

Cargo is required to compile the program, and can be used to run it too. Example packing the directory `sprites` into a file named `sprites.c` (execute in the same directory as `Cargo.toml`):
```bash
cargo run sprites sprites.c
```
