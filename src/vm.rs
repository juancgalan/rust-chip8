use decoder;


pub struct CPU {
    v: [u8; 16],
    i: u16,
    sound_timer: u8,
    delay_timer: u8,
    pc: u16,
    sp: u8,
    memory: [u8; 4096]
}

impl CPU {
    fn new() -> CPU {
        CPU {
            v: [0x0; 16],
            i: 0x200,
            sound_timer: 0x0,
            delay_timer: 0x0,
            pc: 0x0,
            sp: 0x0,
            memory: [0; 4096]
        }
    }

    fn load_program(&mut self, program: &[u8], address: Option<u16>) -> &CPU {
        let offset = address.unwrap_or(0x200);
        for (i, c) in program.iter().enumerate() {
            self.memory[i] = *c;
        }
        self
    }
}

impl decoder::Decoder for CPU {
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