use std::{
    env,
    fs::{File, read_dir},
    io::Write,
};

use image::ImageReader;

fn main() {
    let mut args = env::args();

    let mut output = String::from("#include <stdint.h>\n\n");

    read_dir(
        args.nth(1)
            .expect("No source directory path (first argument) provided!"),
    )
    .unwrap()
    .map(Result::unwrap)
    .for_each(|file| {
        if let Ok(image) = ImageReader::open(file.path()).unwrap().decode() {
            let mut init = String::new();

            image.to_rgb8().rows().for_each(|row| {
                let mut row_init = String::new();

                row.for_each(|pixel| {
                    let packed_pix = ((pixel.0[0] / 32) << 5)
                        | ((pixel.0[1] / 32) << 2)
                        | ((pixel.0[2] / 64) & 3); // Pack to B2G3R3.
                    row_init.push_str(&format!("{packed_pix},"));
                });

                row_init.pop(); // Remove the last comma.

                init.push_str(&format!("{{{row_init}}},"));
            });

            output.push_str(&format!(
                "const uint8_t {}[{}][{}] = {{{init}}};\n",
                {
                    let mut name = file.file_name().into_string().unwrap();
                    name.truncate(name.len() - 4); // Remove ".png".
                    name
                },
                image.height(),
                image.width()
            ));
        }
    });

    File::create_new(
        args.next()
            .expect("No target file path (second argument) provided!"),
    )
    .unwrap()
    .write_all(output.as_bytes())
    .unwrap();
}
