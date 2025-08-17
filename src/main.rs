use std::io::{self, Write};

use sim86::{
    decoder::{
        decode_instruction, DisasmContext, Instruction, InstructionFlag, Operand,
    }, 
    instruction_formats::OperationType, memory::{Memory, SegmentedAccess},
    register::{RegisterFile, RegisterAccess, RegisterIndex},
    execution_unit::execute_instruction,
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
                Operand::Immediate(val) => {
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
    let mut register_file = RegisterFile::new();

    let starting_address = disasm_start.get_absolute_address(0);
    register_file.update_ip(starting_address as u16);

    loop {
        at.segment_offset = register_file.get_register_value(&RegisterAccess {index: RegisterIndex::IP, offset: 0, count: 2});

        let instruction = decode_instruction(&context, memory, &mut at);

        if instruction.op == OperationType::None {
            eprintln!("ERROR: Unrecognized binary in instruction stream.");
            break;
        }

        register_file.update_ip(instruction.size as u16);
        let current_address = at.get_absolute_address(0);

        if current_address - starting_address >= disasm_byte_count {
            break;
        }

        execute_instruction(&instruction, memory, &mut register_file)?;
        context.update(&instruction);

        if is_printable(&instruction) {
            print_instruction(&instruction, &mut io::stdout())?;
            println!();
        }
    }

    register_file.print_state();

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

