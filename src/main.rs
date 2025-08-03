use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

// Type aliases matching the original C++ types
type U8 = u8;
type U16 = u16;
type U32 = u32;
type S16 = i16;
type S32 = i32;
type B32 = bool;

// Constants
const MEMORY_SIZE: usize = 1024 * 1024;
const MEMORY_ACCESS_MASK: u32 = 0xfffff;

// Enums
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum OperationType {
    None = 0,
    Mov,
    Push,
    Pop,
    Xchg,
    In,
    Out,
    Xlat,
    Lea,
    Lds,
    Les,
    Lahf,
    Sahf,
    Pushf,
    Popf,
    Add,
    Adc,
    Inc,
    Aaa,
    Daa,
    Sub,
    Sbb,
    Dec,
    Neg,
    Cmp,
    Aas,
    Das,
    Mul,
    Imul,
    Aam,
    Div,
    Idiv,
    Aad,
    Cbw,
    Cwd,
    Not,
    Shl,
    Sal,
    Shr,
    Sar,
    Rol,
    Ror,
    Rcl,
    Rcr,
    And,
    Test,
    Or,
    Xor,
    Rep,
    Call,
    Jmp,
    Ret,
    Retf,
    Je,
    Jl,
    Jle,
    Jb,
    Jbe,
    Jp,
    Jo,
    Js,
    Jne,
    Jnl,
    Jg,
    Jnb,
    Ja,
    Jnp,
    Jno,
    Jns,
    Loop,
    Loopz,
    Loopnz,
    Jcxz,
    Int,
    Int3,
    Into,
    Iret,
    Clc,
    Cmc,
    Stc,
    Cld,
    Std,
    Cli,
    Sti,
    Hlt,
    Wait,
    Esc,
    Lock,
    Segment,
}

impl OperationType {
    fn mnemonic(&self) -> &'static str {
        match self {
            OperationType::None => "",
            OperationType::Mov => "mov",
            OperationType::Push => "push",
            OperationType::Pop => "pop",
            OperationType::Xchg => "xchg",
            OperationType::In => "in",
            OperationType::Out => "out",
            OperationType::Xlat => "xlat",
            OperationType::Lea => "lea",
            OperationType::Lds => "lds",
            OperationType::Les => "les",
            OperationType::Lahf => "lahf",
            OperationType::Sahf => "sahf",
            OperationType::Pushf => "pushf",
            OperationType::Popf => "popf",
            OperationType::Add => "add",
            OperationType::Adc => "adc",
            OperationType::Inc => "inc",
            OperationType::Aaa => "aaa",
            OperationType::Daa => "daa",
            OperationType::Sub => "sub",
            OperationType::Sbb => "sbb",
            OperationType::Dec => "dec",
            OperationType::Neg => "neg",
            OperationType::Cmp => "cmp",
            OperationType::Aas => "aas",
            OperationType::Das => "das",
            OperationType::Mul => "mul",
            OperationType::Imul => "imul",
            OperationType::Aam => "aam",
            OperationType::Div => "div",
            OperationType::Idiv => "idiv",
            OperationType::Aad => "aad",
            OperationType::Cbw => "cbw",
            OperationType::Cwd => "cwd",
            OperationType::Not => "not",
            OperationType::Shl => "shl",
            OperationType::Sal => "sal",
            OperationType::Shr => "shr",
            OperationType::Sar => "sar",
            OperationType::Rol => "rol",
            OperationType::Ror => "ror",
            OperationType::Rcl => "rcl",
            OperationType::Rcr => "rcr",
            OperationType::And => "and",
            OperationType::Test => "test",
            OperationType::Or => "or",
            OperationType::Xor => "xor",
            OperationType::Rep => "rep",
            OperationType::Call => "call",
            OperationType::Jmp => "jmp",
            OperationType::Ret => "ret",
            OperationType::Retf => "retf",
            OperationType::Je => "je",
            OperationType::Jl => "jl",
            OperationType::Jle => "jle",
            OperationType::Jb => "jb",
            OperationType::Jbe => "jbe",
            OperationType::Jp => "jp",
            OperationType::Jo => "jo",
            OperationType::Js => "js",
            OperationType::Jne => "jne",
            OperationType::Jnl => "jnl",
            OperationType::Jg => "jg",
            OperationType::Jnb => "jnb",
            OperationType::Ja => "ja",
            OperationType::Jnp => "jnp",
            OperationType::Jno => "jno",
            OperationType::Jns => "jns",
            OperationType::Loop => "loop",
            OperationType::Loopz => "loopz",
            OperationType::Loopnz => "loopnz",
            OperationType::Jcxz => "jcxz",
            OperationType::Int => "int",
            OperationType::Int3 => "int3",
            OperationType::Into => "into",
            OperationType::Iret => "iret",
            OperationType::Clc => "clc",
            OperationType::Cmc => "cmc",
            OperationType::Stc => "stc",
            OperationType::Cld => "cld",
            OperationType::Std => "std",
            OperationType::Cli => "cli",
            OperationType::Sti => "sti",
            OperationType::Hlt => "hlt",
            OperationType::Wait => "wait",
            OperationType::Esc => "esc",
            OperationType::Lock => "lock",
            OperationType::Segment => "segment",
        }
    }
}

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
    fn get_name(&self, offset: u8, count: u8) -> &'static str {
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
    fn expression(&self) -> &'static str {
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

#[derive(Debug, Clone, Copy)]
pub struct EffectiveAddressExpression {
    pub segment: RegisterIndex,
    pub base: EffectiveAddressBase,
    pub displacement: S32,
}

#[derive(Debug, Clone, Copy)]
pub struct RegisterAccess {
    pub index: RegisterIndex,
    pub offset: U8,
    pub count: U8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandType {
    None,
    Register,
    Memory,
    Immediate,
    RelativeImmediate,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionOperand {
    pub operand_type: OperandType,
    pub address: EffectiveAddressExpression,
    pub register: RegisterAccess,
    pub immediate_u32: U32,
    pub immediate_s32: S32,
}

impl Default for InstructionOperand {
    fn default() -> Self {
        Self {
            operand_type: OperandType::None,
            address: EffectiveAddressExpression {
                segment: RegisterIndex::None,
                base: EffectiveAddressBase::Direct,
                displacement: 0,
            },
            register: RegisterAccess {
                index: RegisterIndex::None,
                offset: 0,
                count: 0,
            },
            immediate_u32: 0,
            immediate_s32: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub address: U32,
    pub size: U32,
    pub op: OperationType,
    pub flags: U32,
    pub operands: [InstructionOperand; 2],
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            address: 0,
            size: 0,
            op: OperationType::None,
            flags: 0,
            operands: [InstructionOperand::default(); 2],
        }
    }
}

// Memory structure
pub struct Memory {
    pub bytes: [U8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, absolute_address: U32) -> U8 {
        assert!((absolute_address as usize) < self.bytes.len());
        self.bytes[absolute_address as usize]
    }

    pub fn load_from_file(&mut self, filename: &str, at_offset: U32) -> io::Result<U32> {
        if (at_offset as usize) >= self.bytes.len() {
            return Ok(0);
        }

        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        let bytes_read = file.read_to_end(&mut buffer)?;
        
        let available_space = self.bytes.len() - (at_offset as usize);
        let bytes_to_copy = bytes_read.min(available_space);
        
        self.bytes[at_offset as usize..at_offset as usize + bytes_to_copy]
            .copy_from_slice(&buffer[..bytes_to_copy]);
            
        Ok(bytes_to_copy as U32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentedAccess {
    pub segment_base: U16,
    pub segment_offset: U16,
}

impl Default for SegmentedAccess {
    fn default() -> Self {
        Self {
            segment_base: 0,
            segment_offset: 0,
        }
    }
}

impl SegmentedAccess {
    pub fn get_absolute_address(&self, additional_offset: U16) -> U32 {
        let result = (((self.segment_base as U32) << 4) + 
                     (self.segment_offset + additional_offset) as U32) & MEMORY_ACCESS_MASK;
        result
    }
}

// Instruction bits and decoding
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum InstructionBitsUsage {
    Literal,
    Mod,
    Reg,
    Rm,
    Sr,
    Disp,
    Data,
    HasDisp,
    DispAlwaysW,
    HasData,
    WMakesDataW,
    RmRegAlwaysW,
    RelJmpDisp,
    D,
    S,
    W,
    V,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionBits {
    pub usage: InstructionBitsUsage,
    pub bit_count: U8,
    pub shift: U8,
    pub value: U8,
}

#[derive(Debug, Clone)]
pub struct InstructionFormat {
    pub op: OperationType,
    pub bits: Vec<InstructionBits>,
}

pub struct DisasmContext {
    pub default_segment: RegisterIndex,
    pub additional_flags: U32,
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
                self.default_segment = instruction.operands[1].register.index;
            }
            _ => {
                self.additional_flags = 0;
                self.default_segment = RegisterIndex::Ds;
            }
        }
    }
}

// Simple instruction table - in a real implementation, this would be more comprehensive
fn get_instruction_formats() -> Vec<InstructionFormat> {
    vec![
        // MOV register/memory to/from register - 100010dw mod reg r/m
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100010 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        // MOV immediate to register - 1011wreg data
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 4, shift: 0, value: 0b1011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
    ]
}

fn get_reg_operand(intel_reg_index: U32, wide: bool) -> InstructionOperand {
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
    
    InstructionOperand {
        operand_type: OperandType::Register,
        register: RegisterAccess {
            index,
            offset,
            count,
        },
        ..Default::default()
    }
}

fn parse_data_value(memory: &Memory, access: &mut SegmentedAccess, exists: bool, wide: bool, sign_extended: bool) -> U32 {
    if !exists {
        return 0;
    }

    let result = if wide {
        let d0 = memory.read(access.get_absolute_address(0));
        let d1 = memory.read(access.get_absolute_address(1));
        access.segment_offset += 2;
        ((d1 as U32) << 8) | (d0 as U32)
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

fn try_decode(context: &DisasmContext, format: &InstructionFormat, memory: &Memory, mut at: SegmentedAccess) -> Option<Instruction> {
    let mut dest = Instruction::default();
    let mut bits = [0u32; 18]; // Assuming max 18 different bit types
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

    dest.op = format.op;
    dest.flags = context.additional_flags;
    dest.address = starting_address;
    dest.size = at.get_absolute_address(0) - starting_address;
    
    if w {
        dest.flags |= InstructionFlag::WIDE;
    }

    let displacement = bits[InstructionBitsUsage::Disp as usize] as i16;

    let reg_operand_index = if d { 0 } else { 1 };
    let mod_operand_index = if d { 1 } else { 0 };

    // Handle segment register
    if (has_bits & (1 << (InstructionBitsUsage::Sr as usize))) != 0 {
        dest.operands[reg_operand_index].operand_type = OperandType::Register;
        let sr_val = bits[InstructionBitsUsage::Sr as usize] & 0x3;
        dest.operands[reg_operand_index].register = RegisterAccess {
            index: match sr_val {
                0 => RegisterIndex::Es,
                1 => RegisterIndex::Cs,
                2 => RegisterIndex::Ss,
                3 => RegisterIndex::Ds,
                _ => RegisterIndex::Es,
            },
            offset: 0,
            count: 2,
        };
    }

    // Handle REG field
    if (has_bits & (1 << (InstructionBitsUsage::Reg as usize))) != 0 {
        dest.operands[reg_operand_index] = get_reg_operand(bits[InstructionBitsUsage::Reg as usize], w);
    }

    // Handle MOD field
    if (has_bits & (1 << (InstructionBitsUsage::Mod as usize))) != 0 {
        if mod_val == 0b11 {
            dest.operands[mod_operand_index] = get_reg_operand(rm, w || bits[InstructionBitsUsage::RmRegAlwaysW as usize] != 0);
        } else {
            dest.operands[mod_operand_index].operand_type = OperandType::Memory;
            dest.operands[mod_operand_index].address.segment = context.default_segment;
            dest.operands[mod_operand_index].address.displacement = displacement as S32;

            if (mod_val == 0b00) && (rm == 0b110) {
                dest.operands[mod_operand_index].address.base = EffectiveAddressBase::Direct;
            } else {
                dest.operands[mod_operand_index].address.base = match rm {
                    0 => EffectiveAddressBase::BxSi,
                    1 => EffectiveAddressBase::BxDi,
                    2 => EffectiveAddressBase::BpSi,
                    3 => EffectiveAddressBase::BpDi,
                    4 => EffectiveAddressBase::Si,
                    5 => EffectiveAddressBase::Di,
                    6 => EffectiveAddressBase::Bp,
                    7 => EffectiveAddressBase::Bx,
                    _ => EffectiveAddressBase::Direct,
                };
            }
        }
    }

    // Handle additional operands
    let last_operand_index = if dest.operands[0].operand_type != OperandType::None { 1 } else { 0 };

    if bits[InstructionBitsUsage::RelJmpDisp as usize] != 0 {
        dest.operands[last_operand_index].operand_type = OperandType::RelativeImmediate;
        dest.operands[last_operand_index].immediate_s32 = displacement as S32 + dest.size as S32;
    }

    if bits[InstructionBitsUsage::HasData as usize] != 0 {
        dest.operands[last_operand_index].operand_type = OperandType::Immediate;
        dest.operands[last_operand_index].immediate_u32 = bits[InstructionBitsUsage::Data as usize];
    }

    if (has_bits & (1 << (InstructionBitsUsage::V as usize))) != 0 {
        if bits[InstructionBitsUsage::V as usize] != 0 {
            dest.operands[last_operand_index].operand_type = OperandType::Register;
            dest.operands[last_operand_index].register = RegisterAccess {
                index: RegisterIndex::C,
                offset: 0,
                count: 1,
            };
        } else {
            dest.operands[last_operand_index].operand_type = OperandType::Immediate;
            dest.operands[last_operand_index].immediate_s32 = 1;
        }
    }

    Some(dest)
}

// Text output functions
pub fn is_printable(instruction: &Instruction) -> bool {
    !matches!(instruction.op, OperationType::Lock | OperationType::Rep | OperationType::Segment)
}

pub fn print_instruction(instruction: &Instruction, output: &mut dyn Write) -> io::Result<()> {
    let flags = instruction.flags;
    let w = (flags & InstructionFlag::WIDE) != 0;

    if (flags & InstructionFlag::LOCK) != 0 {
        if instruction.op == OperationType::Xchg {
            // Swap operands for xchg with lock prefix
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
        if operand.operand_type != OperandType::None {
            write!(output, "{}", separator)?;
            separator = ", ";

            match operand.operand_type {
                OperandType::None => {}
                OperandType::Register => {
                    write!(output, "{}", operand.register.index.get_name(operand.register.offset, operand.register.count))?;
                }
                OperandType::Memory => {
                    let address = operand.address;

                    if instruction.operands[0].operand_type != OperandType::Register {
                        write!(output, "{} ", if w { "word" } else { "byte" })?;
                    }

                    if (flags & InstructionFlag::SEGMENT) != 0 {
                        write!(output, "{}:", address.segment.get_name(0, 2))?;
                    }

                    write!(output, "[{}", address.base.expression())?;
                    if address.displacement != 0 {
                        write!(output, "{:+}", address.displacement)?;
                    }
                    write!(output, "]")?;
                }
                OperandType::Immediate => {
                    write!(output, "{}", operand.immediate_s32)?;
                }
                OperandType::RelativeImmediate => {
                    write!(output, "${:+}", operand.immediate_s32)?;
                }
            }
        }
    }

    Ok(())
}

pub fn disasm_8086(memory: &Memory, disasm_byte_count: U32, disasm_start: SegmentedAccess) -> io::Result<()> {
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
            eprintln!("ERROR: Instruction extends outside disassembly region");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_names() {
        assert_eq!(RegisterIndex::A.get_name(0, 1), "al");
        assert_eq!(RegisterIndex::A.get_name(1, 1), "ah");
        assert_eq!(RegisterIndex::A.get_name(0, 2), "ax");
        assert_eq!(RegisterIndex::B.get_name(0, 1), "bl");
        assert_eq!(RegisterIndex::B.get_name(1, 1), "bh");
        assert_eq!(RegisterIndex::B.get_name(0, 2), "bx");
    }

    #[test]
    fn test_effective_address_expressions() {
        assert_eq!(EffectiveAddressBase::BxSi.expression(), "bx+si");
        assert_eq!(EffectiveAddressBase::BxDi.expression(), "bx+di");
        assert_eq!(EffectiveAddressBase::Si.expression(), "si");
        assert_eq!(EffectiveAddressBase::Direct.expression(), "");
    }

    #[test]
    fn test_segmented_access() {
        let access = SegmentedAccess {
            segment_base: 0x1000,
            segment_offset: 0x0100,
        };
        assert_eq!(access.get_absolute_address(0), 0x10100);
        assert_eq!(access.get_absolute_address(0x10), 0x10110);
    }

    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new();
        
        // Test basic read/write
        memory.bytes[0] = 0x42;
        assert_eq!(memory.read(0), 0x42);
        
        // Test bounds
        memory.bytes[MEMORY_SIZE - 1] = 0xFF;
        assert_eq!(memory.read((MEMORY_SIZE - 1) as u32), 0xFF);
    }

    #[test]
    fn test_operation_type_mnemonics() {
        assert_eq!(OperationType::Mov.mnemonic(), "mov");
        assert_eq!(OperationType::Add.mnemonic(), "add");
        assert_eq!(OperationType::Sub.mnemonic(), "sub");
        assert_eq!(OperationType::Jmp.mnemonic(), "jmp");
        assert_eq!(OperationType::None.mnemonic(), "");
    }

    #[test]
    fn test_instruction_flags() {
        assert_eq!(InstructionFlag::LOCK, 1);
        assert_eq!(InstructionFlag::REP, 2);
        assert_eq!(InstructionFlag::SEGMENT, 4);
        assert_eq!(InstructionFlag::WIDE, 8);
    }

    #[test]
    fn test_get_reg_operand() {
        let operand = get_reg_operand(0, false); // AL
        assert_eq!(operand.operand_type, OperandType::Register);
        assert_eq!(operand.register.index, RegisterIndex::A);
        assert_eq!(operand.register.offset, 0);
        assert_eq!(operand.register.count, 1);

        let operand = get_reg_operand(0, true); // AX
        assert_eq!(operand.operand_type, OperandType::Register);
        assert_eq!(operand.register.index, RegisterIndex::A);
        assert_eq!(operand.register.offset, 0);
        assert_eq!(operand.register.count, 2);
    }

    #[test]
    fn test_disasm_context() {
        let mut context = DisasmContext::new();
        assert_eq!(context.default_segment, RegisterIndex::Ds);
        assert_eq!(context.additional_flags, 0);

        let lock_instruction = Instruction {
            op: OperationType::Lock,
            ..Default::default()
        };
        context.update(&lock_instruction);
        assert_eq!(context.additional_flags & InstructionFlag::LOCK, InstructionFlag::LOCK);

        let mov_instruction = Instruction {
            op: OperationType::Mov,
            ..Default::default()
        };
        context.update(&mov_instruction);
        assert_eq!(context.additional_flags, 0);
        assert_eq!(context.default_segment, RegisterIndex::Ds);
    }
}
