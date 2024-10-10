mod rle;
mod png;

pub struct Rle;

impl Rle {
    pub fn compress(source: &[u8]) -> Vec<u8> {
        rle::compress(source)
    }
    
    pub fn decompress(source: &[u8]) -> Vec<u8> {
        rle::decompress(source)
    }
}

pub struct Image;

impl Image {
    pub fn png(input_file: &str, output_file: &str) -> std::io::Result<()> {
        png::compress(input_file, output_file)
    }
}
