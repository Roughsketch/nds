#![feature(test)]
extern crate test;
extern crate memmap;

use test::Bencher;

#[bench]
fn test_file_read(b: &mut Bencher) {
    use std::fs::{File, metadata};
    use std::io::Read;

    b.iter(|| {
        let mut file = File::open("big.nds").unwrap();
        let meta = metadata("big.nds").unwrap();
        let mut buf = Vec::with_capacity(meta.len() as usize + 1);

        file.read_to_end(&mut buf)
    })
}

#[bench]
fn test_mmap_read(b: &mut Bencher) {
    use std::fs::File;
    use memmap::Mmap;

    b.iter(|| {
        let file = File::open("big.nds").unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };

        mmap.to_vec()
    })
}