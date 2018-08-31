extern crate md5;
extern crate nds;

#[cfg(test)]
mod tests {
    use super::nds::{Extractor, Builder};
    use super::md5::compute;

    #[test]
    fn built_rom_is_same() {
        use std::fs::{read, remove_dir_all, remove_file};

        let extractor = Extractor::new("small.nds")
            .expect("Could not make Extractor");

        extractor.extract("output")
            .expect("Could not extract");

        let builder = Builder::new("output")
            .expect("Could not create builder");

        builder.build("built.nds")
            .expect("Could not build");

        let original = read("small.nds")
            .expect("Could not read small.nds'");

        let built = read("built.nds")
            .expect("Could not read built.nds'");

        let original_md5 = compute(&original);
        let built_md5 = compute(&built);

        remove_dir_all("output").unwrap();
        remove_file("built.nds").unwrap();

        assert!(&original_md5 == &built_md5);
    }
}