use image::{self, save_buffer_with_format};
use std::fs;
use std::path::Path;
use std::env;

mod pixel;
mod bytes;
mod encode;
mod decode;

#[derive(Debug)]
pub struct QoiHeader {
    _magic: [char; 4],
    width: u32,
    height: u32,
    channels: u8,
    _colorspace: u8,
}
#[quit::main]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("[QOI: Error] Incorrect use: Please provide 2 arguments");
        println!("[QOI] Use: qoirs input.png output.qoi");
        quit::with_code(1);
    }

    if args[1].ends_with(".png") {
        // Get image data + metadata
        let input_img = image::open(&args[1]).expect("[QOI: Error] Image not found");
        
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
        
        println!("[QOI] Encoding...");
        let data_out = encode::encode_image(header, input_img_raw_data);
        println!("[QOI] Encoding complete!");

        println!("[QOI] Final size: {}K", data_out.len() / 1000);

        println!("[QOI] Writing to file...");
        fs::write(Path::new(&args[2]), data_out).expect("[QOI: Error] Failed to write to file");
        println!("[QOI] Done!");
    } else if args[1].ends_with(".qoi") {
        let input_img_raw_data = fs::read(&args[1]).expect("[QOI: Error] Image not found");

        println!("[QOI] Decoding...");
        let (data_out, header) = decode::decode_image(input_img_raw_data);
        println!("[QOI] Decoding complete!");

        println!("[QOI] Writing to file...");
        save_buffer_with_format(&args[2], &data_out, header.width, header.height, image::ColorType::Rgba8, image::ImageFormat::Png).expect("[QOI: Error] Failed to write to file");
        println!("[QOI] Done!");
    } else {
        println!("[QOI: Error] Incorrect use: Not a valid file format: {}", args[1]);
        println!("[QOI] Use: qoirs input.png output.qoi");
        quit::with_code(1);
    }
}
