use std::io;

use crate::{
    memory::Memory,
    register::RegisterFile,
    instruction_formats::OperationType,
    decoder::{Instruction, Operand},
};

pub fn execute_instruction(
    instruction: &Instruction,
    _memory: &Memory,
    reg_file: &mut RegisterFile,
) -> io::Result<()> {
    match instruction.op {
        OperationType::Mov => {
            if let Operand::Register(dst) = &instruction.operands[0] {
                let val = match &instruction.operands[1] {
                    Operand::Immediate(v) => (*v & 0xFFFF) as u16,
                    Operand::Register(src) => reg_file.get_register_value(src),
                    Operand::Memory(_) => 0, // TODO: memory read
                    _ => 0,
                };
                reg_file.update_register(dst, val);
            }
        }

        OperationType::Add => {
            if let Operand::Register(dst) = &instruction.operands[0] {
                let val1 = reg_file.get_register_value(dst);
                let val2 = match &instruction.operands[1] {
                    Operand::Immediate(v) => (*v & 0xFFFF) as u16,
                    Operand::Register(src) => reg_file.get_register_value(src),
                    Operand::Memory(_) => 0,
                    _ => 0,
                };
                let res = reg_file.alu_add(val1, val2);
                reg_file.update_register(dst, res);
            }
        }

        OperationType::Sub => {
            if let Operand::Register(dst) = &instruction.operands[0] {
                let val1 = reg_file.get_register_value(dst);
                let val2 = match &instruction.operands[1] {
                    Operand::Immediate(v) => (*v & 0xFFFF) as u16,
                    Operand::Register(src) => reg_file.get_register_value(src),
                    Operand::Memory(_) => 0,
                    _ => 0,
                };
                let res = reg_file.alu_sub(val1, val2);
                reg_file.update_register(dst, res);
            }
        }

        OperationType::Cmp => {
            if let Operand::Register(dst) = &instruction.operands[0] {
                let val1 = reg_file.get_register_value(dst);
                let val2 = match &instruction.operands[1] {
                    Operand::Immediate(v) => (*v & 0xFFFF) as u16,
                    Operand::Register(src) => reg_file.get_register_value(src),
                    Operand::Memory(_) => 0,
                    _ => 0,
                };
                // Compare is just SUB without writing result
                let _ = reg_file.alu_sub(val1, val2);
            }
        }

        _ => {}
    }
    Ok(())
}
