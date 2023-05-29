use palette::{rgb::Rgb, Srgb};
use serde::{Deserialize, Serialize, Serializer};
use std::{
    fs::{self, read_dir, File, FileType},
    iter::Sum,
    ops::{Add, DivAssign},
    ptr::read,
};

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    name: String,
    color: Srgb,
    bin: Vec<u8>,
}

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/defaultpack");

    let texture_dir = read_dir("src/defaultpack/assets/minecraft/textures/block")
        .expect("default minecraft textures");

    let mut blocks: Vec<Block> = vec![];

    for maybe_file in texture_dir {
        let file = maybe_file.expect("valid file");
        if !file.file_name().to_str().unwrap().ends_with(".png") {
            continue;
        }
        let img = image::open(file.path()).expect("valid png").to_rgba8();
        if img.height() != img.width() {
            continue;
        }
        let found_non_transparent_pixel = img.enumerate_pixels().find(|x| x.2 .0[3] != 255);
        if found_non_transparent_pixel.is_some() {
            continue;
        }
        let mut temp: (Vec<u8>, Vec<u8>, Vec<u8>) = (vec![], vec![], vec![]);
        for pixel in img.enumerate_pixels() {
            let (r, g, b) = (pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]);
            temp.0.push(r);
            temp.1.push(g);
            temp.2.push(b);
        }

        trait Average {
            fn avg(&self) -> f32;
        }
        impl Average for Vec<u8> {
            fn avg(&self) -> f32 {
                let mut sum = 0u32;
                for x in self.iter() {
                    sum += *x as u32;
                }
                sum as f32 / self.len() as f32
            }
        }

        let average_color = (temp.0.avg(), temp.1.avg(), temp.2.avg());
        blocks.push(Block {
            name: file
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".png")
                .to_owned(),
            color: Srgb::new(
                average_color.0 / 255f32,
                average_color.1 / 255f32,
                average_color.2 / 255f32,
            ),
            bin: std::fs::read(file.path()).unwrap(),
        })
    }

    let block_table_bin = bincode::serialize(&blocks).expect("serialization");

    fs::write("src/block_table", block_table_bin).expect("block_table file write");
}
