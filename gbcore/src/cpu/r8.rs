use super::{
    Cpu,
    r16::{self, BC, D16, DE, HL},
};
use crate::Memory;
use core::marker::PhantomData;

pub trait Read {
    const CYCLES_OVERHEAD: u8 = 0;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8;
}

pub trait Write {
    const CYCLES_OVERHEAD: u8 = 0;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8);
}

macro_rules! cpu_reg8 {
    ($name:ident, $getter:ident, $setter:ident) => {
        pub struct $name {}

        impl Read for $name {
            fn read(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
                cpu.regs.$getter()
            }
        }

        impl Write for $name {
            fn write(cpu: &mut Cpu, _mmu: &mut impl Memory, val: u8) {
                cpu.regs.$setter(val);
            }
        }
    };
}

cpu_reg8!(A, a, set_a);
cpu_reg8!(B, b, set_b);
cpu_reg8!(C, c, set_c);
cpu_reg8!(D, d, set_d);
cpu_reg8!(E, e, set_e);
cpu_reg8!(H, h, set_h);
cpu_reg8!(L, l, set_l);

pub struct D8 {}
impl Read for D8 {
    const CYCLES_OVERHEAD: u8 = 4;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        let pc = cpu.regs.pc();
        let val = mmu.read(pc);
        cpu.regs.set_pc(pc.wrapping_add(1));
        val
    }
}

pub struct MemReg16<R16: r16::Read> {
    _phantom: PhantomData<R16>,
}

impl<R16: r16::Read> Read for MemReg16<R16> {
    const CYCLES_OVERHEAD: u8 = R16::CYCLES_OVERHEAD + 8;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        let addr = R16::read(cpu, mmu);
        mmu.read(addr)
    }
}
impl<R16: r16::Read> Write for MemReg16<R16> {
    const CYCLES_OVERHEAD: u8 = R16::CYCLES_OVERHEAD + 8;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8) {
        let addr = R16::read(cpu, mmu);
        mmu.write(addr, val);
    }
}

pub type MemBC = MemReg16<BC>;
pub type MemDE = MemReg16<DE>;
pub type MemHL = MemReg16<HL>;
pub type MemD16 = MemReg16<D16>;

pub struct MemHLDec {}

impl Read for MemHLDec {
    const CYCLES_OVERHEAD: u8 = 4;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        let hl = cpu.regs.hl();
        cpu.regs.set_hl(hl.wrapping_sub(1));
        mmu.read(hl)
    }
}
impl Write for MemHLDec {
    const CYCLES_OVERHEAD: u8 = 4;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8) {
        mmu.write(cpu.regs.hl(), val);
        cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
    }
}

pub struct MemHLInc {}

impl Read for MemHLInc {
    const CYCLES_OVERHEAD: u8 = 4;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        let hl = cpu.regs.hl();
        cpu.regs.set_hl(hl.wrapping_add(1));
        mmu.read(hl)
    }
}
impl Write for MemHLInc {
    const CYCLES_OVERHEAD: u8 = 4;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8) {
        mmu.write(cpu.regs.hl(), val);
        cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1));
    }
}

pub struct MemC {}

impl Read for MemC {
    const CYCLES_OVERHEAD: u8 = 4;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        mmu.read(0xFF00 | (cpu.regs.c() as u16))
    }
}
impl Write for MemC {
    const CYCLES_OVERHEAD: u8 = 4;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8) {
        mmu.write(0xFF00 | (cpu.regs.c() as u16), val);
    }
}

pub struct MemD8 {}

impl Read for MemD8 {
    const CYCLES_OVERHEAD: u8 = 8;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
        let d8 = D8::read(cpu, mmu);
        mmu.read(0xFF00 | (d8 as u16))
    }
}
impl Write for MemD8 {
    const CYCLES_OVERHEAD: u8 = 8;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u8) {
        let d8 = D8::read(cpu, mmu);
        mmu.write(0xFF00 | (d8 as u16), val);
    }
}
