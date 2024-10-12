use crate::lib;

pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let mut count: u8 = 1u8;
    let mut compressed: Vec<u8> = Vec::new();
    let source: Vec<u8> = lib::io::File::open(input_file).unwrap();
    let mut current_byte: u8 = 0u8;

    if source.is_empty() == false {
        current_byte = source[0];
        count = 1u8;

        for &byte in &source[1..] {
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

    lib::io::File::write(output_file, compressed.as_slice());

    Ok(())
}

pub fn decompress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let mut i: usize = 0;
    let mut byte: u8 = 0u8;
    let mut count: u8 = 0u8;
    let mut decompressed: Vec<u8> = Vec::new();
    let source: Vec<u8> = lib::io::File::open(input_file).unwrap();

    while i < source.len() {
        byte = source[i];
        count = source[i + 1];
        for _ in 0..count {
            decompressed.push(byte);
        }

        i += 2;
    }

    lib::io::File::write(output_file, decompressed.as_slice());

    Ok(())
}