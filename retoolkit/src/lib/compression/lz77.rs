use std::fs::File;
use std::io::{self, Write, BufReader, BufWriter, BufRead};

use crate::lib;

pub fn compress(input_file: &str, output_file: &str) -> io::Result<()> {
    let window_size: usize = 4096;
    let lookahead_buffer_size: usize = 15;
    let input_file: Vec<u8> = lib::io::File::open(input_file).unwrap();
    let output_file = File::create(output_file)?;
    let mut writer = BufWriter::new(output_file);

    let mut cursor = 0;

    while cursor < input_file.len() {
        let (distance, length, next_char) = find_longest_match(&input_file, cursor, window_size, lookahead_buffer_size);

        writer.write_all(&distance.to_le_bytes())?;
        writer.write_all(&[length as u8])?;
        writer.write_all(&[next_char])?;

        cursor += length + 1;
    }

    Ok(())
}

fn find_longest_match(data: &[u8], cursor: usize, window_size: usize, lookahead_buffer_size: usize) -> (u16, usize, u8) {
    let window_start = if cursor >= window_size {
        cursor - window_size
    } else {
        0
    };

    let lookahead_end = std::cmp::min(cursor + lookahead_buffer_size, data.len());

    let mut best_distance = 0;
    let mut best_length = 0;

    for i in window_start..cursor {
        let mut match_length = 0;

        while cursor + match_length < lookahead_end && data[i + match_length] == data[cursor + match_length] {
            match_length += 1;

            if match_length >= lookahead_buffer_size {
                break;
            }
        }

        if match_length > best_length {
            best_distance = cursor - i;
            best_length = match_length;
        }
    }

    let next_char = if cursor + best_length < data.len() {
        data[cursor + best_length]
    } else {
        0
    };

    (best_distance as u16, best_length, next_char)
}

pub fn decompress(input_file: &str, output_file: &str) -> io::Result<()> {
    let input_file = File::open(input_file)?;
    let mut reader = BufReader::new(input_file);

    let mut decompressed_data = Vec::new();

    // Read until end of file
    while let Ok(buffer) = reader.fill_buf() {
        if buffer.is_empty() {
            break;
        }

        let mut cursor = 0;
        while cursor < buffer.len() {
            if cursor + 4 > buffer.len() {
                break; // Ensure we don't read beyond the buffer
            }

            // Read the distance, length, and next character
            let distance = u16::from_le_bytes([buffer[cursor], buffer[cursor + 1]]);
            let length = buffer[cursor + 2] as usize;
            let next_char = buffer[cursor + 3];

            // Handle the decompressed copy based on distance and length
            let start = decompressed_data.len().saturating_sub(distance as usize);
            for i in 0..length {
                let byte = decompressed_data[start + i];
                decompressed_data.push(byte);
            }

            // Add the next literal character
            decompressed_data.push(next_char);

            // Move the cursor forward
            cursor += 4; // Distance (2 bytes), length (1 byte), next_char (1 byte)
        }

        // Consume the buffer we've processed
        reader.consume(cursor);
    }

    // Write decompressed data to the output file
    let output_file = File::create(output_file)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&decompressed_data)?;

    Ok(())
}