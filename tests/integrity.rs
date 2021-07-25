use md5::compute;
use nds::{Builder, Extractor};
use std::panic;

// our testing .nds files
const TEST_HELLO_WORLD: &'static str = "tests/test_nds_files/hello_world.nds";
const TEST_3D_BOTH_SCREENS: &'static str = "tests/test_nds_files/3D_Both_Screens.nds";

#[test]
fn extract_tiny() {
    let extractor = Extractor::new(TEST_HELLO_WORLD, false).expect("Could not make Extractor");

    assert!(extractor.extract("tmp/tiny").is_ok());
}

#[test]
fn checksum_matches() {
    assert!(Extractor::new(TEST_HELLO_WORLD, true).is_ok());
    assert!(Extractor::new(TEST_3D_BOTH_SCREENS, true).is_ok());
}

#[test]
fn built_rom_is_same() {
    run_test(_built_rom_is_same, _built_rom_is_same_cleanup);
}

fn _built_rom_is_same() {
    use std::fs::read;

    let extractor = Extractor::new(TEST_HELLO_WORLD, true).expect("Could not make Extractor");

    extractor.extract("output").expect("Could not extract");

    let builder = Builder::new("output").expect("Could not create builder");

    builder.build("built.nds").expect("Could not build");

    let original = read(TEST_HELLO_WORLD).expect("Could not read small.nds'");

    let built = read("built.nds").expect("Could not read built.nds'");

    let original_md5 = compute(&original);
    let built_md5 = compute(&built);

    assert!(&original_md5 == &built_md5);
}

fn _built_rom_is_same_cleanup() {
    use std::fs::{remove_dir_all, remove_file};

    let _ = remove_dir_all("output");
    let _ = remove_file("built.nds");
}

fn run_test<T, U>(test: T, cleanup: U) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
    U: FnOnce(),
{
    let result = panic::catch_unwind(|| test());

    cleanup();

    assert!(result.is_ok());
}
