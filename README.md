# DTEK-V Sprite Packer

Script for packing sprites of any [supported format](https://docs.rs/image/latest/image/codecs/index.html#supported-formats) into a C file of arrays in the native format of the DTEK-V's VGA output.

It takes in 2 paths as CLI arguments, the first one being the source directory containing all sprites and the second one being the target file that will contain the arrays. The script does not overwrite the target file if it already exists.

Example packing the directory `sprites` into a file named `sprites.c`:
```bash
cargo run sprites sprites.c
```
