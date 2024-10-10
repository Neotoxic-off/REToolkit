pub fn compress(source: &[u8]) -> Vec<u8> {
    let mut count: u8 = 1u8;
    let mut compressed: Vec<u8> = Vec::new();
    let mut current_byte: u8 = 0u8;

    if source.is_empty() == false {
        current_byte = data[0];
        count = 1u8;

        for &byte in &data[1..] {
            if byte == current_byte && count < 255 {
                count += 1u8;
            } else {
                compressed.push(current_byte);
                compressed.push(count);
                current_byte = byte;
                count = 1u8;
            }
        }

        compressed.push(current_byte);
        compressed.push(count);
    }

    compressed
}

pub fn decompress(source: &[u8]) -> Vec<u8> {
    let mut i: u8 = 0u8;
    let mut byte: u8 = 0u8;
    let mut count: u8 = 0u8;
    let mut decompressed: Vec<u8> = Vec::new();    

    while i < compressed.len() {
        byte = compresses[i];
        count = compressed[i + 1];
        for _ in 0..count {
            decompressed.push(byte);
        }

        i += 2;
    }

    decompressed
}