pub fn get_octet(x: u16, num: u16) -> u8 {
    ((x & (0xF << num)) >> num) as u8
}

pub fn to_octets(x: u16) -> (u8, u8, u8, u8) {
    let oct0 = get_octet(x, 0);
    let oct1 = get_octet(x, 4);
    let oct2 = get_octet(x, 8);
    let oct3 = get_octet(x, 12);

    (oct3, oct2, oct1, oct0)
}

pub fn to_addr((oct0, oct1, oct2) : (u8, u8, u8)) -> u16 {
    (oct0 as u16) << 8 | (oct1 as u16) << 4 | (oct2 as u16)
}

pub fn to_id((oct0, oct1, oct2, oct3) : (u8, u8, u8, u8)) -> u16 {
    (oct0 as u16) << 12 | (oct1 as u16) << 8 | (oct2 as u16) << 4 | (oct3 as u16)
}

pub fn to_u8((oct0, oct1): (u8, u8)) -> u8 {
    (oct0 << 4) | oct1
}

