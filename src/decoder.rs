use utils;

pub trait Decoder {
    fn before(&mut self, opcode: u16, addr: u16);
    fn unknown(&mut self, opcode: u16, addr: u16);
    fn clear(&mut self);
    fn ret(&mut self);
    fn jmp(&mut self, address: u16);
    fn call(&mut self, address: u16);
    fn jeq(&mut self, vr: u8, value: u8);
    fn jneq(&mut self, vr: u8, value: u8);
    fn jeqr(&mut self, vr: u8, vy: u8);
    fn set(&mut self, vr: u8, value: u8);
    fn add(&mut self, vr: u8, value: u8);
    fn setr(&mut self, vr: u8, vy: u8);
    fn or(&mut self, vr: u8, vy: u8);
    fn and(&mut self, vr: u8, vy: u8);
    fn xor(&mut self, vr: u8, vy: u8);
    fn addr(&mut self, vr: u8, vy: u8);
    fn sub(&mut self, vr: u8, vy: u8);
    fn shr(&mut self, vr: u8);
    fn subb(&mut self, vr: u8, vy: u8);
    fn shl(&mut self, vr: u8);
    fn jneqr(&mut self, vr: u8, vy: u8);
    fn seti(&mut self, value: u16);
    fn jmpv0(&mut self, address: u16);
    fn rand(&mut self, vr: u8, value: u8);
    fn draw(&mut self, vr: u8, vy: u8, value: u8);
    fn jkey(&mut self, vr: u8);
    fn jnkey(&mut self, vr: u8);
    fn getdelay(&mut self, vr: u8);
    fn waitkey(&mut self, vr: u8);
    fn setdelay(&mut self, vr: u8);
    fn setsound(&mut self, vr: u8);
    fn addi(&mut self, vr: u8);
    fn spritei(&mut self, vr: u8);
    fn bcd(&mut self, vr: u8);
    fn push(&mut self, vr: u8);
    fn pop(&mut self, vr: u8);

    fn decode(&mut self, addr: u16, msb: u8, lsb: u8) {
        let opcode: u16 = (msb as u16) << 8 | lsb as u16; 
        self.before(opcode, addr);
        match msb >> 4 {
            0x0 => match lsb {
                0xE0 => self.clear(),
                0xEE => self.ret(),
                _    => self.unknown(opcode, addr),
            },
            0x1 => self.jmp(utils::calc_addr(msb, lsb)),
            0x2 => self.call(utils::calc_addr(msb, lsb)),
            0x3 => self.jeq(msb & 0x0F, lsb),
            0x4 => self.jneq(msb & 0x0F, lsb),
            0x5 => self.jeqr(msb & 0x0F, lsb),
            0x6 => self.set(msb & 0x0F, lsb),
            0x7 => self.add(msb & 0x0F, lsb),
            0x8 => {
                let reg1 = msb & 0x0F;
                let reg2 = lsb >> 4;
                match lsb & 0x0F {
                    0x0 => self.setr(reg1, reg2),
                    0x1 => self.or(reg1, reg2),
                    0x2 => self.and(reg1, reg2),
                    0x3 => self.xor(reg1, reg2),
                    0x4 => self.addr(reg1, reg2),
                    0x5 => self.sub(reg1, reg2),
                    0x6 => self.shr(reg1),
                    0x7 => self.subb(reg1, reg2),
                    0xe => self.shl(reg1),
                    _   => self.unknown(opcode, addr),
                }
            },
            0x9 => self.jneqr(msb & 0xF, lsb >> 4),
            0xA => self.seti(utils::calc_addr(msb, lsb)),
            0xB => self.jmpv0(utils::calc_addr(msb, lsb)),
            0xC => self.rand(msb & 0xF, lsb),
            0xD => self.draw(msb & 0xF, lsb >> 4, lsb & 0xF),
            0xE => match lsb {
                0x9E => self.jkey(msb & 0xF),
                0xA1 => self.jnkey(msb & 0xF),
                _    => self.unknown(opcode, addr),
            },
            0xF => match lsb {
                0x07 => self.getdelay(msb & 0xF),
                0x0A => self.waitkey(msb & 0xF),
                0x15 => self.setdelay(msb & 0xF),
                0x18 => self.setsound(msb & 0xF),
                0x1E => self.addi(msb & 0xF),
                0x29 => self.spritei(msb & 0xF),
                0x33 => self.bcd(msb & 0xF),
                0x55 => self.push(msb & 0xF),
                0x65 => self.pop(msb & 0xF),
                _    => self.unknown(opcode, addr),
            },
            _   => self.unknown(opcode, addr),
        }
    }
}