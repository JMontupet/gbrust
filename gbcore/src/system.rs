use crate::{
    MBit, MByte, Memory, Screen,
    cartridge::Cartridge,
    cpu,
    gpu::{
        self, State,
        oam::{self, OamDmaManager},
        vram,
    },
    hram,
    mmu::MMU,
    timers::Timers,
    unusable, wram,
};

pub struct System<C: Cartridge> {
    cpu: cpu::Cpu,
    gpu: gpu::GPU,
    mmu: MMU<C>,
    joypad: Joypad,
    timers: Timers,
    oam_manager: OamDmaManager,
}

impl<C: Cartridge> System<C> {
    pub fn new(mut cartridge: C) -> Self {
        let color_mode = cartridge.read(0x0143) == 0x80 || cartridge.read(0x0143) == 0xC0;

        let mut cpu = cpu::Cpu::default();
        let gpu = gpu::GPU::new(color_mode);
        let hram = hram::HRAM::default();
        let wram = wram::WRAM::default();
        let unusable = unusable::Unusable::default();
        let vram = vram::VRAM::default();
        let oam = oam::OAM::default();
        let mut mmu = MMU::new(cartridge, hram, wram, unusable, vram, oam);
        let joypad = Joypad::default();
        let timers = Timers::default();
        let oam_manager = OamDmaManager::default();
        cpu.reset(&mut mmu);

        Self {
            cpu,
            gpu,
            mmu,
            joypad,
            timers,
            oam_manager,
        }
    }

    pub fn tick(&mut self, screen: &mut Screen, keys: &u8) {
        let mut done = false;
        // use std::time::Instant;
        // let now = Instant::now();
        self.joypad.handle_keys(&mut self.mmu, keys);

        while !done {
            self.joypad.tick(&mut self.mmu);
            let ticks = self.cpu.tick(&mut self.mmu);

            match self.gpu.tick(&mut self.mmu, ticks) {
                State::Default => {}
                State::Frame => done |= true,
            };
            self.timers.tick(&mut self.mmu, ticks);
            self.oam_manager.tick(&mut self.mmu);
        }
        self.gpu.swap_screen(screen);
        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);
    }
}

pub struct Joypad {
    hw_buttons: u8,
    hw_arrow: u8,
}

type JoypadInterrupt = MBit<0xFF0F, 4>;
type JoypadMemory = MByte<0xFF00>;
const BUTTON_MASK: u8 = 0b00100000;
const ARROW_MASK: u8 = 0b00010000;
const SELECT_MASK: u8 = BUTTON_MASK | ARROW_MASK;

impl Default for Joypad {
    fn default() -> Self {
        Self {
            hw_buttons: 0x0F,
            hw_arrow: 0x0F,
        }
    }
}

impl Joypad {
    pub fn handle_keys(&mut self, mmu: &mut impl Memory, keys: &u8) {
        let hw_buttons = (!(keys & 0x0F)) & 0x0F; // Complement
        let hw_arrow = (!(keys >> 4)) & 0x0F; // Complement

        if self.hw_buttons & hw_buttons != self.hw_buttons
            || self.hw_arrow & hw_arrow != self.hw_arrow
        {
            JoypadInterrupt::set(mmu, true);
        }

        self.hw_buttons = hw_buttons;
        self.hw_arrow = hw_arrow;
    }

    pub fn tick(&mut self, mmu: &mut impl Memory) {
        let mem = JoypadMemory::read(mmu);
        let mut res = mem & SELECT_MASK;
        if res == SELECT_MASK {
            res |= 0x0F
        } else if mem & BUTTON_MASK == 0 {
            res |= self.hw_buttons;
        } else if mem & ARROW_MASK == 0 {
            res |= self.hw_arrow;
        }
        JoypadMemory::write(mmu, res);
    }
}
