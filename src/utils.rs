pub fn calc_addr(msb: u8, lsb: u8) -> u16 {
    (((msb & 0x0F) as u16) << 8) | (lsb as u16)
}