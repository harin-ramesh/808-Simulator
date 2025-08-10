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
    Movs,
    Cmps,
    Scas,
    Lods,
    Stos,
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
    pub fn mnemonic(&self) -> &'static str {
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
            OperationType::Movs => "movs",
            OperationType::Cmps => "cmps",
            OperationType::Scas => "scas",
            OperationType::Lods => "lods",
            OperationType::Stos => "stos",
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
    pub bit_count: u8,
    pub shift: u8,
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct InstructionFormat {
    pub op: OperationType,
    pub bits: Vec<InstructionBits>,
}

pub fn get_instruction_formats() -> Vec<InstructionFormat> {
    vec![
        // mov
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
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1100011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 0 }, // ImpD(0)
            ],
        },
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 4, shift: 0, value: 0b1011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 }, // ImpD(1)
            ],
        },
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010000 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::DispAlwaysW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 0, shift: 0, value: 0 }, // ImpMOD(0)
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 0, shift: 0, value: 0b110 }, // ImpRM(0b110)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 }, // ImpD(1)
            ],
        },
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010001 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::DispAlwaysW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 0, shift: 0, value: 0 }, // ImpMOD(0)
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 0, shift: 0, value: 0b110 }, // ImpRM(0b110)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 0 }, // ImpD(0)
            ],
        },
        InstructionFormat {
            op: OperationType::Mov,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100011 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 1, shift: 0, value: 0b0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 1, shift: 0, value: 0b0 },
                InstructionBits { usage: InstructionBitsUsage::Sr, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },

        // push
        InstructionFormat {
            op: OperationType::Push,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b110 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 }, // ImpW(1)
            ],
        },
        InstructionFormat {
            op: OperationType::Push,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b01010 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Push,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Sr, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // pop
        InstructionFormat {
            op: OperationType::Pop,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10001111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Pop,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b01011 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Pop,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Sr, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b111 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // xchg
        InstructionFormat {
            op: OperationType::Xchg,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1000011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 }, // ImpD(1)
            ],
        },
        InstructionFormat {
            op: OperationType::Xchg,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b10010 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 0, shift: 0, value: 0b11 }, // ImpMOD(0b11)
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 }, // ImpW(1)
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 0, shift: 0, value: 0 }, // ImpRM(0)
            ],
        },

        // in/out
        InstructionFormat {
            op: OperationType::In,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1110010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },   // ImpD(1)
            ],
        },
        InstructionFormat {
            op: OperationType::In,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1110110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },   // ImpD(1)
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 0, shift: 0, value: 0b11 }, // ImpMOD(0b11)
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 0, shift: 0, value: 2 }, // ImpRM(2)
                InstructionBits { usage: InstructionBitsUsage::RmRegAlwaysW, bit_count: 0, shift: 0, value: 1 }, // Flags(Bits_RMRegAlwaysW)
            ],
        },
        InstructionFormat {
            op: OperationType::Out,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1110011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 0 },   // ImpD(0)
            ],
        },
        InstructionFormat {
            op: OperationType::Out,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1110111 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 0 },   // ImpD(0)
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 0, shift: 0, value: 0b11 }, // ImpMOD(0b11)
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 0, shift: 0, value: 2 }, // ImpRM(2)
                InstructionBits { usage: InstructionBitsUsage::RmRegAlwaysW, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // xlat, lea, lds, les, lahf, sahf, pushf, popf
        InstructionFormat {
            op: OperationType::Xlat,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11010111 }],
        },
        InstructionFormat {
            op: OperationType::Lea,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10001101 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Lds,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11000101 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Les,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11000100 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Lahf,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011111 }],
        },
        InstructionFormat {
            op: OperationType::Sahf,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011110 }],
        },
        InstructionFormat {
            op: OperationType::Pushf,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011100 }],
        },
        InstructionFormat {
            op: OperationType::Popf,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011101 }],
        },

        // add
        InstructionFormat {
            op: OperationType::Add,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b000000 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Add,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100000 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Add,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0000010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // adc
        InstructionFormat {
            op: OperationType::Adc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b000100 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Adc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100000 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b010 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Adc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0001010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // inc
        InstructionFormat {
            op: OperationType::Inc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111111 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Inc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b01000 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // aaa, daa
        InstructionFormat {
            op: OperationType::Aaa,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00110111 }],
        },
        InstructionFormat {
            op: OperationType::Daa,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00100111 }],
        },

        // sub
        InstructionFormat {
            op: OperationType::Sub,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b001010 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Sub,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100000 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b101 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Sub,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0010110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // sbb
        InstructionFormat {
            op: OperationType::Sbb,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b000110 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Sbb,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100000 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b011 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Sbb,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0001110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // dec
        InstructionFormat {
            op: OperationType::Dec,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111111 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b001 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Dec,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b01001 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // neg
        InstructionFormat {
            op: OperationType::Neg,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b011 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },

        // cmp
        InstructionFormat {
            op: OperationType::Cmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b001110 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Cmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100000 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b111 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Cmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0011110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // aas, das
        InstructionFormat {
            op: OperationType::Aas,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00111111 }],
        },
        InstructionFormat {
            op: OperationType::Das,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00101111 }],
        },

        // mul, imul, aam, div, idiv, aad, cbw, cwd
        InstructionFormat {
            op: OperationType::Mul,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b100 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 0, shift: 0, value: 0 }, // ImpS(0)
            ],
        },
        InstructionFormat {
            op: OperationType::Imul,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b101 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 0, shift: 0, value: 1 }, // ImpS(1)
            ],
        },
        InstructionFormat {
            op: OperationType::Aam,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11010100 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00001010 },
            ],
        },
        InstructionFormat {
            op: OperationType::Div,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b110 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 0, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Idiv,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b111 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::S, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Aad,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11010101 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b00001010 },
            ],
        },
        InstructionFormat {
            op: OperationType::Cbw,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011000 }],
        },
        InstructionFormat {
            op: OperationType::Cwd,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011001 }],
        },

        // not, shifts and rotates
        InstructionFormat {
            op: OperationType::Not,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b010 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Shl,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b100 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Shr,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b101 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Sar,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b111 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Rol,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Ror,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b001 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Rcl,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b010 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Rcr,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b110100 },
                InstructionBits { usage: InstructionBitsUsage::V, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b011 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },

        // and / test / or / xor
        InstructionFormat {
            op: OperationType::And,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b001000 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::And,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1000000 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b100 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::And,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0010010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        InstructionFormat {
            op: OperationType::Test,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b100001 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Test,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b000 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Test,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010100 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        InstructionFormat {
            op: OperationType::Or,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b000010 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Or,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1000000 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b001 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Or,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0000110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        InstructionFormat {
            op: OperationType::Xor,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 6, shift: 0, value: 0b001100 },
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Xor,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1000000 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b110 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Xor,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b0011010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::Reg, bit_count: 0, shift: 0, value: 0 }, // ImpREG(0)
                InstructionBits { usage: InstructionBitsUsage::D, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // rep and string ops
        InstructionFormat {
            op: OperationType::Rep,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1111001 },
                InstructionBits { usage: InstructionBitsUsage::Z, bit_count: 1, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Movs,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010010 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Cmps,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010011 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Scas,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010111 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Lods,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010110 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
            ],
        },
        InstructionFormat {
            op: OperationType::Stos,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 7, shift: 0, value: 0b1010101 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 1, shift: 0, value: 0 },
            ],
        },

        // call (direct and indirect variants)
        InstructionFormat {
            op: OperationType::Call,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11101000 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Call,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b010 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Call,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011010 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Call,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b011 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // jmp (many variants)
        InstructionFormat {
            op: OperationType::Jmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11101001 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11101011 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b100 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11101010 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jmp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111111 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b101 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // ret variants
        InstructionFormat {
            op: OperationType::Ret,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11000011 }],
        },
        InstructionFormat {
            op: OperationType::Ret,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11000010 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Ret,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001011 }],
        },
        InstructionFormat {
            op: OperationType::Ret,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001010 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::WMakesDataW, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::W, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // conditional jumps & loops (each uses DISP and RelJmpDisp)
        InstructionFormat {
            op: OperationType::Je,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110100 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jl,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111100 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jle,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111110 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jb,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110010 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jbe,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110110 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111010 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jo,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110000 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Js,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111000 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jne,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110101 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jnl,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111101 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jg,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111111 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jnb,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110011 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Ja,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110111 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jnp,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111011 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jno,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01110001 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jns,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b01111001 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        InstructionFormat {
            op: OperationType::Loop,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11100010 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Loopz,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11100001 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Loopnz,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11100000 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Jcxz,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11100011 },
                InstructionBits { usage: InstructionBitsUsage::HasDisp, bit_count: 0, shift: 0, value: 1 },
                InstructionBits { usage: InstructionBitsUsage::RelJmpDisp, bit_count: 0, shift: 0, value: 1 },
            ],
        },

        // interrupts
        InstructionFormat {
            op: OperationType::Int,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001101 },
                InstructionBits { usage: InstructionBitsUsage::HasData, bit_count: 0, shift: 0, value: 1 },
            ],
        },
        InstructionFormat {
            op: OperationType::Int3,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001100 }],
        },
        InstructionFormat {
            op: OperationType::Into,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001110 }],
        },
        InstructionFormat {
            op: OperationType::Iret,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11001111 }],
        },

        // flags control
        InstructionFormat {
            op: OperationType::Clc,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111000 }],
        },
        InstructionFormat {
            op: OperationType::Cmc,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11110101 }],
        },
        InstructionFormat {
            op: OperationType::Stc,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111001 }],
        },
        InstructionFormat {
            op: OperationType::Cld,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111100 }],
        },
        InstructionFormat {
            op: OperationType::Std,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111101 }],
        },
        InstructionFormat {
            op: OperationType::Cli,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111010 }],
        },
        InstructionFormat {
            op: OperationType::Sti,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11111011 }],
        },
        InstructionFormat {
            op: OperationType::Hlt,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11110100 }],
        },
        InstructionFormat {
            op: OperationType::Wait,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b10011011 }],
        },

        // esc
        InstructionFormat {
            op: OperationType::Esc,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 5, shift: 0, value: 0b11011 },
                InstructionBits { usage: InstructionBitsUsage::Data, bit_count: 3, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Mod, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Data, bit_count: 3, shift: 0, value: 3 },
                InstructionBits { usage: InstructionBitsUsage::Rm, bit_count: 3, shift: 0, value: 0 },
            ],
        },

        // lock
        InstructionFormat {
            op: OperationType::Lock,
            bits: vec![InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 8, shift: 0, value: 0b11110000 }],
        },

        // segment (special)
        InstructionFormat {
            op: OperationType::Segment,
            bits: vec![
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b001 },
                InstructionBits { usage: InstructionBitsUsage::Sr, bit_count: 2, shift: 0, value: 0 },
                InstructionBits { usage: InstructionBitsUsage::Literal, bit_count: 3, shift: 0, value: 0b110 },
            ],
        },
    ]
}

