use image;
use std::fs;
use std::path::Path;

mod pixel;
mod bytes;
mod encode;

pub struct QoiHeader {
    _magic: [char; 4],
    width: u32,
    height: u32,
    channels: u8,
    _colorspace: u8,
}

fn main() {
    // Get image data + metadata
    let input_img = image::open("image.png").expect("Image not found");
    
    let input_img_width = input_img.width();
    let input_img_height = input_img.height();
    let input_img_raw_data = input_img.into_bytes();
    let input_img_channels = if input_img_width * input_img_height * 3 == input_img_raw_data.len() as u32 { 3 } else { 4 };

    println!("[QOI] Original size: {}K", input_img_raw_data.len() / 1000);

    // Initialize header
    let header = QoiHeader {
        _magic: ['q', 'o', 'i', 'f'],
        width: input_img_width,
        height: input_img_height,
        channels: input_img_channels,
        _colorspace: 0,
    };
    
    println!("[QOI] Compressing...");
    let data_out = encode::encode_image(header, input_img_raw_data);
    println!("[QOI] Compressing complete!");

    println!("[QOI] Final size: {}K", data_out.len() / 1000);

    println!("[QOI] Writing to file...");
    fs::write(Path::new("res.qoi"), data_out).expect("Failed to write to file");
    println!("[QOI] Done!");
}
