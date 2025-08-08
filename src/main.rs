mod decoder;

use std::io;

use crate::decoder::{
    Memory, SegmentedAccess,
    disasm_8086,
};

fn main() -> io::Result<()> {
    //let args: Vec<String> = env::args().collect();
    let mut memory = Memory::new();

    let filename = "listing_0038_many_register_mov";
    match memory.load_from_file(filename, 0) {
        Ok(bytes_read) => {
            println!("; {} disassembly:", filename);
            println!("bits 16");
            disasm_8086(&memory, bytes_read, SegmentedAccess::default())?;
        }
        Err(e) => {
            eprintln!("ERROR: Unable to open {}: {}", filename, e);
        }
    }

    Ok(())
}

