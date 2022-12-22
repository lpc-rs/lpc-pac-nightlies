#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Flash configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - Flash signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Flash signaure stop address register"]
    pub fmsstop: FMSSTOP,
    _reserved3: [u8; 0x04],
    #[doc = "0x2c - Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
    pub fmsw0: FMSW0,
}
#[doc = "FLASHCFG (rw) register accessor: an alias for `Reg<FLASHCFG_SPEC>`"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash configuration register"]
pub mod flashcfg;
#[doc = "FMSSTART (rw) register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Flash signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP (rw) register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Flash signaure stop address register"]
pub mod fmsstop;
#[doc = "FMSW0 (r) register accessor: an alias for `Reg<FMSW0_SPEC>`"]
pub type FMSW0 = crate::Reg<fmsw0::FMSW0_SPEC>;
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
pub mod fmsw0;
