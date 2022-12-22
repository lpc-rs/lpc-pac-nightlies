#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM command register"]
    pub cmd: CMD,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EEPROM read wait state register"]
    pub rwstate: RWSTATE,
    #[doc = "0x0c - EEPROM auto programming register"]
    pub autoprog: AUTOPROG,
    #[doc = "0x10 - EEPROM wait state register"]
    pub wstate: WSTATE,
    #[doc = "0x14 - EEPROM clock divider register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - EEPROM power-down register"]
    pub pwrdwn: PWRDWN,
    _reserved6: [u8; 0x0fbc],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub intenset: INTENSET,
    #[doc = "0xfe0 - EEPROM interrupt status"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: INTEN,
    #[doc = "0xfe8 - EEPROM interrupt status clear"]
    pub intstatclr: INTSTATCLR,
    #[doc = "0xfec - EEPROM interrupt status set"]
    pub intstatset: INTSTATSET,
}
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "EEPROM command register"]
pub mod cmd;
#[doc = "RWSTATE (rw) register accessor: an alias for `Reg<RWSTATE_SPEC>`"]
pub type RWSTATE = crate::Reg<rwstate::RWSTATE_SPEC>;
#[doc = "EEPROM read wait state register"]
pub mod rwstate;
#[doc = "AUTOPROG (rw) register accessor: an alias for `Reg<AUTOPROG_SPEC>`"]
pub type AUTOPROG = crate::Reg<autoprog::AUTOPROG_SPEC>;
#[doc = "EEPROM auto programming register"]
pub mod autoprog;
#[doc = "WSTATE (rw) register accessor: an alias for `Reg<WSTATE_SPEC>`"]
pub type WSTATE = crate::Reg<wstate::WSTATE_SPEC>;
#[doc = "EEPROM wait state register"]
pub mod wstate;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "EEPROM clock divider register"]
pub mod clkdiv;
#[doc = "PWRDWN (rw) register accessor: an alias for `Reg<PWRDWN_SPEC>`"]
pub type PWRDWN = crate::Reg<pwrdwn::PWRDWN_SPEC>;
#[doc = "EEPROM power-down register"]
pub mod pwrdwn;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "EEPROM interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET (w) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "EEPROM interrupt enable set"]
pub mod intenset;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "EEPROM interrupt status"]
pub mod intstat;
#[doc = "INTEN (r) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "INTSTATCLR (w) register accessor: an alias for `Reg<INTSTATCLR_SPEC>`"]
pub type INTSTATCLR = crate::Reg<intstatclr::INTSTATCLR_SPEC>;
#[doc = "EEPROM interrupt status clear"]
pub mod intstatclr;
#[doc = "INTSTATSET (w) register accessor: an alias for `Reg<INTSTATSET_SPEC>`"]
pub type INTSTATSET = crate::Reg<intstatset::INTSTATSET_SPEC>;
#[doc = "EEPROM interrupt status set"]
pub mod intstatset;
