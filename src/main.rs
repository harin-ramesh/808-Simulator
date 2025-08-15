use std::io::{self, Write};

use sim86::{
    decoder::{
        Instruction, InstructionFlag, Operand,
        DisasmContext, decode_instruction,
    },
    memory::{Memory, SegmentedAccess},
    instruction_formats::OperationType,
};

pub fn is_printable(instruction: &Instruction) -> bool {
    !matches!(instruction.op, OperationType::Lock | OperationType::Rep | OperationType::Segment)
}

pub fn print_instruction(instruction: &Instruction, output: &mut dyn Write) -> io::Result<()> {
    let flags = instruction.flags;
    let w = (flags & InstructionFlag::WIDE) != 0;

    if (flags & InstructionFlag::LOCK) != 0 {
        if instruction.op == OperationType::Xchg {
            write!(output, "lock ")?;
        } else {
            write!(output, "lock ")?;
        }
    }

    let mut mnemonic_suffix = "";
    if (flags & InstructionFlag::REP) != 0 {
        write!(output, "rep ")?;
        mnemonic_suffix = if w { "w" } else { "b" };
    }

    write!(output, "{}{} ", instruction.op.mnemonic(), mnemonic_suffix)?;

    let mut separator = "";
    for operand in &instruction.operands {
        if *operand != Operand::None {
            write!(output, "{}", separator)?;
            separator = ", ";

            match operand {
                Operand::None => {}
                Operand::Register(reg) => {
                    write!(output, "{}", reg.index.get_name(reg.offset, reg.count))?;
                }
                Operand::Memory(address) => {
                    // Word/byte size hint logic
                    if !matches!(instruction.operands[0], Operand::Register(_))
                        && !matches!(instruction.operands[1], Operand::Register(_))
                        && !matches!(instruction.operands[0], Operand::RelativeImmediate(_))
                        && !matches!(instruction.operands[1], Operand::RelativeImmediate(_))
                    {
                        write!(output, "{} ", if w { "word" } else { "byte" })?;
                    }

                    // Segment prefix
                    if (flags & InstructionFlag::SEGMENT) != 0 {
                        write!(output, "{}:", address.segment.get_name(0, 2))?;
                    }

                    // Address formatting
                    write!(output, "[{}", address.base.expression())?;
                    if address.displacement != 0 {
                        write!(output, "{:+}", address.displacement)?;
                    }
                    write!(output, "]")?;
                }
                Operand::ImmediateU32(val) => {
                    write!(output, "{}", val)?;
                }
                Operand::ImmediateS32(val) => {
                    write!(output, "{}", val)?;
                }
                Operand::RelativeImmediate(offset) => {
                    write!(output, "${:+}", offset)?;
                }
            }
        }
    }

    Ok(())
}

pub fn disasm_8086(memory: &Memory, disasm_byte_count: u32, disasm_start: SegmentedAccess) -> io::Result<()> {
    let mut at = disasm_start;
    let mut context = DisasmContext::new();
    let mut count = disasm_byte_count;

    while count > 0 {
        let instruction = decode_instruction(&context, memory, &mut at);
        
        if instruction.op == OperationType::None {
            eprintln!("ERROR: Unrecognized binary in instruction stream.");
            break;
        }

        if count >= instruction.size {
            count -= instruction.size;
        } else {
            // eprintln!("ERROR: Instruction extends outside disassembly region");
            break;
        }

        context.update(&instruction);
        
        if is_printable(&instruction) {
            print_instruction(&instruction, &mut io::stdout())?;
            println!();
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let mut memory = Memory::new();

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

