use modular_bitfield::prelude::*;

#[bitfield]
pub struct R00h {
    #[skip] _1: B20,
    sw_reset: bool,
    #[skip] _2: B1,
    tm_count_rst: bool,
    reg_read: bool,
}

#[bitfield]
pub struct R01h {
    #[skip] _1: B8,
    led2stc: u16,
}

#[bitfield]
pub struct R02h {
    #[skip] _1: B8,
    led2endc: u16,
}
