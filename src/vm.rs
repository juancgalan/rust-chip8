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
    fn before(&mut self, opcode: u16, addr: u16) { unimplemented!(); }
    fn unknown(&mut self, opcode: u16, addr: u16) { unimplemented!(); }
    fn clear(&mut self, ) { unimplemented!(); }
    fn ret(&mut self, ) { unimplemented!(); }
    fn jmp(&mut self, address: u16) { unimplemented!(); }
    fn call(&mut self, address: u16) { unimplemented!(); }
    fn jeq(&mut self, vr: u8, value: u8) { unimplemented!(); }
    fn jneq(&mut self, vr: u8, value: u8) { unimplemented!(); }
    fn jeqr(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn set(&mut self, vr: u8, value: u8) { unimplemented!(); }
    fn add(&mut self, vr: u8, value: u8) { unimplemented!(); }
    fn setr(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn or(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn and(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn xor(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn addr(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn sub(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn shr(&mut self, vr: u8) { unimplemented!(); }
    fn subb(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn shl(&mut self, vr: u8) { unimplemented!(); }
    fn jneqr(&mut self, vr: u8, vy: u8) { unimplemented!(); }
    fn seti(&mut self, value: u16) { unimplemented!(); }
    fn jmpv0(&mut self, address: u16) { unimplemented!(); }
    fn rand(&mut self, vr: u8, value: u8) { unimplemented!(); }
    fn draw(&mut self, vr: u8, vy: u8, value: u8) { unimplemented!(); }
    fn jkey(&mut self, vr: u8) { unimplemented!(); }
    fn jnkey(&mut self, vr: u8) { unimplemented!(); }
    fn getdelay(&mut self, vr: u8) { unimplemented!(); }
    fn waitkey(&mut self, vr: u8) { unimplemented!(); }
    fn setdelay(&mut self, vr: u8) { unimplemented!(); }
    fn setsound(&mut self, vr: u8) { unimplemented!(); }
    fn addi(&mut self, vr: u8) { unimplemented!(); }
    fn spritei(&mut self, vr: u8) { unimplemented!(); }
    fn bcd(&mut self, vr: u8) { unimplemented!(); }
    fn push(&mut self, vr: u8) { unimplemented!(); }
    fn pop(&mut self, vr: u8) { unimplemented!(); }
}