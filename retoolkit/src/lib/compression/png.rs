use std::fs::File;
use std::io::{BufReader, BufWriter};
use png::{Decoder, Encoder, Compression};

pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    // Open the input PNG file
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    // Create a PNG decoder
    let decoder = Decoder::new(reader);
    
    // Read the image info and get a Reader for pixel data
    let mut reader = decoder.read_info().expect("Failed to decode PNG");

    // Allocate a buffer to store image data
    let mut buf = vec![0; reader.output_buffer_size()];

    // Read the pixel data into the buffer
    reader.next_frame(&mut buf).expect("Failed to read PNG frame");

    // Open the output file for writing
    let output_file = File::create(output_file)?;
    let writer = BufWriter::new(output_file);

    // Create a PNG encoder
    let mut encoder = Encoder::new(writer, reader.info().width, reader.info().height);
    
    // Set compression settings
    encoder.set_compression(Compression::Best);
    encoder.set_color(reader.info().color_type);
    encoder.set_depth(reader.info().bit_depth);

    // Write the PNG header and pixel data
    let mut png_writer = encoder.write_header().expect("Failed to write PNG header");
    png_writer.write_image_data(&buf).expect("Failed to write PNG data");

    Ok(())
}