use crate::Memory;

use super::{
    Cpu,
    r8::{self, A, B, C, D, E, H, L, MemHL},
    regs::{CARRY, HCARRY, SUB, ZERO},
};

pub fn exec_next(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let pc = cpu.regs.pc();
    let op_code = mmu.read(pc);
    cpu.regs.set_pc(pc.wrapping_add(1));
    (match op_code {
        // RLC
        0x00 => rlc::<B>,
        0x01 => rlc::<C>,
        0x02 => rlc::<D>,
        0x03 => rlc::<E>,
        0x04 => rlc::<H>,
        0x05 => rlc::<L>,
        0x06 => rlc::<MemHL>,
        0x07 => rlc::<A>,

        // RRC
        0x08 => rrc::<B>,
        0x09 => rrc::<C>,
        0x0A => rrc::<D>,
        0x0B => rrc::<E>,
        0x0C => rrc::<H>,
        0x0D => rrc::<L>,
        0x0E => rrc::<MemHL>,
        0x0F => rrc::<A>,

        // RL
        0x10 => rl::<B>,
        0x11 => rl::<C>,
        0x12 => rl::<D>,
        0x13 => rl::<E>,
        0x14 => rl::<H>,
        0x15 => rl::<L>,
        0x16 => rl::<MemHL>,
        0x17 => rl::<A>,

        // RR
        0x18 => rr::<B>,
        0x19 => rr::<C>,
        0x1A => rr::<D>,
        0x1B => rr::<E>,
        0x1C => rr::<H>,
        0x1D => rr::<L>,
        0x1E => rr::<MemHL>,
        0x1F => rr::<A>,

        // SLA
        0x20 => sla::<B>,
        0x21 => sla::<C>,
        0x22 => sla::<D>,
        0x23 => sla::<E>,
        0x24 => sla::<H>,
        0x25 => sla::<L>,
        0x26 => sla::<MemHL>,
        0x27 => sla::<A>,

        // SRA
        0x28 => sra::<B>,
        0x29 => sra::<C>,
        0x2A => sra::<D>,
        0x2B => sra::<E>,
        0x2C => sra::<H>,
        0x2D => sra::<L>,
        0x2E => sra::<MemHL>,
        0x2F => sra::<A>,

        // SWAP
        0x30 => swap::<B>,
        0x31 => swap::<C>,
        0x32 => swap::<D>,
        0x33 => swap::<E>,
        0x34 => swap::<H>,
        0x35 => swap::<L>,
        0x36 => swap::<MemHL>,
        0x37 => swap::<A>,

        // SRL
        0x38 => srl::<B>,
        0x39 => srl::<C>,
        0x3A => srl::<D>,
        0x3B => srl::<E>,
        0x3C => srl::<H>,
        0x3D => srl::<L>,
        0x3E => srl::<MemHL>,
        0x3F => srl::<A>,

        // BIT 0
        0x40 => bit::<0, B>,
        0x41 => bit::<0, C>,
        0x42 => bit::<0, D>,
        0x43 => bit::<0, E>,
        0x44 => bit::<0, H>,
        0x45 => bit::<0, L>,
        0x46 => bit::<0, MemHL>,
        0x47 => bit::<0, A>,

        // BIT 1
        0x48 => bit::<1, B>,
        0x49 => bit::<1, C>,
        0x4A => bit::<1, D>,
        0x4B => bit::<1, E>,
        0x4C => bit::<1, H>,
        0x4D => bit::<1, L>,
        0x4E => bit::<1, MemHL>,
        0x4F => bit::<1, A>,

        // BIT 2
        0x50 => bit::<2, B>,
        0x51 => bit::<2, C>,
        0x52 => bit::<2, D>,
        0x53 => bit::<2, E>,
        0x54 => bit::<2, H>,
        0x55 => bit::<2, L>,
        0x56 => bit::<2, MemHL>,
        0x57 => bit::<2, A>,

        // BIT 3
        0x58 => bit::<3, B>,
        0x59 => bit::<3, C>,
        0x5A => bit::<3, D>,
        0x5B => bit::<3, E>,
        0x5C => bit::<3, H>,
        0x5D => bit::<3, L>,
        0x5E => bit::<3, MemHL>,
        0x5F => bit::<3, A>,

        // BIT 4
        0x60 => bit::<4, B>,
        0x61 => bit::<4, C>,
        0x62 => bit::<4, D>,
        0x63 => bit::<4, E>,
        0x64 => bit::<4, H>,
        0x65 => bit::<4, L>,
        0x66 => bit::<4, MemHL>,
        0x67 => bit::<4, A>,

        // BIT 5
        0x68 => bit::<5, B>,
        0x69 => bit::<5, C>,
        0x6A => bit::<5, D>,
        0x6B => bit::<5, E>,
        0x6C => bit::<5, H>,
        0x6D => bit::<5, L>,
        0x6E => bit::<5, MemHL>,
        0x6F => bit::<5, A>,

        // BIT 6
        0x70 => bit::<6, B>,
        0x71 => bit::<6, C>,
        0x72 => bit::<6, D>,
        0x73 => bit::<6, E>,
        0x74 => bit::<6, H>,
        0x75 => bit::<6, L>,
        0x76 => bit::<6, MemHL>,
        0x77 => bit::<6, A>,

        // BIT 7
        0x78 => bit::<7, B>,
        0x79 => bit::<7, C>,
        0x7A => bit::<7, D>,
        0x7B => bit::<7, E>,
        0x7C => bit::<7, H>,
        0x7D => bit::<7, L>,
        0x7E => bit::<7, MemHL>,
        0x7F => bit::<7, A>,

        // RES 0
        0x80 => res::<0, B>,
        0x81 => res::<0, C>,
        0x82 => res::<0, D>,
        0x83 => res::<0, E>,
        0x84 => res::<0, H>,
        0x85 => res::<0, L>,
        0x86 => res::<0, MemHL>,
        0x87 => res::<0, A>,

        // RES 1
        0x88 => res::<1, B>,
        0x89 => res::<1, C>,
        0x8A => res::<1, D>,
        0x8B => res::<1, E>,
        0x8C => res::<1, H>,
        0x8D => res::<1, L>,
        0x8E => res::<1, MemHL>,
        0x8F => res::<1, A>,

        // RES 2
        0x90 => res::<2, B>,
        0x91 => res::<2, C>,
        0x92 => res::<2, D>,
        0x93 => res::<2, E>,
        0x94 => res::<2, H>,
        0x95 => res::<2, L>,
        0x96 => res::<2, MemHL>,
        0x97 => res::<2, A>,

        // RES 3
        0x98 => res::<3, B>,
        0x99 => res::<3, C>,
        0x9A => res::<3, D>,
        0x9B => res::<3, E>,
        0x9C => res::<3, H>,
        0x9D => res::<3, L>,
        0x9E => res::<3, MemHL>,
        0x9F => res::<3, A>,

        // RES 4
        0xA0 => res::<4, B>,
        0xA1 => res::<4, C>,
        0xA2 => res::<4, D>,
        0xA3 => res::<4, E>,
        0xA4 => res::<4, H>,
        0xA5 => res::<4, L>,
        0xA6 => res::<4, MemHL>,
        0xA7 => res::<4, A>,

        // RES 5
        0xA8 => res::<5, B>,
        0xA9 => res::<5, C>,
        0xAA => res::<5, D>,
        0xAB => res::<5, E>,
        0xAC => res::<5, H>,
        0xAD => res::<5, L>,
        0xAE => res::<5, MemHL>,
        0xAF => res::<5, A>,

        // RES 6
        0xB0 => res::<6, B>,
        0xB1 => res::<6, C>,
        0xB2 => res::<6, D>,
        0xB3 => res::<6, E>,
        0xB4 => res::<6, H>,
        0xB5 => res::<6, L>,
        0xB6 => res::<6, MemHL>,
        0xB7 => res::<6, A>,

        // RES 7
        0xB8 => res::<7, B>,
        0xB9 => res::<7, C>,
        0xBA => res::<7, D>,
        0xBB => res::<7, E>,
        0xBC => res::<7, H>,
        0xBD => res::<7, L>,
        0xBE => res::<7, MemHL>,
        0xBF => res::<7, A>,

        // SET 0
        0xC0 => set::<0, B>,
        0xC1 => set::<0, C>,
        0xC2 => set::<0, D>,
        0xC3 => set::<0, E>,
        0xC4 => set::<0, H>,
        0xC5 => set::<0, L>,
        0xC6 => set::<0, MemHL>,
        0xC7 => set::<0, A>,

        // SET 1
        0xC8 => set::<1, B>,
        0xC9 => set::<1, C>,
        0xCA => set::<1, D>,
        0xCB => set::<1, E>,
        0xCC => set::<1, H>,
        0xCD => set::<1, L>,
        0xCE => set::<1, MemHL>,
        0xCF => set::<1, A>,

        // SET 2
        0xD0 => set::<2, B>,
        0xD1 => set::<2, C>,
        0xD2 => set::<2, D>,
        0xD3 => set::<2, E>,
        0xD4 => set::<2, H>,
        0xD5 => set::<2, L>,
        0xD6 => set::<2, MemHL>,
        0xD7 => set::<2, A>,

        // SET 3
        0xD8 => set::<3, B>,
        0xD9 => set::<3, C>,
        0xDA => set::<3, D>,
        0xDB => set::<3, E>,
        0xDC => set::<3, H>,
        0xDD => set::<3, L>,
        0xDE => set::<3, MemHL>,
        0xDF => set::<3, A>,

        // SET 4
        0xE0 => set::<4, B>,
        0xE1 => set::<4, C>,
        0xE2 => set::<4, D>,
        0xE3 => set::<4, E>,
        0xE4 => set::<4, H>,
        0xE5 => set::<4, L>,
        0xE6 => set::<4, MemHL>,
        0xE7 => set::<4, A>,

        // SET 5
        0xE8 => set::<5, B>,
        0xE9 => set::<5, C>,
        0xEA => set::<5, D>,
        0xEB => set::<5, E>,
        0xEC => set::<5, H>,
        0xED => set::<5, L>,
        0xEE => set::<5, MemHL>,
        0xEF => set::<5, A>,

        // SET 6
        0xF0 => set::<6, B>,
        0xF1 => set::<6, C>,
        0xF2 => set::<6, D>,
        0xF3 => set::<6, E>,
        0xF4 => set::<6, H>,
        0xF5 => set::<6, L>,
        0xF6 => set::<6, MemHL>,
        0xF7 => set::<6, A>,

        // SET 7
        0xF8 => set::<7, B>,
        0xF9 => set::<7, C>,
        0xFA => set::<7, D>,
        0xFB => set::<7, E>,
        0xFC => set::<7, H>,
        0xFD => set::<7, L>,
        0xFE => set::<7, MemHL>,
        0xFF => set::<7, A>,
    })(cpu, mmu)
}

fn bit<const B: u8, R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let zero = R::read(cpu, mmu) & (1 << B) == 0;
    cpu.regs.set_flag::<ZERO>(zero);
    cpu.regs.set_flag::<SUB>(false);
    cpu.regs.set_flag::<HCARRY>(true);
    8 + R::CYCLES_OVERHEAD
}

fn rl<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = (val << 1) | (cpu.regs.flag::<CARRY>() as u8);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val >> 7 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn rr<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = (val >> 1) | ((cpu.regs.flag::<CARRY>() as u8) << 7);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn srl<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = val >> 1;
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn swap<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = val.rotate_right(4);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY | CARRY }>(false);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn res<const B: u8, RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let res = RW::read(cpu, mmu) & !(1 << B);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn set<const B: u8, RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let res = RW::read(cpu, mmu) | (1 << B);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn rlc<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = val.rotate_left(1);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val >> 7 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn rrc<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = val.rotate_right(1);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn sla<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = val << 1;
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val >> 7 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}

fn sra<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    let res = (val >> 1) | (val & 0x80);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    RW::write(cpu, mmu, res);
    8 + <RW as r8::Read>::CYCLES_OVERHEAD
}
