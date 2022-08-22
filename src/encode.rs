use crate::{pixel::{Pixel, pixels_eq}, QoiHeader, bytes::{self, IDENT_DIFF, IDENT_LUMA}};

pub fn encode_image(header: QoiHeader, data: Vec<u8>) -> Vec<u8> {
    let mut data_out: Vec<u8> = Vec::new();

    // Add header to data
    data_out.extend_from_slice(&bytes::char_array_to_u8_array(&header._magic));
    data_out.extend_from_slice(&bytes::u32_to_u8_array(&header.width));
    data_out.extend_from_slice(&bytes::u32_to_u8_array(&header.height));
    data_out.push(header.channels);
    data_out.push(header._colorspace);

    let last_pixel = (header.width * header.height * header.channels as u32 - header.channels as u32) as usize;

    let mut hash_pixels: [Option<Pixel>; 64] = [None; 64];
    let mut prev_pixel = Pixel { r: 0, g: 0, b: 0, a: 255 };
    let mut run = 0;
    for y in 0..header.height {
        for x in 0..header.width {
            let pixel_location = ((y * header.width + x) * header.channels as u32) as usize;
            let r = data[pixel_location];
            let g = data[pixel_location + 1];
            let b = data[pixel_location + 2];
            let a = if header.channels == 4 { data[pixel_location + 3] } else { 255 };
            let pixel = Pixel { r: r, g: g, b: b, a: a };

            // Run
            if pixels_eq(prev_pixel, pixel) {
                run += 1;
                if run == 62 || pixel_location == last_pixel {
                    let identifier = 3 << 6;
                    data_out.push(identifier | (run - 1));
                    run = 0;
                }
            } else {
                if run > 0 {
                    let identifier = 3 << 6;
                    data_out.push(identifier | (run - 1));
                    run = 0;
                }

                // Hash
                let pixel_hash = ((r as u32 * 3 + g as u32 * 5 + b as u32 * 7 + a as u32 * 11) % 64) as u8;
                match hash_pixels[pixel_hash as usize] {
                    Some(hash_pixel) => {
                        if pixels_eq(pixel, hash_pixel) {
                            data_out.push(pixel_hash);
                            prev_pixel = pixel;
                            continue;
                        }
                    },
                    None => {}
                }
                
                hash_pixels[pixel_hash as usize] = Some(Pixel { r: r, g: g, b: b, a: a });

                if pixel.a == prev_pixel.a {
                    // Small difference
                    let diff_r = r.wrapping_sub(prev_pixel.r);
                    let diff_g = g.wrapping_sub(prev_pixel.g);
                    let diff_b = b.wrapping_sub(prev_pixel.b);

                    // Big difference
                    let diff_r_g = r.wrapping_sub(prev_pixel.r).wrapping_sub(diff_g);
                    let diff_b_g = b.wrapping_sub(prev_pixel.b).wrapping_sub(diff_g);

                    if (diff_r >= 254 || diff_r <= 1) &&
                       (diff_g >= 254 || diff_g <= 1) &&
                       (diff_b >= 254 || diff_b <= 1) {
                        // Small difference
                        data_out.push(IDENT_DIFF | (diff_r.wrapping_add(2)) << 4 | (diff_g.wrapping_add(2)) << 2 | diff_b.wrapping_add(2));
                    } else if (diff_r_g >= 248 || diff_r_g <= 7) &&
                              (diff_g >= 224 || diff_g <= 31) &&
                              (diff_b_g >= 248 || diff_b_g <= 7) {
                        // Big difference
                        data_out.push(IDENT_LUMA | diff_g.wrapping_add(32));
                        data_out.push(diff_r_g.wrapping_add(8) << 4 | diff_b_g.wrapping_add(8));
                    } else {
                        // Normal rgb
                        data_out.push(0b11111110); 
                        data_out.push(r);
                        data_out.push(g);
                        data_out.push(b);
                    }
                } else {
                    // Normal rgba
                    data_out.push(0b11111111); 
                    data_out.push(r);
                    data_out.push(g);
                    data_out.push(b);
                    data_out.push(a);
                }
            }

            prev_pixel = pixel;
        }
    }

    // Closing bits
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x00);
    data_out.push(0x01);

    data_out
}