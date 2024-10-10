mod rle;

pub struct Rle;

impl Rle {
    pub fn compress(source: &[u8]) -> Vec<u8> {
        rle::compress(source)
    }

    pub fn decompress(source: &[u8]) -> Vec<u8> {
        rle::decompress(source)
    }
}
