#[derive(Default)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

pub const ZERO: u8 = 1 << 7;
pub const SUB: u8 = 1 << 6;
pub const HCARRY: u8 = 1 << 5;
pub const CARRY: u8 = 1 << 4;

macro_rules! r8_get {
    ($name:ident, $r8:ident) => {
        pub fn $name(&self) -> u8 {
            self.$r8
        }
    };
}

macro_rules! r8_set {
    ($name:ident, $r8:ident) => {
        pub fn $name(&mut self, val: u8) {
            self.$r8 = val;
        }
    };
}

macro_rules! r16_get {
    ($name:ident, $hi:ident, $lo:ident, $r16:ident) => {
        pub fn $name(&self) -> u16 {
            ((self.$hi as u16) << 8) | (self.$lo as u16)
        }
    };
}

macro_rules! r16_set {
    ($name:ident, $hi:ident, $lo:ident, $r16:ident) => {
        pub fn $name(&mut self, val: u16) {
            self.$hi = (val >> 8) as u8;
            self.$lo = val as u8;
        }
    };
}

impl Registers {
    r8_get!(a, a);
    r8_get!(b, b);
    r8_get!(c, c);
    r8_get!(d, d);
    r8_get!(e, e);
    r8_get!(h, h);
    r8_get!(l, l);

    r8_set!(set_a, a);
    r8_set!(set_b, b);
    r8_set!(set_c, c);
    r8_set!(set_d, d);
    r8_set!(set_e, e);
    r8_set!(set_h, h);
    r8_set!(set_l, l);

    r16_get!(af, a, f, af);
    r16_get!(bc, b, c, bc);
    r16_get!(de, d, e, de);
    r16_get!(hl, h, l, hl);

    r16_set!(set_af, a, f, af);
    r16_set!(set_bc, b, c, bc);
    r16_set!(set_de, d, e, de);
    r16_set!(set_hl, h, l, hl);

    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }
    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    pub fn flag<const M: u8>(&mut self) -> bool {
        self.f & M != 0
    }
    pub fn set_flag<const M: u8>(&mut self, value: bool) {
        match value {
            true => self.f |= M,
            false => self.f &= !M,
        };
    }
}
