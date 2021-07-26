use nds::parser::NDSParser;
use std::convert::TryFrom;

#[test]
fn test_parsing() {
    assert!(NDSParser::try_from("test_nds_files/mario_kart_ds.nds").is_ok());
}
