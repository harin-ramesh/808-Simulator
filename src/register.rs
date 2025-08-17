#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RegisterIndex {
    A,
    B,
    C,
    D,
    SP,
    BP,
    SI,
    DI,
    ES,
    CS,
    SS,
    DS,
    IP,
    Flags,
}

impl RegisterIndex {
    pub fn get_name(&self, offset: u8, count: u8) -> &'static str {
        match self {
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
            RegisterIndex::SP => "sp",
            RegisterIndex::BP => "bp",
            RegisterIndex::SI => "si",
            RegisterIndex::DI => "di",
            RegisterIndex::ES => "es",
            RegisterIndex::CS => "cs",
            RegisterIndex::SS => "ss",
            RegisterIndex::DS => "ds",
            RegisterIndex::IP => "ip",
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

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Carry     = 0,  // CF - Carry Flag
    Parity    = 2,  // PF - Parity Flag
    Auxiliary = 4,  // AF - Auxiliary Carry Flag
    Zero      = 6,  // ZF - Zero Flag
    Sign      = 7,  // SF - Sign Flag
    Overflow  = 11, // OF - Overflow Flag
}

impl Flag {
    fn mask(self) -> u16 {
        1 << (self as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RegisterFile {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub si: u16,
    pub di: u16,
    pub bp: u16,
    pub sp: u16,
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub ss: u16,
    pub flags: u16,
    pub ip: u16,
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            si: 0,
            di: 0,
            bp: 0,
            sp: 0,
            cs: 0,
            ds: 0,
            es: 0,
            ss: 0,
            flags: 0,
            ip: 0,
        }
    }

    pub fn print_state(&self) {
        println!("\nRegisters:");
        println!(
            "AX={:04}  BX={:04}  CX={:04}  DX={:04}",
            self.ax, self.bx, self.cx, self.dx
        );
        println!(
            "SI={:04}  DI={:04}  BP={:04}  SP={:04}",
            self.si, self.di, self.bp, self.sp
        );
        println!(
            "CS={:04}  DS={:04}  ES={:04}  SS={:04}",
            self.cs, self.ds, self.es, self.ss
        );
        println!("IP={:04}", self.ip);

        println!("\nFlags:");
        println!(
            "CF={} PF={} AF={} ZF={} SF={} OF={}",
            self.get_flag(Flag::Carry) as u8,
            self.get_flag(Flag::Parity) as u8,
            self.get_flag(Flag::Auxiliary) as u8,
            self.get_flag(Flag::Zero) as u8,
            self.get_flag(Flag::Sign) as u8,
            self.get_flag(Flag::Overflow) as u8
        );
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= flag.mask();
        } else {
            self.flags &= !flag.mask();
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.flags & flag.mask()) != 0
    }

    pub fn get_register_value(&self, reg_access: &RegisterAccess) -> u16 {
        match reg_access.index {
            RegisterIndex::A => {
                if reg_access.count == 2 {
                    self.ax as u16
                } else {
                    if reg_access.offset == 0 {
                        (self.ax & 0x00FF) as u16
                    } else {
                        (self.ax >> 8) as u16
                    }
                }
            }
            RegisterIndex::B => {
                if reg_access.count == 2 {
                    self.bx as u16
                } else {
                    if reg_access.offset == 0 {
                        (self.bx & 0x00FF) as u16
                    } else {
                        (self.bx >> 8) as u16
                    }
                }
            }
            RegisterIndex::C => {
                if reg_access.count == 2 {
                    self.cx as u16
                } else {
                    if reg_access.offset == 0 {
                        (self.cx & 0x00FF) as u16
                    } else {
                        (self.cx >> 8) as u16
                    }
                }
            }
            RegisterIndex::D => {
                if reg_access.count == 2 {
                    self.dx as u16
                } else {
                    if reg_access.offset == 0 {
                        (self.dx & 0x00FF) as u16
                    } else {
                        (self.dx >> 8) as u16
                    }
                }
            }
            RegisterIndex::SI => self.si,
            RegisterIndex::DI => self.di,
            RegisterIndex::BP => self.bp,
            RegisterIndex::SP => self.sp,
            RegisterIndex::CS => self.cs,
            RegisterIndex::DS => self.ds,
            RegisterIndex::ES => self.es,
            RegisterIndex::SS => self.ss,
            RegisterIndex::IP => self.ip,
            RegisterIndex::Flags => self.flags,
        }
    }

    pub fn update_register(&mut self, reg_access: &RegisterAccess, value: u16) {
        match reg_access.index {
            RegisterIndex::A => {
                if reg_access.count == 2 {
                    self.ax = value;
                } else {
                    if reg_access.offset == 0 {
                        self.ax = (self.ax & 0xFF00) | (value & 0x00FF);
                    } else {
                        self.ax = (self.ax & 0x00FF) | ((value & 0x00FF) << 8);
                    }
                }
            }
            RegisterIndex::B => {
                if reg_access.count == 2 {
                    self.bx = value;
                } else {
                    if reg_access.offset == 0 {
                        self.bx = (self.bx & 0xFF00) | (value & 0x00FF);
                    } else {
                        self.bx = (self.bx & 0x00FF) | ((value & 0x00FF) << 8);
                    }
                }
            }
            RegisterIndex::C => {
                if reg_access.count == 2 {
                    self.cx = value;
                } else {
                    if reg_access.offset == 0 {
                        self.cx = (self.cx & 0xFF00) | (value & 0x00FF);
                    } else {
                        self.cx = (self.cx & 0x00FF) | ((value & 0x00FF) << 8);
                    }
                }
            }
            RegisterIndex::D => {
                if reg_access.count == 2 {
                    self.dx = value;
                } else {
                    if reg_access.offset == 0 {
                        self.dx = (self.dx & 0xFF00) | (value & 0x00FF);
                    } else {
                        self.dx = (self.dx & 0x00FF) | ((value & 0x00FF) << 8);
                    }
                }
            }
            RegisterIndex::SI => self.si = value,
            RegisterIndex::DI => self.di = value,
            RegisterIndex::BP => self.bp = value,
            RegisterIndex::SP => self.sp = value,
            RegisterIndex::CS => self.cs = value,
            RegisterIndex::DS => self.ds = value,
            RegisterIndex::ES => self.es = value,
            RegisterIndex::SS => self.ss = value,
            RegisterIndex::IP => self.ip = value,
            RegisterIndex::Flags => self.flags = value,
        }
    }

    pub fn update_ip(&mut self, value: u16) {
        self.ip = self.ip + value;
    }

    pub fn get_ip(&mut self) -> u16 {
        self.ip
    }

    fn set_flags_add(&mut self, src: u16, dst: u16, res: u16) {
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Sign, (res & 0x8000) != 0);
        self.set_flag(Flag::Overflow, ((dst ^ res) & (src ^ res) & 0x8000) != 0);
        self.set_flag(Flag::Parity, (res & 0xFF).count_ones() % 2 == 0);
        self.set_flag(Flag::Auxiliary, ((dst ^ src ^ res) & 0x10) != 0);
    }

    fn set_flags_sub(&mut self, src: u16, dst: u16, res: u16) {
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Sign, (res & 0x8000) != 0);
        self.set_flag(Flag::Overflow, ((dst ^ src) & (dst ^ res) & 0x8000) != 0);
        self.set_flag(Flag::Parity, (res & 0xFF).count_ones() % 2 == 0);
        self.set_flag(Flag::Auxiliary, ((dst ^ src ^ res) & 0x10) != 0);
    }

    pub fn alu_add(&mut self, dst: u16, src: u16) -> u16 {
        let res = dst.wrapping_add(src);
        self.set_flag(Flag::Carry, (dst as u32 + src as u32) > 0xFFFF);
        self.set_flags_add(src, dst, res);
        res
    }

    pub fn alu_sub(&mut self, dst: u16, src: u16) -> u16 {
        let res = dst.wrapping_sub(src);
        self.set_flag(Flag::Carry, dst < src);
        self.set_flags_sub(src, dst, res);
        res
    }
}
