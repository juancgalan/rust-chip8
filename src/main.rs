mod utils;
mod decoder;
mod vm;
mod disassembly;

use disassembly::Disassembler;
use disassembly::Executable;

fn main() {
    let mut vmachine = Disassembler::load_file("roms//15puzzle.rom").unwrap();
    vmachine.disassembly();
    println!("{}", vmachine.source);
}
