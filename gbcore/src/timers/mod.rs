use crate::{MBit, MByte, Memory};

const CPUCLOCK: usize = 4194304;
const DIVCLOCK: usize = CPUCLOCK / 16384;
const TACCLOCK0: usize = CPUCLOCK / 4096;
const TACCLOCK1: usize = CPUCLOCK / 262144;
const TACCLOCK2: usize = CPUCLOCK / 65536;
const TACCLOCK3: usize = CPUCLOCK / 16384;

type Div = MByte<0xFF04>;
type Tima = MByte<0xFF05>;
type Tma = MByte<0xFF06>;
type Tac = MByte<0xFF07>;
type TimerInt = MBit<0xFF0F, 2>;

#[derive(Default)]
pub struct Timers {
    // Cycles counters
    div_count: usize,
    tima_count: usize,
}

impl Timers {
    pub fn tick(&mut self, mmu: &mut impl Memory, ticks: u8) {
        // INC DIV register
        self.div_count = self.div_count.wrapping_add(ticks as usize);
        if self.div_count >= DIVCLOCK {
            let div = Div::read(mmu);
            if div == 0xFF {
                Div::write(mmu, 0);
            } else {
                Div::write(mmu, div.wrapping_add(1));
            }
            self.div_count %= DIVCLOCK;
        }

        if Tac::bit::<2>(mmu) {
            self.tima_count = self.tima_count.wrapping_add(ticks as usize);
            let clock = self.cycles_tima_inc(mmu);
            if self.tima_count >= clock {
                let mut tima = Tima::read(mmu);
                if tima == 0xFF {
                    tima = Tma::read(mmu);
                    TimerInt::set(mmu, true);
                } else {
                    tima = tima.wrapping_add(1);
                }
                Tima::write(mmu, tima);
                self.tima_count %= clock;
            }
        }
    }

    fn cycles_tima_inc(&mut self, mmu: &mut impl Memory) -> usize {
        match Tac::masked::<0b00000011>(mmu) {
            0 => TACCLOCK0,
            1 => TACCLOCK1,
            2 => TACCLOCK2,
            3 => TACCLOCK3,
            _ => 1, // Impossible value,
        }
    }
}
