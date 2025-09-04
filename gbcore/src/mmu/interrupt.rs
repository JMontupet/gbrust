use crate::{get_bit, set_bit};

#[derive(Default)]
pub(crate) struct Interrupt {
    // FF0F — IF: Interrupt flag
    pub ff0f_if: u8,
    // FFFF — IE: Interrupt enable
    pub ffff_ie: u8,
}

impl Interrupt {
    // FF0F - IF - Interrupt Flag (R/W)
    //   Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
    //   Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
    //   Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
    //   Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
    //   Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)

    // Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
    pub fn vblank_interrupt_request(&self) -> bool {
        get_bit::<0>(self.ff0f_if)
    }
    // Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
    pub fn set_vblank_interrupt_request(&mut self, value: bool) {
        set_bit::<0>(&mut self.ff0f_if, value)
    }
    // Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
    pub fn lcd_stat_interrupt_request(&self) -> bool {
        get_bit::<1>(self.ff0f_if)
    }
    // Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
    pub fn set_lcd_stat_interrupt_request(&mut self, value: bool) {
        set_bit::<1>(&mut self.ff0f_if, value)
    }
}
