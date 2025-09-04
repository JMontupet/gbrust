use crate::Memory;

use super::Cpu;

pub trait Read {
    const CYCLES_OVERHEAD: u8 = 0;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u16;
}

pub trait Write {
    const CYCLES_OVERHEAD: u8 = 0;
    fn write(cpu: &mut Cpu, mmu: &mut impl Memory, val: u16);
}

macro_rules! cpu_reg16 {
    ($name:ident, $getter:ident, $setter:ident) => {
        pub struct $name {}

        impl Read for $name {
            fn read(cpu: &mut Cpu, _: &mut impl Memory) -> u16 {
                cpu.regs.$getter()
            }
        }

        impl Write for $name {
            fn write(cpu: &mut Cpu, _: &mut impl Memory, val: u16) {
                cpu.regs.$setter(val);
            }
        }
    };
}

cpu_reg16!(AF, af, set_af);
cpu_reg16!(BC, bc, set_bc);
cpu_reg16!(DE, de, set_de);
cpu_reg16!(HL, hl, set_hl);
cpu_reg16!(SP, sp, set_sp);
cpu_reg16!(PC, pc, set_pc);

pub struct D16 {}
impl Read for D16 {
    const CYCLES_OVERHEAD: u8 = 8;
    fn read(cpu: &mut Cpu, mmu: &mut impl Memory) -> u16 {
        let pc = cpu.regs.pc();
        cpu.regs.set_pc(pc.wrapping_add(2));
        let lo = mmu.read(pc);
        let hi = mmu.read(pc.wrapping_add(1));
        ((hi as u16) << 8) | (lo as u16)
    }
}
