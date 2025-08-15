use crate::{
    instruction_formats::{
        get_instruction_formats, InstructionBitsUsage, InstructionFormat, OperationType
    }, memory::{Memory, SegmentedAccess}
};

#[derive(Debug, Clone, Copy)]
pub struct InstructionFlag;

impl InstructionFlag {
    pub const LOCK: u32 = 1 << 0;
    pub const REP: u32 = 1 << 1;
    pub const SEGMENT: u32 = 1 << 2;
    pub const WIDE: u32 = 1 << 3;
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RegisterIndex {
    None = 0,
    A,
    B,
    C,
    D,
    Sp,
    Bp,
    Si,
    Di,
    Es,
    Cs,
    Ss,
    Ds,
    Ip,
    Flags,
}

impl RegisterIndex {
    pub fn get_name(&self, offset: u8, count: u8) -> &'static str {
        match self {
            RegisterIndex::None => "",
            RegisterIndex::A => match (count, offset & 1) {
                (2, _) => "ax",
                (1, 0) => "al",
                (1, 1) => "ah",
                _ => "ax",
            },
            RegisterIndex::B => match (count, offset & 1) {
                (2, _) => "bx",
                (1, 0) => "bl",
                (1, 1) => "bh",
                _ => "bx",
            },
            RegisterIndex::C => match (count, offset & 1) {
                (2, _) => "cx",
                (1, 0) => "cl",
                (1, 1) => "ch",
                _ => "cx",
            },
            RegisterIndex::D => match (count, offset & 1) {
                (2, _) => "dx",
                (1, 0) => "dl",
                (1, 1) => "dh",
                _ => "dx",
            },
            RegisterIndex::Sp => "sp",
            RegisterIndex::Bp => "bp",
            RegisterIndex::Si => "si",
            RegisterIndex::Di => "di",
            RegisterIndex::Es => "es",
            RegisterIndex::Cs => "cs",
            RegisterIndex::Ss => "ss",
            RegisterIndex::Ds => "ds",
            RegisterIndex::Ip => "ip",
            RegisterIndex::Flags => "flags",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum EffectiveAddressBase {
    Direct = 0,
    BxSi,
    BxDi,
    BpSi,
    BpDi,
    Si,
    Di,
    Bp,
    Bx,
}

impl EffectiveAddressBase {
    pub fn expression(&self) -> &'static str {
        match self {
            EffectiveAddressBase::Direct => "",
            EffectiveAddressBase::BxSi => "bx+si",
            EffectiveAddressBase::BxDi => "bx+di",
            EffectiveAddressBase::BpSi => "bp+si",
            EffectiveAddressBase::BpDi => "bp+di",
            EffectiveAddressBase::Si => "si",
            EffectiveAddressBase::Di => "di",  
            EffectiveAddressBase::Bp => "bp",
            EffectiveAddressBase::Bx => "bx",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EffectiveAddressExpression {
    pub segment: RegisterIndex,
    pub base: EffectiveAddressBase,
    pub displacement: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterAccess {
    pub index: RegisterIndex,
    pub offset: u8,
    pub count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    None,
    Register(RegisterAccess),
    Memory(EffectiveAddressExpression),
    ImmediateU32(u32),
    ImmediateS32(i32),
    RelativeImmediate(i32),
}

impl Default for Operand {
    fn default() -> Self {
        Operand::None
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub address: u32,
    pub size: u32,
    pub op: OperationType,
    pub flags: u32,
    pub operands: [Operand; 2],
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            address: 0,
            size: 0,
            op: OperationType::None,
            flags: 0,
            operands: [Operand::default(); 2],
        }
    }
}

pub struct DisasmContext {
    pub default_segment: RegisterIndex,
    pub additional_flags: u32,
}

impl DisasmContext {
    pub fn new() -> Self {
        Self {
            default_segment: RegisterIndex::Ds,
            additional_flags: 0,
        }
    }

    pub fn update(&mut self, instruction: &Instruction) {
        match instruction.op {
            OperationType::Lock => {
                self.additional_flags |= InstructionFlag::LOCK;
            }
            OperationType::Rep => {
                self.additional_flags |= InstructionFlag::REP;
            }
            OperationType::Segment => {
                self.additional_flags |= InstructionFlag::SEGMENT;
                self.default_segment = match instruction.operands[1] {
                    Operand::Register(reg) => reg.index,
                    _ => unreachable!(),
                };
            }
            _ => {
                self.additional_flags = 0;
                self.default_segment = RegisterIndex::Ds;
            }
        }
    }
}

fn get_reg_operand(intel_reg_index: u32, wide: bool) -> Operand {
    let reg_table = [
        [(RegisterIndex::A, 0, 1), (RegisterIndex::A, 0, 2)],
        [(RegisterIndex::C, 0, 1), (RegisterIndex::C, 0, 2)],
        [(RegisterIndex::D, 0, 1), (RegisterIndex::D, 0, 2)],
        [(RegisterIndex::B, 0, 1), (RegisterIndex::B, 0, 2)],
        [(RegisterIndex::A, 1, 1), (RegisterIndex::Sp, 0, 2)],
        [(RegisterIndex::C, 1, 1), (RegisterIndex::Bp, 0, 2)],
        [(RegisterIndex::D, 1, 1), (RegisterIndex::Si, 0, 2)],
        [(RegisterIndex::B, 1, 1), (RegisterIndex::Di, 0, 2)],
    ];

    let (index, offset, count) = reg_table[(intel_reg_index & 0x7) as usize][if wide { 1 } else { 0 }];
 
    Operand::Register(RegisterAccess {
        index,
        offset,
        count,
    })
}

fn parse_data_value(memory: &Memory, access: &mut SegmentedAccess, exists: bool, wide: bool, sign_extended: bool) -> u32 {
    if !exists {
        return 0;
    }

    let result = if wide {
        let d0 = memory.read(access.get_absolute_address(0));
        let d1 = memory.read(access.get_absolute_address(1));
        access.segment_offset += 2;
        ((d1 as u32) << 8) | (d0 as u32)
    } else {
        let d = memory.read(access.get_absolute_address(0));
        access.segment_offset += 1;
        if sign_extended {
            (d as i8) as u32
        } else {
            d as u32
        }
    };

    result
}

fn try_decode(context: &DisasmContext, format: &InstructionFormat, memory: &Memory, mut at: SegmentedAccess) -> Option<Instruction> {
    let mut instruction = Instruction::default();
    let mut bits = [0u32; 18];
    let mut has_bits = 0u32;
    let mut valid = true;

    let starting_address = at.get_absolute_address(0);
 
    let mut bits_pending_count = 0u8;
    let mut bits_pending = 0u8;

    for test_bits in &format.bits {
        if test_bits.usage == InstructionBitsUsage::Literal && test_bits.bit_count == 0 {
            break;
        }

        let mut read_bits = test_bits.value as u32;
        
        if test_bits.bit_count != 0 {
            if bits_pending_count == 0 {
                bits_pending_count = 8;
                bits_pending = memory.read(at.get_absolute_address(0));
                at.segment_offset += 1;
            }

            if test_bits.bit_count > bits_pending_count {
                valid = false;
                break;
            }

            bits_pending_count -= test_bits.bit_count;
            read_bits = bits_pending as u32;
            read_bits >>= bits_pending_count;
            read_bits &= !(0xff << test_bits.bit_count);
        }

        if test_bits.usage == InstructionBitsUsage::Literal {
            valid = valid && (read_bits == test_bits.value as u32);
        } else {
            let usage_index = test_bits.usage as usize;
            bits[usage_index] |= read_bits << test_bits.shift;
            has_bits |= 1 << usage_index;
        }
    }

    if !valid {
        return None;
    }

    // Decode the instruction using the parsed bits
    let mod_val = bits[InstructionBitsUsage::Mod as usize];
    let rm = bits[InstructionBitsUsage::Rm as usize];
    let w = bits[InstructionBitsUsage::W as usize] != 0;
    let s = bits[InstructionBitsUsage::S as usize] != 0;
    let d = bits[InstructionBitsUsage::D as usize] != 0;

    let has_direct_address = (mod_val == 0b00) && (rm == 0b110);
    let has_displacement = bits[InstructionBitsUsage::HasDisp as usize] != 0 || 
                          mod_val == 0b10 || mod_val == 0b01 || has_direct_address;
    let displacement_is_w = bits[InstructionBitsUsage::DispAlwaysW as usize] != 0 || 
                           mod_val == 0b10 || has_direct_address;
    let data_is_w = bits[InstructionBitsUsage::WMakesDataW as usize] != 0 && !s && w;

    bits[InstructionBitsUsage::Disp as usize] = parse_data_value(memory, &mut at, has_displacement, displacement_is_w, !displacement_is_w);
    bits[InstructionBitsUsage::Data as usize] = parse_data_value(memory, &mut at, bits[InstructionBitsUsage::HasData as usize] != 0, data_is_w, s);

    instruction.op = format.op;
    instruction.flags = context.additional_flags;
    instruction.address = starting_address;
    instruction.size = at.get_absolute_address(0) - starting_address;
    
    if w {
        instruction.flags |= InstructionFlag::WIDE;
    }

    let displacement = bits[InstructionBitsUsage::Disp as usize] as i16;

    let reg_operand_index = if d { 0 } else { 1 };
    let mod_operand_index = if d { 1 } else { 0 };

    // Handle segment register
    if (has_bits & (1 << (InstructionBitsUsage::Sr as usize))) != 0 {
        let sr_val = bits[InstructionBitsUsage::Sr as usize] & 0x3;
        instruction.operands[reg_operand_index] = Operand::Register(RegisterAccess {
            index: match sr_val {
                0 => RegisterIndex::Es,
                1 => RegisterIndex::Cs,
                2 => RegisterIndex::Ss,
                3 => RegisterIndex::Ds,
                _ => RegisterIndex::Es,
            },
            offset: 0,
            count: 2,
        });
    }

    // Handle REG field
    if (has_bits & (1 << (InstructionBitsUsage::Reg as usize))) != 0 {
        instruction.operands[reg_operand_index] = get_reg_operand(bits[InstructionBitsUsage::Reg as usize], w);
    }

    // Handle MOD field
    if (has_bits & (1 << (InstructionBitsUsage::Mod as usize))) != 0 {
        if mod_val == 0b11 {
            instruction.operands[mod_operand_index] = get_reg_operand(rm, w || bits[InstructionBitsUsage::RmRegAlwaysW as usize] != 0);
        } else {

            let base = if (mod_val == 0b00) && (rm == 0b110) {
                EffectiveAddressBase::Direct
            } else {
                match rm {
                    0 => EffectiveAddressBase::BxSi,
                    1 => EffectiveAddressBase::BxDi,
                    2 => EffectiveAddressBase::BpSi,
                    3 => EffectiveAddressBase::BpDi,
                    4 => EffectiveAddressBase::Si,
                    5 => EffectiveAddressBase::Di,
                    6 => EffectiveAddressBase::Bp,
                    7 => EffectiveAddressBase::Bx,
                    _ => EffectiveAddressBase::Direct,
                }
            };

            instruction.operands[mod_operand_index] = Operand::Memory(
                EffectiveAddressExpression {
                    segment: context.default_segment,
                    base,
                    displacement: displacement as i32,
                }
            );
        }
    }

    // Handle additional operands
    let last_operand_index = if instruction.operands[0] != Operand::None { 1 } else { 0 };

    if bits[InstructionBitsUsage::RelJmpDisp as usize] != 0 {
        instruction.operands[last_operand_index] = Operand::RelativeImmediate(displacement as i32 + instruction.size as i32);
    }

    if bits[InstructionBitsUsage::HasData as usize] != 0 {
        instruction.operands[last_operand_index] = Operand::ImmediateS32(bits[InstructionBitsUsage::Data as usize] as i32);
    }

    if (has_bits & (1 << (InstructionBitsUsage::V as usize))) != 0 {
        if bits[InstructionBitsUsage::V as usize] != 0 {
            instruction.operands[last_operand_index] = Operand::Register(
                RegisterAccess {
                index: RegisterIndex::C,
                offset: 0,
                count: 1,
            });
        } else { 
            instruction.operands[last_operand_index] = Operand::ImmediateS32(1);
        }
    }

    Some(instruction)
}

pub fn decode_instruction(context: &DisasmContext, memory: &Memory, at: &mut SegmentedAccess) -> Instruction {
    let instruction_formats = get_instruction_formats();
    
    for format in &instruction_formats {
        if let Some(instruction) = try_decode(context, format, memory, *at) {
            at.segment_offset += instruction.size as u16;
            return instruction;
        }
    }
    
    Instruction::default()
}

