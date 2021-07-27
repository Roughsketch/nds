use nds::parser::NDSParser;
use std::convert::TryFrom;

// our testing .nds files
const TEST_HELLO_WORLD: &'static str = "tests/test_nds_files/hello_world.nds";
const TEST_3D_BOTH_SCREENS: &'static str = "tests/test_nds_files/3D_Both_Screens.nds";

#[test]
fn test_parsing() {
    assert!(NDSParser::try_from(TEST_HELLO_WORLD).is_ok());
}
