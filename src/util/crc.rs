pub fn crc16(data: &[u8]) -> u16 {
    let masks = [0xC0C1,0xC181,0xC301,0xC601,0xCC01,0xD801,0xF001,0xA001];
    let mut crc = 0xFFFF;

    for byte in data {
        crc ^= byte;

        for (index, mask) in masks.enumerate() {
            let carry = crc & 1;

            crc >>= 1;

            if carry {
                crc ^= mask << (7 - index);
            }
        }
    }

    crc
}