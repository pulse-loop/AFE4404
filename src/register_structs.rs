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

#[bitfield]
pub struct R03h {
    #[skip] _1: B8,
    led1ledstc: u16,
}

#[bitfield]
pub struct R04h {
    #[skip] _1: B8,
    led1ledendc: u16,
}

#[bitfield]
pub struct R05h {
    #[skip] _1: B8,
    aled2stc_or_led3stc: u16,
}

#[bitfield]
pub struct R06h {
    #[skip] _1: B8,
    aled2endc_or_led3endc: u16,
}

#[bitfield]
pub struct R07h {
    #[skip] _1: B8,
    led1stc: u16,
}

#[bitfield]
pub struct R08h {
    #[skip] _1: B8,
    led1endc: u16,
}

#[bitfield]
pub struct R09h {
    #[skip] _1: B8,
    led2ledstc: u16,
}

#[bitfield]
pub struct R0Ah {
    #[skip] _1: B8,
    led2ledendc: u16,
}

#[bitfield]
pub struct R0Bh {
    #[skip] _1: B8,
    aled1stc: u16,
}

#[bitfield]
pub struct R0Ch {
    #[skip] _1: B8,
    aled1endc: u16,
}

#[bitfield]
pub struct R0Dh {
    #[skip] _1: B8,
    led2convst: u16,
}

#[bitfield]
pub struct R0Eh {
    #[skip] _1: B8,
    led2convend: u16,
}

#[bitfield]
pub struct R0Fh {
    #[skip] _1: B8,
    aled2convst_or_led3convst: u16,
}

#[bitfield]
pub struct R10h {
    #[skip] _1: B8,
    aled2convend_or_led3convend: u16,
}

#[bitfield]
pub struct R11h {
    #[skip] _1: B8,
    led1convst: u16,
}

#[bitfield]
pub struct R12h {
    #[skip] _1: B8,
    led1convend: u16,
}

#[bitfield]
pub struct R13h {
    #[skip] _1: B8,
    aled1convst: u16,
}

#[bitfield]
pub struct R14h {
    #[skip] _1: B8,
    aled1convend: u16,
}

#[bitfield]
pub struct R15h {
    #[skip] _1: B8,
    adcrststct0: u16,
}

#[bitfield]
pub struct R16h {
    #[skip] _1: B8,
    adcrstendct0: u16,
}

#[bitfield]
pub struct R17h {
    #[skip] _1: B8,
    adcrststct1: u16,
}

#[bitfield]
pub struct R18h {
    #[skip] _1: B8,
    adcrstendct1: u16,
}

#[bitfield]
pub struct R19h {
    #[skip] _1: B8,
    adcrststct2: u16,
}

#[bitfield]
pub struct R1Ah {
    #[skip] _1: B8,
    adcrstendct2: u16,
}

#[bitfield]
pub struct R1Bh {
    #[skip] _1: B8,
    adcrststct3: u16,
}

#[bitfield]
pub struct R1Ch {
    #[skip] _1: B8,
    adcrstendct3: u16,
}

#[bitfield]
pub struct R1Dh {
    #[skip] _1: B8,
    prpct: u16,
}

#[bitfield]
pub struct R1Eh {
    #[skip] _1: B15,
    timeren: bool,
    #[skip] _2: B4,
    numav: B4,
}

#[bitfield]
pub struct R20h {
    #[skip] _1: B8,
    ensepgain: bool,
    #[skip] _2: B9,
    tia_cf_sep: B3,
    tia_gain_sep: B3,
}

#[bitfield]
pub struct R21h {
    #[skip] _1: B15,
    prog_tg_en: bool,
    #[skip] _2: B2,
    tia_cf: B3,
    tia_gain: B3,
}

#[bitfield]
pub struct R22h {
    #[skip] _1: B6,
    iled3: B6,
    iled2: B6,
    iled1: B6,
}

#[bitfield]
pub struct R23h {
    #[skip] _1: B3,
    dynamic1: bool,
    #[skip] _2: B2,
    iled_2x: bool,
    #[skip] _3: B2,
    dynamic2: bool,
    #[skip] _4: B4,
    osc_enable: bool,
    #[skip] _5: B4,
    dynamic3: bool,
    dynamic4: bool,
    #[skip] _6: B1,
    pdnrx: bool,
    pdnafe: bool,
}

#[bitfield]
pub struct R28h {
    #[skip] _1: B24,
}

#[bitfield]
pub struct R29h {
    #[skip] _1: B14,
    enable_clkout: bool,
    #[skip] _2: B4,
    clkdiv_clkout: B4,
    #[skip] _3: B1,
}

#[bitfield]
pub struct R2Ah {
    led2val: B24,
}

#[bitfield]
pub struct R2Bh {
    aled2val_or_led3val: B24,
}

#[bitfield]
pub struct R2Ch {
    led1val: B24,
}

#[bitfield]
pub struct R2Dh {
    aled1val: B24,
}

#[bitfield]
pub struct R2Eh {
    led2_minus_aled2val: B24,
}

#[bitfield]
pub struct R2Fh {
    led1_minus_aled1val: B24,
}

#[bitfield]
pub struct R31h {
    #[skip] _1: B13,
    pd_disconnect: bool,
    #[skip] _2: B4,
    enable_input_short: bool,
    #[skip] _3: B2,
    clkdiv_extmode: B3,
}

#[bitfield]
pub struct R32h {
    #[skip] _1: B8,
    pdncyclestc: u16,
}

#[bitfield]
pub struct R33h {
    #[skip] _1: B8,
    pdncycleendc: u16,
}

#[bitfield]
pub struct R34h {
    #[skip] _1: B8,
    prog_tg_stc: u16,
}

#[bitfield]
pub struct R35h {
    #[skip] _1: B8,
    prog_tg_endc: u16,
}

#[bitfield]
pub struct R36h {
    #[skip] _1: B8,
    led3ledstc: u16,
}

#[bitfield]
pub struct R37h {
    #[skip] _1: B8,
    led3ledendc: u16,
}

#[bitfield]
pub struct R39h {
    #[skip] _1: B21,
    clkdiv_prf: B3,
}

#[bitfield]
pub struct R3Ah {
    #[skip] _1: B4,
    pol_offdac_led2: bool,
    i_offdac_led2: B4,
    pol_offdac_amb1: bool,
    i_offdac_amb1: B4,
    pol_offdac_led1: bool,
    i_offdac_led1: B4,
    pol_offdac_amb2_or_pol_offdac_led3: bool,
    i_offdac_amb2_or_i_offdac_led3: B4,
}

#[bitfield]
pub struct R3Dh {
    #[skip] _1: B18,
    dec_en: bool,
    #[skip] _2: B1,
    dec_factor: B3,
    #[skip] _3: B1,
}

#[bitfield]
pub struct R3Fh {
    avg_led2_minus_aled2val: B24,
}

#[bitfield]
pub struct R40h {
    avg_led1_minus_aled1val: B24,
}