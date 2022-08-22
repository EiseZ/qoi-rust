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