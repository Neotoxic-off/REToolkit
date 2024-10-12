mod rle;
mod png;

pub struct Rle;

impl Rle {
    pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        rle::compress(input_file, output_file)
    }
    
    pub fn decompress(input_file: &str, output_file: &str) -> std::io::Result<()> {
        rle::decompress(input_file, output_file)
    }
}

pub struct Image;

impl Image {
    pub fn png(input_file: &str, output_file: &str) -> std::io::Result<()> {
        png::compress(input_file, output_file)
    }
}
