pub const IDENT_RGB: u8 = 0b11111110;
pub const IDENT_RGBA: u8 = 0b11111111;
pub const IDENT_INDEX: u8 = 0b00000000;
pub const IDENT_DIFF: u8 = 0b01000000;
pub const IDENT_LUMA: u8 = 0b10000000;
pub const IDENT_RUN: u8 = 0b11000000;
pub const FIRST_2_BYTES: u8 = 0b11000000;
pub const SECOND_2_BYTES: u8 = 0b00110000;
pub const THIRD_2_BYTES: u8 = 0b00001100;
pub const LAST_2_BYTES: u8 = 0b00000011;
pub const LAST_6_BYTES: u8 = 0b00111111;
pub const FIRST_4_BYTES: u8 = 0b11110000;
pub const LAST_4_BYTES: u8 = 0b00001111;



pub fn char_array_to_u8_array<const N: usize>(array: & [char; N]) -> [u8; N] {
    let mut res: [u8; N] = [0; N];
    for i in 0..array.len() {
        res[i] = array[i] as u8;
    }
    res
}

pub fn u32_to_u8_array(num: &u32) -> [u8; 4] {
    num.to_be_bytes()
}

pub fn u8_array_to_u32(array: &[u8; 4]) -> u32 {
    let res: u32 = (array[0] as u32) << 24 | (array[1] as u32) << 16 | (array[2] as u32) << 8 | array[3] as u32;
    res
}