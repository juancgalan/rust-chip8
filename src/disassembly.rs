use decoder;
use std::fs;
use std::io;

pub struct Disassembler {
    memory: [u8; 4096]
}

impl Disassembler {
    fn load_program(&mut self, program: &[u8], address: Option<u16>) -> &Disassembler {
        let offset = address.unwrap_or(0x200);
        for (i, c) in program.iter().enumerate() {
            self.memory[i] = *c;
        }
        self
    }

    fn load_file(&self, filename: &str) -> io::Result<&Disassembler> {
        let content = fs::read_to_string(filename)?;
        Ok(self.load_program(content.as_bytes(), None))
    }

    
}

impl decoder::Decoder for Disassembler {
    fn before(&self, opcode: u16, addr: u16) { unimplemented!(); }
    fn unknown(&self, opcode: u16, addr: u16) { unimplemented!(); }
    fn clear(&self, ) { unimplemented!(); }
    fn ret(&self, ) { unimplemented!(); }
    fn jmp(&self, address: u16) { unimplemented!(); }
    fn call(&self, address: u16) { unimplemented!(); }
    fn jeq(&self, vr: u8, value: u8) { unimplemented!(); }
    fn jneq(&self, vr: u8, value: u8) { unimplemented!(); }
    fn jeqr(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn set(&self, vr: u8, value: u8) { unimplemented!(); }
    fn add(&self, vr: u8, value: u8) { unimplemented!(); }
    fn setr(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn or(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn and(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn xor(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn addr(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn sub(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn shr(&self, vr: u8) { unimplemented!(); }
    fn subb(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn shl(&self, vr: u8) { unimplemented!(); }
    fn jneqr(&self, vr: u8, vy: u8) { unimplemented!(); }
    fn seti(&self, value: u16) { unimplemented!(); }
    fn jmpv0(&self, address: u16) { unimplemented!(); }
    fn rand(&self, vr: u8, value: u8) { unimplemented!(); }
    fn draw(&self, vr: u8, vy: u8, value: u8) { unimplemented!(); }
    fn jkey(&self, vr: u8) { unimplemented!(); }
    fn jnkey(&self, vr: u8) { unimplemented!(); }
    fn getdelay(&self, vr: u8) { unimplemented!(); }
    fn waitkey(&self, vr: u8) { unimplemented!(); }
    fn setdelay(&self, vr: u8) { unimplemented!(); }
    fn setsound(&self, vr: u8) { unimplemented!(); }
    fn addi(&self, vr: u8) { unimplemented!(); }
    fn spritei(&self, vr: u8) { unimplemented!(); }
    fn bcd(&self, vr: u8) { unimplemented!(); }
    fn push(&self, vr: u8) { unimplemented!(); }
    fn pop(&self, vr: u8) { unimplemented!(); }
}