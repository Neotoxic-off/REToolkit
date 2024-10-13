mod rle;
mod lz77;
mod png;

pub fn dummy_compress(_input_file: &str, _output_file: &str) -> std::io::Result<()> {
    Ok(())
}

pub struct Rle;

impl Rle {
    pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        rle::compress(input_file, output_file)
    }
    
    pub fn decompress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        rle::decompress(input_file, output_file)
    }
}
pub struct Lz77;

impl Lz77 {
    pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        lz77::compress(input_file, output_file)
    }
    
    pub fn decompress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        lz77::decompress(input_file, output_file)
    }
}

pub struct Image;

impl Image {
    pub fn png(input_file: &str, output_file: &str) -> std::io::Result<()> {
        png::compress(input_file, output_file)
    }
}
