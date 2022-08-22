use crate::bytes::{u8_array_to_u32, IDENT_RGB, IDENT_RGBA, IDENT_INDEX, IDENT_DIFF, IDENT_LUMA, IDENT_RUN, FIRST_2_BYTES, SECOND_2_BYTES, THIRD_2_BYTES, LAST_2_BYTES, LAST_6_BYTES, FIRST_4_BYTES, LAST_4_BYTES};

use crate::{pixel::Pixel, QoiHeader};

pub fn decode_image(data: Vec<u8>) -> (Vec<u8>, QoiHeader) {
    let mut data_out: Vec<u8> = Vec::new();

    let header = QoiHeader {
        _magic: ['q', 'o', 'i', 'f'],
        width: u8_array_to_u32(data[4..=7].try_into().unwrap()),
        height: u8_array_to_u32(data[8..=11].try_into().unwrap()),
        channels: data[12],
        _colorspace: data[13],
    };

    let mut hash_pixels: [Option<Pixel>; 64] = [None; 64];
    let mut prev_pixel = Pixel { r: 0, g: 0, b: 0, a: 255 };
    let mut current_byte = 14;

    loop {
        if data[current_byte] == IDENT_RGB {
            data_out.push(data[current_byte + 1]);
            data_out.push(data[current_byte + 2]);
            data_out.push(data[current_byte + 3]);
            data_out.push(prev_pixel.a);
            current_byte += 4;
        } else if data[current_byte] == IDENT_RGBA {
            data_out.push(data[current_byte + 1]);
            data_out.push(data[current_byte + 2]);
            data_out.push(data[current_byte + 3]);
            data_out.push(data[current_byte + 4]);
            current_byte += 5;
        } else if data[current_byte] & FIRST_2_BYTES == IDENT_INDEX {
            let hash_pixel = hash_pixels[data[current_byte] as usize].unwrap();
            data_out.push(hash_pixel.r);
            data_out.push(hash_pixel.g);
            data_out.push(hash_pixel.b);
            data_out.push(hash_pixel.a);
            current_byte += 1;
        } else if data[current_byte] & FIRST_2_BYTES == IDENT_DIFF {
            data_out.push(((prev_pixel.r as i8).wrapping_add(((data[current_byte] & SECOND_2_BYTES) >> 4) as i8 - 2)) as u8);
            data_out.push(((prev_pixel.g as i8).wrapping_add(((data[current_byte] & THIRD_2_BYTES) >> 2) as i8 - 2)) as u8);
            data_out.push((prev_pixel.b as i8).wrapping_add(((data[current_byte] & LAST_2_BYTES)) as i8 - 2) as u8);
            data_out.push(prev_pixel.a);
            current_byte += 1;
        } else if data[current_byte] & FIRST_2_BYTES == IDENT_LUMA {
            let diff_g: i8 = ((data[current_byte] & LAST_6_BYTES) as i8 - 32 as i8) as i8;
            let diff_r: i8 = (((data[current_byte + 1] & FIRST_4_BYTES) >> 4) as i8 - 8 as i8) as i8;
            let diff_b: i8 = ((data[current_byte + 1] & LAST_4_BYTES) as i8 - 8 as i8) as i8;
            data_out.push((prev_pixel.r as i8).wrapping_add(diff_r + diff_g) as u8);
            data_out.push((prev_pixel.g as i8).wrapping_add(diff_g) as u8);
            data_out.push((prev_pixel.b as i8).wrapping_add(diff_b + diff_g) as u8);
            data_out.push(prev_pixel.a);
            current_byte += 2;
        } else if data[current_byte] & FIRST_2_BYTES == IDENT_RUN {
            for _ in 0..=(data[current_byte] & LAST_6_BYTES) {
                data_out.push(prev_pixel.r);
                data_out.push(prev_pixel.g);
                data_out.push(prev_pixel.b);
                data_out.push(prev_pixel.a);
            }
            current_byte += 1;
        }

        prev_pixel = Pixel { r: data_out[data_out.len() - 4], g: data_out[data_out.len() - 3], b: data_out[data_out.len() - 2], a: data_out[data_out.len() - 1] };
        
        let hash = ((prev_pixel.r as u32 * 3 + prev_pixel.g as u32 * 5 + prev_pixel.b as u32 * 7 + prev_pixel.a as u32 * 11) % 64) as usize;
        hash_pixels[hash] = Some(prev_pixel);

        if current_byte == data.len() - 8 {
            break;
        }
    }

    (data_out, header)
}