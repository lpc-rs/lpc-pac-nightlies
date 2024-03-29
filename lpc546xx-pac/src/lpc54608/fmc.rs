#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub fctr: FCTR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Wait state register"]
    pub fbwst: FBWST,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved4: [u8; 0x04],
    #[doc = "0x2c..0x3c - Words of 128-bit signature word"]
    pub fmsw: [FMSW; 4],
    _reserved5: [u8; 0x0fa4],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: FMSTAT,
    _reserved6: [u8; 0x04],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "FCTR (rw) register accessor: an alias for `Reg<FCTR_SPEC>`"]
pub type FCTR = crate::Reg<fctr::FCTR_SPEC>;
#[doc = "Control register"]
pub mod fctr;
#[doc = "FBWST (rw) register accessor: an alias for `Reg<FBWST_SPEC>`"]
pub type FBWST = crate::Reg<fbwst::FBWST_SPEC>;
#[doc = "Wait state register"]
pub mod fbwst;
#[doc = "FMSSTART (rw) register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP (rw) register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "FMSW (r) register accessor: an alias for `Reg<FMSW_SPEC>`"]
pub type FMSW = crate::Reg<fmsw::FMSW_SPEC>;
#[doc = "Words of 128-bit signature word"]
pub mod fmsw;
#[doc = "FMSTAT (r) register accessor: an alias for `Reg<FMSTAT_SPEC>`"]
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "FMSTATCLR (w) register accessor: an alias for `Reg<FMSTATCLR_SPEC>`"]
pub type FMSTATCLR = crate::Reg<fmstatclr::FMSTATCLR_SPEC>;
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
