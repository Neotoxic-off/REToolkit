use std::fs::File;
use std::io::{BufReader, BufWriter};
use png::{Decoder, Encoder, Compression};

pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let decoder = Decoder::new(reader);
    let (info, mut reader) = decoder.read_info().expect("Failed to decode PNG");
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("Failed to read PNG frame");

    let output_file = File::create(output_file)?;
    let writer = BufWriter::new(output_file);

    let mut encoder = Encoder::new(writer, info.width, info.height);
    encoder.set_compression(Compression::Fast);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut png_writer = encoder.write_header().expect("Failed to write PNG header");
    png_writer.write_image_data(&buf).expect("Failed to write PNG data");

    Ok(())
}
