mod cb;
mod r16;
mod r8;
mod regs;

use crate::{Memory, cartridge::Cartridge, mmu::MMU};
use r8::{
    A, B, C, D, D8, E, H, L, MemBC, MemC, MemD8, MemD16, MemDE, MemHL, MemHLDec, MemHLInc, Read,
};
use r16::{AF, BC, D16, DE, HL, PC, Read as Read16, SP};
use regs::{CARRY, HCARRY, Registers, SUB, ZERO};

#[derive(Default)]
pub struct Cpu {
    regs: Registers,
    halt: bool,
    i_master: bool,
}

impl Cpu {
    pub fn reset(&mut self, mmu: &mut impl Memory) {
        // SHORTCUT TO INIT CPU & MEMORY WITHOUT BOOT SEQUENCE
        self.regs.set_pc(0x0100);
        self.regs.set_af(0x11B0); // A = 0x11 -> CGB / A = 0x01 -> GB
        self.regs.set_bc(0x0012);
        self.regs.set_de(0x00D8);
        self.regs.set_hl(0x014D);
        self.regs.set_sp(0xFFFE);

        mmu.write(0xFF50, 0x01);
        mmu.write(0xFF05, 0x00); // TIMA
        mmu.write(0xFF06, 0x00); // TMA
        mmu.write(0xFF07, 0x00); // TAC
        mmu.write(0xFF10, 0x80); // NR10
        mmu.write(0xFF11, 0xBF); // NR11
        mmu.write(0xFF12, 0xF3); // NR12
        mmu.write(0xFF14, 0xBF); // NR14
        mmu.write(0xFF16, 0x3F); // NR21
        mmu.write(0xFF17, 0x00); // NR22
        mmu.write(0xFF19, 0xBF); // NR24
        mmu.write(0xFF1A, 0x7F); // NR30
        mmu.write(0xFF1B, 0xFF); // NR31
        mmu.write(0xFF1C, 0x9F); // NR32
        mmu.write(0xFF1E, 0xBF); // NR33
        mmu.write(0xFF20, 0xFF); // NR41
        mmu.write(0xFF21, 0x00); // NR42
        mmu.write(0xFF22, 0x00); // NR43
        mmu.write(0xFF23, 0xBF); // NR30
        mmu.write(0xFF24, 0x77); // NR50
        mmu.write(0xFF25, 0xF3); // NR51
        mmu.write(0xFF26, 0xF1); // NR52
        mmu.write(0xFF40, 0x91); // LCDC
        mmu.write(0xFF42, 0x00); // SCY
        mmu.write(0xFF43, 0x00); // SCX
        mmu.write(0xFF45, 0x00); // LYC
        mmu.write(0xFF47, 0xFC); // BGP
        mmu.write(0xFF48, 0xFF); // OBP0
        mmu.write(0xFF49, 0xFF); // OBP1
        mmu.write(0xFF4A, 0x00); // WY
        mmu.write(0xFF4B, 0x00); // WX
        mmu.write(0xFFFF, 0x00); // IE
    }

    pub fn tick<C: Cartridge>(&mut self, mmu: &mut MMU<C>) -> u8 {
        let addr_interrupt = self.next_interrupt(mmu);
        if addr_interrupt != 0x0000 {
            self.halt = false;
            push::<PC>(self, mmu);
            self.regs.set_pc(addr_interrupt);
            return 8;
        }
        if self.halt {
            return 4;
        }

        exec_next(self, mmu)
    }

    fn next_interrupt<C: Cartridge>(&mut self, mmu: &mut MMU<C>) -> u16 {
        let i_flag = mmu.interrupt.ff0f_if;
        if self.halt && i_flag & 0b1111 != 0 {
            self.halt = false;
        }
        if !self.i_master {
            return 0x0000;
        }
        let i_enable = mmu.interrupt.ffff_ie;
        let valid_interrupts = i_enable & i_flag;
        match valid_interrupts.trailing_zeros() {
            // V-Blank
            0 => {
                mmu.interrupt.ff0f_if &= !(1 << 0);
                self.i_master = false;
                0x0040
            }
            // LCD STAT
            1 => {
                mmu.interrupt.ff0f_if &= !(1 << 1);
                self.i_master = false;
                0x0048
            }
            // Timer
            2 => {
                mmu.interrupt.ff0f_if &= !(1 << 2);
                self.i_master = false;
                0x0050
            }
            // Serial
            3 => {
                mmu.interrupt.ff0f_if &= !(1 << 3);
                self.i_master = false;
                0x0058
            }
            // Joypad
            4 => {
                mmu.interrupt.ff0f_if &= !(1 << 4);
                self.i_master = false;
                0x0060
            }
            _ => 0x0000,
        }
    }
}

fn exec_next<M: Memory>(cpu: &mut Cpu, mmu: &mut M) -> u8 {
    let pc = cpu.regs.pc();
    let op_code = mmu.read(pc);

    // println!("decode: {:#04x} addr:{:#06x}", op_code, pc);

    cpu.regs.set_pc(pc.wrapping_add(1));
    (match op_code {
        0x00 => nop,
        0x10 => nop,                // TODO: STOP
        0xF3 => disable_interrupts, // TODO: DI
        0xFB => enable_interrupts,  // TODO: EI
        0x76 => halt,               // TODO: HALT
        0xCB => cb::exec_next,

        // Do not exist
        0xD3 => nop,
        0xDB => nop,
        0xDD => nop,
        0xE3 => nop,
        0xE4 => nop,
        0xEB => nop,
        0xEC => nop,
        0xED => nop,
        0xF4 => nop,
        0xFC => nop,
        0xFD => nop,

        // INC
        0x04 => inc::<B>,
        0x0C => inc::<C>,
        0x14 => inc::<D>,
        0x1C => inc::<E>,
        0x24 => inc::<H>,
        0x2C => inc::<L>,
        0x34 => inc::<MemHL>,
        0x3C => inc::<A>,

        // INC 16-bit
        0x03 => inc16::<BC>,
        0x13 => inc16::<DE>,
        0x23 => inc16::<HL>,
        0x33 => inc16::<SP>,

        // DEC
        0x05 => dec::<B>,
        0x0D => dec::<C>,
        0x15 => dec::<D>,
        0x1D => dec::<E>,
        0x25 => dec::<H>,
        0x2D => dec::<L>,
        0x35 => dec::<MemHL>,
        0x3D => dec::<A>,

        // DEC 16-bit
        0x0B => dec16::<BC>,
        0x1B => dec16::<DE>,
        0x2B => dec16::<HL>,
        0x3B => dec16::<SP>,

        // ADD 16-bit
        0x09 => add16::<HL, BC>,
        0x19 => add16::<HL, DE>,
        0x29 => add16::<HL, HL>,
        0x39 => add16::<HL, SP>,

        0x07 => rlca,
        0x17 => rla,
        0x0F => rrca,
        0x1F => rra,
        0x2f => cpl,
        0x37 => scf,
        0x3F => ccf,
        0x27 => daa,

        // JR
        0x18 => jr::<D8>,
        0x28 => jr_flag::<ZERO, D8>,
        0x38 => jr_flag::<CARRY, D8>,
        0x20 => jr_n_flag::<ZERO, D8>,
        0x30 => jr_n_flag::<CARRY, D8>,

        // JP
        0xC3 => jp::<D16>,
        0xCA => jp_flag::<ZERO, D16>,
        0xDA => jp_flag::<CARRY, D16>,
        0xC2 => jp_n_flag::<ZERO, D16>,
        0xD2 => jp_n_flag::<CARRY, D16>,
        0xE9 => jp::<HL>,

        // CALL
        0xCD => call::<D16>,
        0xCC => call_flag::<ZERO, D16>,
        0xDC => call_flag::<CARRY, D16>,
        0xC4 => call_n_flag::<ZERO, D16>,
        0xD4 => call_n_flag::<CARRY, D16>,

        // RST
        0xC7 => rst::<0x0000>,
        0xD7 => rst::<0x0010>,
        0xE7 => rst::<0x0020>,
        0xF7 => rst::<0x0030>,
        0xCF => rst::<0x0008>,
        0xDF => rst::<0x0018>,
        0xEF => rst::<0x0028>,
        0xFF => rst::<0x0038>,

        // RET
        0xC9 => ret,
        0xD9 => reti,
        0xC8 => ret_flag::<ZERO>,
        0xD8 => ret_flag::<CARRY>,
        0xC0 => ret_n_flag::<ZERO>,
        0xD0 => ret_n_flag::<CARRY>,

        // PUSH
        0xC5 => push::<BC>,
        0xD5 => push::<DE>,
        0xE5 => push::<HL>,
        0xF5 => push::<AF>,

        // POP
        0xC1 => pop::<BC>,
        0xD1 => pop::<DE>,
        0xE1 => pop::<HL>,
        0xF1 => pop_af,

        // ADD
        0x80 => add::<B>,
        0x81 => add::<C>,
        0x82 => add::<D>,
        0x83 => add::<E>,
        0x84 => add::<H>,
        0x85 => add::<L>,
        0x86 => add::<MemHL>,
        0x87 => add::<A>,
        0xC6 => add::<D8>,

        // ADC
        0x88 => adc::<B>,
        0x89 => adc::<C>,
        0x8A => adc::<D>,
        0x8B => adc::<E>,
        0x8C => adc::<H>,
        0x8D => adc::<L>,
        0x8E => adc::<MemHL>,
        0x8F => adc::<A>,
        0xCE => adc::<D8>,

        // SUB
        0x90 => sub::<B>,
        0x91 => sub::<C>,
        0x92 => sub::<D>,
        0x93 => sub::<E>,
        0x94 => sub::<H>,
        0x95 => sub::<L>,
        0x96 => sub::<MemHL>,
        0x97 => sub::<A>,
        0xD6 => sub::<D8>,

        // SBC
        0x98 => sbc::<B>,
        0x99 => sbc::<C>,
        0x9A => sbc::<D>,
        0x9B => sbc::<E>,
        0x9C => sbc::<H>,
        0x9D => sbc::<L>,
        0x9E => sbc::<MemHL>,
        0x9F => sbc::<A>,
        0xDE => sbc::<D8>,

        // AND
        0xA0 => and::<B>,
        0xA1 => and::<C>,
        0xA2 => and::<D>,
        0xA3 => and::<E>,
        0xA4 => and::<H>,
        0xA5 => and::<L>,
        0xA6 => and::<MemHL>,
        0xA7 => and::<A>,
        0xE6 => and::<D8>,

        // XOR
        0xA8 => xor::<B>,
        0xA9 => xor::<C>,
        0xAA => xor::<D>,
        0xAB => xor::<E>,
        0xAC => xor::<H>,
        0xAD => xor::<L>,
        0xAE => xor::<MemHL>,
        0xAF => xor::<A>,
        0xEE => xor::<D8>,

        // OR
        0xB0 => or::<B>,
        0xB1 => or::<C>,
        0xB2 => or::<D>,
        0xB3 => or::<E>,
        0xB4 => or::<H>,
        0xB5 => or::<L>,
        0xB6 => or::<MemHL>,
        0xB7 => or::<A>,
        0xF6 => or::<D8>,

        // CP
        0xB8 => cp::<B>,
        0xB9 => cp::<C>,
        0xBA => cp::<D>,
        0xBB => cp::<E>,
        0xBC => cp::<H>,
        0xBD => cp::<L>,
        0xBE => cp::<MemHL>,
        0xBF => cp::<A>,
        0xFE => cp::<D8>,

        // LD 16-bit
        0x01 => ld_16::<BC, D16>,
        0x11 => ld_16::<DE, D16>,
        0x21 => ld_16::<HL, D16>,
        0x31 => ld_16::<SP, D16>,
        0xF8 => ld_hl_spr8,
        0xF9 => ld_16::<SP, HL>,
        0x08 => ld_a16_sp,
        0xE8 => add_sp_r8,

        // LD 8-bit
        0x40 => ld::<B, B>,
        0x41 => ld::<B, C>,
        0x42 => ld::<B, D>,
        0x43 => ld::<B, E>,
        0x44 => ld::<B, H>,
        0x45 => ld::<B, L>,
        0x46 => ld::<B, MemHL>,
        0x47 => ld::<B, A>,
        0x06 => ld::<B, D8>,

        0x48 => ld::<C, B>,
        0x49 => ld::<C, C>,
        0x4A => ld::<C, D>,
        0x4B => ld::<C, E>,
        0x4C => ld::<C, H>,
        0x4D => ld::<C, L>,
        0x4E => ld::<C, MemHL>,
        0x4F => ld::<C, A>,
        0x0E => ld::<C, D8>,

        0x50 => ld::<D, B>,
        0x51 => ld::<D, C>,
        0x52 => ld::<D, D>,
        0x53 => ld::<D, E>,
        0x54 => ld::<D, H>,
        0x55 => ld::<D, L>,
        0x56 => ld::<D, MemHL>,
        0x57 => ld::<D, A>,
        0x16 => ld::<D, D8>,

        0x58 => ld::<E, B>,
        0x59 => ld::<E, C>,
        0x5A => ld::<E, D>,
        0x5B => ld::<E, E>,
        0x5C => ld::<E, H>,
        0x5D => ld::<E, L>,
        0x5E => ld::<E, MemHL>,
        0x5F => ld::<E, A>,
        0x1E => ld::<E, D8>,

        0x60 => ld::<H, B>,
        0x61 => ld::<H, C>,
        0x62 => ld::<H, D>,
        0x63 => ld::<H, E>,
        0x64 => ld::<H, H>,
        0x65 => ld::<H, L>,
        0x66 => ld::<H, MemHL>,
        0x67 => ld::<H, A>,
        0x26 => ld::<H, D8>,

        0x68 => ld::<L, B>,
        0x69 => ld::<L, C>,
        0x6A => ld::<L, D>,
        0x6B => ld::<L, E>,
        0x6C => ld::<L, H>,
        0x6D => ld::<L, L>,
        0x6E => ld::<L, MemHL>,
        0x6F => ld::<L, A>,
        0x2E => ld::<L, D8>,

        0x70 => ld::<MemHL, B>,
        0x71 => ld::<MemHL, C>,
        0x72 => ld::<MemHL, D>,
        0x73 => ld::<MemHL, E>,
        0x74 => ld::<MemHL, H>,
        0x75 => ld::<MemHL, L>,
        // LD (HL),(HL) => N/A
        0x77 => ld::<MemHL, A>,
        0x36 => ld::<MemHL, D8>,

        0x78 => ld::<A, B>,
        0x79 => ld::<A, C>,
        0x7A => ld::<A, D>,
        0x7B => ld::<A, E>,
        0x7C => ld::<A, H>,
        0x7D => ld::<A, L>,
        0x7E => ld::<A, MemHL>,
        0x7F => ld::<A, A>,
        0x3E => ld::<A, D8>,

        // LD (xx), A
        0x02 => ld::<MemBC, A>,
        0x12 => ld::<MemDE, A>,
        0x22 => ld::<MemHLInc, A>,
        0x32 => ld::<MemHLDec, A>,
        0xE0 => ld::<MemD8, A>,
        0xE2 => ld::<MemC, A>,
        0xEA => ld::<MemD16, A>,

        // LD A, (xx)
        0x0A => ld::<A, MemBC>,
        0x1A => ld::<A, MemDE>,
        0x2A => ld::<A, MemHLInc>,
        0x3A => ld::<A, MemHLDec>,
        0xF0 => ld::<A, MemD8>,
        0xF2 => ld::<A, MemC>,
        0xFA => ld::<A, MemD16>,
    })(cpu, mmu)
}

fn halt(cpu: &mut Cpu, _: &mut impl Memory) -> u8 {
    cpu.halt = true;
    4
}

fn enable_interrupts(cpu: &mut Cpu, _: &mut impl Memory) -> u8 {
    cpu.i_master = true;
    4
}

fn disable_interrupts(cpu: &mut Cpu, _: &mut impl Memory) -> u8 {
    cpu.i_master = false;
    4
}

fn nop(_: &mut Cpu, _: &mut impl Memory) -> u8 {
    4
}

fn ld<W: r8::Write, R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = R::read(cpu, mmu);
    W::write(cpu, mmu, val);
    4 + R::CYCLES_OVERHEAD + W::CYCLES_OVERHEAD
}

fn ld_16<W: r16::Write, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = R::read(cpu, mmu);
    W::write(cpu, mmu, val);
    4 + R::CYCLES_OVERHEAD + W::CYCLES_OVERHEAD
}

fn and<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let res = cpu.regs.a() & R::read(cpu, mmu);
    cpu.regs.set_a(res);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<HCARRY>(true);
    cpu.regs.set_flag::<{ SUB | CARRY }>(false);
    4 + R::CYCLES_OVERHEAD
}

fn or<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let res = cpu.regs.a() | R::read(cpu, mmu);
    cpu.regs.set_a(res);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY | CARRY }>(false);
    4 + R::CYCLES_OVERHEAD
}

fn xor<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let res = cpu.regs.a() ^ R::read(cpu, mmu);
    cpu.regs.set_a(res);
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<{ SUB | HCARRY | CARRY }>(false);
    4 + R::CYCLES_OVERHEAD
}

fn cp<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res, hcarry, carry) = sub8_carry(cpu.regs.a(), R::read(cpu, mmu));
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<SUB>(true);
    cpu.regs.set_flag::<HCARRY>(hcarry);
    cpu.regs.set_flag::<CARRY>(carry);
    4 + R::CYCLES_OVERHEAD
}

fn add<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res, hcarry, carry) = add8_carry(cpu.regs.a(), R::read(cpu, mmu));
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<SUB>(false);
    cpu.regs.set_flag::<HCARRY>(hcarry);
    cpu.regs.set_flag::<CARRY>(carry);
    cpu.regs.set_a(res);
    4 + R::CYCLES_OVERHEAD
}

fn adc<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res_1, hcarry_1, carry_1) = add8_carry(cpu.regs.a(), cpu.regs.flag::<CARRY>() as u8);
    let (res_2, hcarry_2, carry_2) = add8_carry(res_1, R::read(cpu, mmu));

    cpu.regs.set_flag::<ZERO>(res_2 == 0);
    cpu.regs.set_flag::<SUB>(false);
    cpu.regs.set_flag::<HCARRY>(hcarry_1 || hcarry_2);
    cpu.regs.set_flag::<CARRY>(carry_1 || carry_2);
    cpu.regs.set_a(res_2);
    4 + R::CYCLES_OVERHEAD
}

fn sub<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res, hcarry, carry) = sub8_carry(cpu.regs.a(), R::read(cpu, mmu));
    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<SUB>(true);
    cpu.regs.set_flag::<HCARRY>(hcarry);
    cpu.regs.set_flag::<CARRY>(carry);
    cpu.regs.set_a(res);
    4 + R::CYCLES_OVERHEAD
}

fn sbc<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res_1, hcarry_1, carry_1) = sub8_carry(cpu.regs.a(), cpu.regs.flag::<CARRY>() as u8);
    let (res_2, hcarry_2, carry_2) = sub8_carry(res_1, R::read(cpu, mmu));

    cpu.regs.set_flag::<ZERO>(res_2 == 0);
    cpu.regs.set_flag::<SUB>(true);
    cpu.regs.set_flag::<HCARRY>(hcarry_1 || hcarry_2);
    cpu.regs.set_flag::<CARRY>(carry_1 || carry_2);
    cpu.regs.set_a(res_2);
    4 + R::CYCLES_OVERHEAD
}

fn inc<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res, hcarry, _) = add8_carry(RW::read(cpu, mmu), 1);
    RW::write(cpu, mmu, res);

    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<SUB>(false);
    cpu.regs.set_flag::<HCARRY>(hcarry);

    4 + <RW as r8::Read>::CYCLES_OVERHEAD + <RW as r8::Write>::CYCLES_OVERHEAD
}

fn dec<RW: r8::Read + r8::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let (res, hcarry, _) = sub8_carry(RW::read(cpu, mmu), 1);
    RW::write(cpu, mmu, res);

    cpu.regs.set_flag::<ZERO>(res == 0);
    cpu.regs.set_flag::<SUB>(true);
    cpu.regs.set_flag::<HCARRY>(hcarry);

    4 + <RW as r8::Read>::CYCLES_OVERHEAD + <RW as r8::Write>::CYCLES_OVERHEAD
}

fn jr<R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let r8 = R::read(cpu, mmu) as i8;
    cpu.regs
        .set_pc(cpu.regs.pc().wrapping_add_signed(r8 as i16));
    12
}

fn jr_flag<const F: u8, R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => jr::<R>(cpu, mmu),
        false => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(1));
            8
        }
    }
}

fn jr_n_flag<const F: u8, R: r8::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(1));
            8
        }
        false => jr::<R>(cpu, mmu),
    }
}

fn jp<R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let addr = R::read(cpu, mmu);
    cpu.regs.set_pc(addr);
    4 + R::CYCLES_OVERHEAD
}

fn jp_flag<const F: u8, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => jp::<R>(cpu, mmu),
        false => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(2));
            12
        }
    }
}

fn jp_n_flag<const F: u8, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(2));
            12
        }
        false => jp::<R>(cpu, mmu),
    }
}

fn rst<const ADDR: u16>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    push::<PC>(cpu, mmu);
    cpu.regs.set_pc(ADDR);
    16
}

fn call<R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let addr = R::read(cpu, mmu);
    push::<PC>(cpu, mmu);
    cpu.regs.set_pc(addr);
    24
}

fn call_flag<const F: u8, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => call::<R>(cpu, mmu),
        false => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(2));
            12
        }
    }
}

fn call_n_flag<const F: u8, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => {
            cpu.regs.set_pc(cpu.regs.pc().wrapping_add(2));
            12
        }
        false => call::<R>(cpu, mmu),
    }
}

fn ret(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    pop::<PC>(cpu, mmu);
    16
}

fn ret_flag<const F: u8>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => ret(cpu, mmu) + 4,
        false => 8,
    }
}

fn ret_n_flag<const F: u8>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    match cpu.regs.flag::<F>() {
        true => 8,
        false => ret(cpu, mmu) + 4,
    }
}

fn reti(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    pop::<PC>(cpu, mmu);
    cpu.i_master = true;
    16
}

fn push<R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = R::read(cpu, mmu);
    let sp: u16 = cpu.regs.sp();
    mmu.write(sp.wrapping_sub(1), (val >> 8) as u8);
    mmu.write(sp.wrapping_sub(2), (val & 0x00FF) as u8);
    cpu.regs.set_sp(sp.wrapping_sub(2));
    16
}

fn pop<W: r16::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let sp: u16 = cpu.regs.sp();
    let val = (mmu.read(sp) as u16) // LO
     | ((mmu.read(sp.wrapping_add(1)) as u16) << 8); // HI
    cpu.regs.set_sp(sp.wrapping_add(2));
    W::write(cpu, mmu, val);
    12
}

// Special implementation for POP AF
fn pop_af(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let sp: u16 = cpu.regs.sp();
    let val = (mmu.read(sp) as u16) // LO
     + ((mmu.read(sp.wrapping_add(1)) as u16) << 8); // HI
    cpu.regs.set_sp(sp.wrapping_add(2));
    cpu.regs.set_af(val & 0xFFF0);
    12
}

fn inc16<RW: r16::Read + r16::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    RW::write(cpu, mmu, val.wrapping_add(1));

    8
}

fn dec16<RW: r16::Read + r16::Write>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let val = RW::read(cpu, mmu);
    RW::write(cpu, mmu, val.wrapping_sub(1));

    8
}

fn add16<RW: r16::Read + r16::Write, R: r16::Read>(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let reg_1 = RW::read(cpu, mmu);
    let reg_2 = R::read(cpu, mmu);
    let (res, carry) = reg_1.overflowing_add(reg_2);
    cpu.regs.set_flag::<SUB>(false);
    cpu.regs
        .set_flag::<HCARRY>((reg_1 & 0x0FFF) + (reg_2 & 0x0FFF) > 0x0FFF);
    cpu.regs.set_flag::<CARRY>(carry);
    RW::write(cpu, mmu, res);

    8
}

fn rla(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let val = cpu.regs.a();
    let res = (val << 1) | (cpu.regs.flag::<CARRY>() as u8);
    cpu.regs.set_flag::<{ ZERO | SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val >> 7 == 1);
    cpu.regs.set_a(res);
    4
}

fn rra(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let val = cpu.regs.a();
    let res = (val >> 1) | ((cpu.regs.flag::<CARRY>() as u8) << 7);
    cpu.regs.set_flag::<{ ZERO | SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    cpu.regs.set_a(res);
    4
}

fn rlca(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let val = cpu.regs.a();
    let res = val.rotate_left(1);
    cpu.regs.set_flag::<{ ZERO | SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val >> 7 == 1);
    cpu.regs.set_a(res);
    4
}

fn rrca(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let val = cpu.regs.a();
    let res = val.rotate_right(1);
    cpu.regs.set_flag::<{ ZERO | SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(val & 0x01 == 1);
    cpu.regs.set_a(res);
    4
}

fn cpl(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let val = cpu.regs.a();
    cpu.regs.set_a(!val);
    cpu.regs.set_flag::<{ SUB | HCARRY }>(true);
    4
}

fn scf(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(true);
    4
}

fn ccf(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let res = cpu.regs.flag::<CARRY>();
    cpu.regs.set_flag::<{ SUB | HCARRY }>(false);
    cpu.regs.set_flag::<CARRY>(!res);
    4
}

fn daa(cpu: &mut Cpu, _mmu: &mut impl Memory) -> u8 {
    let mut reg_a = cpu.regs.a();
    if cpu.regs.flag::<SUB>() {
        if cpu.regs.flag::<CARRY>() {
            reg_a = reg_a.wrapping_sub(0x60)
        }
        if cpu.regs.flag::<HCARRY>() {
            reg_a = reg_a.wrapping_sub(0x06)
        }
    } else {
        if reg_a > 0x99 || cpu.regs.flag::<CARRY>() {
            reg_a = reg_a.wrapping_add(0x60);
            cpu.regs.set_flag::<CARRY>(true);
        }
        if (reg_a & 0x0F) > 0x09 || cpu.regs.flag::<HCARRY>() {
            reg_a = reg_a.wrapping_add(0x06)
        }
    }
    cpu.regs.set_a(reg_a);
    cpu.regs.set_flag::<ZERO>(reg_a == 0);
    cpu.regs.set_flag::<HCARRY>(false);

    4
}

fn ld_hl_spr8(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let sp = cpu.regs.sp();
    let d8 = D8::read(cpu, mmu);

    let (_, hcarry, carry) = add8_carry(sp as u8, d8);
    let res = sp.wrapping_add_signed(d8 as i8 as i16);

    cpu.regs.set_flag::<{ ZERO | SUB }>(false);
    cpu.regs.set_flag::<HCARRY>(hcarry);
    cpu.regs.set_flag::<CARRY>(carry);
    cpu.regs.set_hl(res);

    12
}

fn add_sp_r8(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let sp = cpu.regs.sp();
    let d8 = D8::read(cpu, mmu);

    let (_, hcarry, carry) = add8_carry(sp as u8, d8);
    let res = sp.wrapping_add_signed(d8 as i8 as i16);

    cpu.regs.set_flag::<{ ZERO | SUB }>(false);
    cpu.regs.set_flag::<HCARRY>(hcarry);
    cpu.regs.set_flag::<CARRY>(carry);
    cpu.regs.set_sp(res);

    16
}

fn ld_a16_sp(cpu: &mut Cpu, mmu: &mut impl Memory) -> u8 {
    let sp = cpu.regs.sp();
    let addr = D16::read(cpu, mmu);

    mmu.write(addr, (sp & 0x00FF) as u8);
    mmu.write(addr.wrapping_add(1), (sp >> 8) as u8);

    20
}

pub fn add8_carry(a: u8, b: u8) -> (u8, bool, bool) {
    let (res, carry) = a.overflowing_add(b);
    (res, (a & 0x0F) + (b & 0x0F) > 0x0F, carry)
}

pub fn sub8_carry(a: u8, b: u8) -> (u8, bool, bool) {
    let (res, carry) = a.overflowing_sub(b);
    (res, (a & 0x0F) < (b & 0x0F), carry)
}
