use decoder;
use std::fs;
use std::io;

pub struct Disassembler {
    memory: [u8; 4096],
    pub source: String
}

impl Disassembler {
    fn new() -> Disassembler {
        Disassembler {
            memory: [0; 4096],
            source : String::from("")
        }
    }

    fn load_program(&mut self, program: &Vec<u8>, address: Option<usize>) {
        let offset: usize = address.unwrap_or(0x200);
        for (i, c) in program.iter().enumerate() {
            self.memory[i + offset] = *c;
        }
    }
    // TODO: Check file size
    pub fn load_file(filename: &str) -> io::Result<Disassembler> {
        let content = fs::read(filename)?;
        let mut ans = Disassembler::new();
        ans.load_program(&content, None);
        Ok(ans)
    }

//    pub fn load_string(source: String) -> io::Result<Disassembler> {
//        let mut ans = Disassembler::new();
//        ans.load_program(source.as), None);
//        Ok(ans)
//    }

}

impl decoder::Decoder for Disassembler {
    // ----
    fn before(&mut self, opcode: u16, addr: u16) {
        let msb = opcode >> 8;
        let lsb = opcode & 0xFF;
        self.source.extend(format!("{:04x}: {:02x} {:02x} ", addr, msb, lsb).chars());
    }
    // ----
    fn unknown(&mut self, opcode: u16, addr: u16) {
        self.source.extend(format!("Unknown opcode addr: {:#x}, op: {:#x}\n", addr, opcode).chars());
    }
    // 0x00e0
    fn clear(&mut self, ) {
        self.source.extend(format!("CLS\n").chars());
    }
    // 0x00ee
    fn ret(&mut self, ) {
        self.source.extend(format!("RET\n").chars());
    }
    // 0x1
    fn jmp(&mut self, address: u16) {
        self.source.extend(format!("JP {:#x}\n", address).chars());
    }
    // 0x2
    fn call(&mut self, address: u16) {
        self.source.extend(format!("CALL {:#x}\n", address).chars());
    }
    // 0x3
    fn jeq(&mut self, vr: u8, value: u8) {
        self.source.extend(format!("SE {:#x}, {:#x}\n", vr, value).chars());
    }
    // 0x4
    fn jneq(&mut self, vr: u8, value: u8) {
        self.source.extend(format!("SNE {:#x}, {:#x}\n", vr, value).chars());
    }
    // 0x5
    fn jeqr(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("SE {:#x}, {:#x}\n", vr, vy).chars());
    }
    // 0x6
    fn set(&mut self, vr: u8, value: u8) {
        self.source.extend(format!("LD {:#x}, {:#x}\n", vr, value).chars());
	}
    // 0x7
    fn add(&mut self, vr: u8, value: u8) {
        self.source.extend(format!("ADD {:#x}, {:#x}\n", vr, value).chars());
	}
    // 0x8xx0
    fn setr(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("LD {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx1
    fn or(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("OR {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx2
    fn and(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("AND {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx3
    fn xor(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("XOR {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx4
    fn addr(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("ADD {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx5
    fn sub(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("SUB {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xx6
    fn shr(&mut self, vr: u8) {
        self.source.extend(format!("SHR {:#x}\n", vr).chars());
	}
    // 0x8xx7
    fn subb(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("SUBN {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0x8xxE
    fn shl(&mut self, vr: u8) {
        self.source.extend(format!("SHL {:#x}\n", vr).chars());
	}
    // 0x9
    fn jneqr(&mut self, vr: u8, vy: u8) {
        self.source.extend(format!("SNE {:#x}, {:#x}\n", vr, vy).chars());
	}
    // 0xA
    fn seti(&mut self, value: u16) {
        self.source.extend(format!("LD I, {:#x}\n", value).chars());
	}
    // 0xB
    fn jmpv0(&mut self, address: u16) {
        self.source.extend(format!("JP V0, {:#x}\n", address).chars());
	}
    // 0xC
    fn rand(&mut self, vr: u8, value: u8) {
        self.source.extend(format!("RND {:#x}, {:#x}\n", vr, value).chars());
	}
    // 0xD
    fn draw(&mut self, vr: u8, vy: u8, value: u8) {
        self.source.extend(format!("DRW {:#x}, {:#x}, {:#x}\n", vr, value, value).chars());
	}
    // 0xEx9E
    fn jkey(&mut self, vr: u8) {
        self.source.extend(format!("SKP {:#x}\n", vr).chars());
	}
    // 0xExA1
    fn jnkey(&mut self, vr: u8) {
        self.source.extend(format!("SKNP {:#x}\n", vr).chars());
	}
    // 0xFx07
    fn getdelay(&mut self, vr: u8) {
        self.source.extend(format!("LD {:#x}, DT\n", vr).chars());
	}
    // 0xFx0A
    fn waitkey(&mut self, vr: u8) {
        self.source.extend(format!("LD {:#x}, K\n", vr).chars());
	}
    // 0xFx15
    fn setdelay(&mut self, vr: u8) {
        self.source.extend(format!("LD DT, {:#x}\n", vr).chars());
	}
    // 0xFx18
    fn setsound(&mut self, vr: u8) {
        self.source.extend(format!("LD ST, {:#x}\n", vr).chars());
	}
    // 0xFx1E
    fn addi(&mut self, vr: u8) {
        self.source.extend(format!("ADD I, {:#x}\n", vr).chars());
	}
    // 0xFx29
    fn spritei(&mut self, vr: u8) {
        self.source.extend(format!("LD F, {:#x}\n", vr).chars());
	}
    // 0xFx33
    fn bcd(&mut self, vr: u8) {
        self.source.extend(format!("LD B, {:#x}\n", vr).chars());
	}
    // 0xFx55
    fn push(&mut self, vr: u8) {
        self.source.extend(format!("LD [I], {:#x}\n", vr).chars());
	}
    // 0xFx65
    fn pop(&mut self, vr: u8) {
        self.source.extend(format!("LD {:#x}, [I]\n", vr).chars());
	}
}

pub trait Executable: decoder::Decoder {
    fn execute(&mut self, addr: u16);

    fn disassembly(&mut self);
}

use decoder::Decoder;

impl Executable for Disassembler {
    fn execute(&mut self, addr: u16) {
        self.decode(addr, self.memory[addr as usize], self.memory[(addr + 1) as usize]);
    }

    fn disassembly(&mut self) {
        self.source = String::from("");
        for i in (0x0200..0xFFE).step_by(2) {
            self.execute(i );
        }
    }

}