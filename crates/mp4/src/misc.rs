pub fn parse_u64(value: &[u8]) -> u64 {
    u64::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_i64(value: &[u8]) -> i64 {
    i64::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_u32(value: &[u8]) -> u32 {
    u32::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_i32(value: &[u8]) -> i32 {
    i32::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_u16(value: &[u8]) -> u16 {
    u16::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_i16(value: &[u8]) -> i16 {
    i16::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_u8(value: &[u8]) -> u8 {
    u8::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_i8(value: &[u8]) -> i8 {
    i8::from_be_bytes(value.try_into().unwrap())
}

pub fn parse_utf8(value: &[u8]) -> String {
    String::from_utf8(value.into()).unwrap()
}

pub fn parse_duration(value: u64, timescale: u32) -> (u64, u64, u64) {
    let second = value / timescale as u64;
    let minute = second / 60;
    let hour = minute / 60;
    (hour % 60, minute % 60, second % 60)
}
