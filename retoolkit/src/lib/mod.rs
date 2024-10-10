mod rle;
mod file;

pub struct Rle;
pub struct Io;

impl Rle {
    pub fn compress(source: &[u8]) -> Vec<u8> {
        rle::compress(source);
    }

    pub fn decompress(source: &[u8]) -> Vec<u8> {
        rle::decompress(source);
    }
}

impl Io {
    pub struct File;

    impl File {
        pub fn write(path: String, content: [u8]) -> std::io::Result<()> {
            file::write(path, content);
        }

        pub fn open(path: String) -> std::io::Result<()> {
            file::open(path);
        }
    }
}
