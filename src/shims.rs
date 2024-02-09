//! Temporary shims to get things working for demos. These should NOT be used in the final version.

pub fn convert_to_u8_bytes(input: Vec<u32>) -> Vec<u8> {
    let mut res = Vec::new();
    for int in input {
        for byte in int.to_le_bytes() {
            res.push(byte);
        }
    }
    res
}

pub fn convert_from_u8_bytes(input: Vec<u8>) -> Vec<u32> {
    let mut res = Vec::new();
    for int_bytes in input.chunks(4) {
        res.push(u32::from_le_bytes(int_bytes.try_into().unwrap()));
    }
    res
}
