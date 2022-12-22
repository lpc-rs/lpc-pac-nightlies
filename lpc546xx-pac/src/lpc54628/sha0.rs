#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Enable register"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Clear register"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Memory Control register"]
    pub memctrl: MEMCTRL,
    #[doc = "0x14 - Memory Address register"]
    pub memaddr: MEMADDR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Input Data register"]
    pub indata: INDATA,
    #[doc = "0x24..0x40 - Alias register"]
    pub alias: [ALIAS; 7],
    #[doc = "0x40..0x60 - Digest register"]
    pub digest: [DIGEST; 8],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Clear register"]
pub mod intenclr;
#[doc = "MEMCTRL (rw) register accessor: an alias for `Reg<MEMCTRL_SPEC>`"]
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
#[doc = "Memory Control register"]
pub mod memctrl;
#[doc = "MEMADDR (rw) register accessor: an alias for `Reg<MEMADDR_SPEC>`"]
pub type MEMADDR = crate::Reg<memaddr::MEMADDR_SPEC>;
#[doc = "Memory Address register"]
pub mod memaddr;
#[doc = "INDATA (rw) register accessor: an alias for `Reg<INDATA_SPEC>`"]
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
#[doc = "Input Data register"]
pub mod indata;
#[doc = "ALIAS (rw) register accessor: an alias for `Reg<ALIAS_SPEC>`"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "Alias register"]
pub mod alias;
#[doc = "DIGEST (r) register accessor: an alias for `Reg<DIGEST_SPEC>`"]
pub type DIGEST = crate::Reg<digest::DIGEST_SPEC>;
#[doc = "Digest register"]
pub mod digest;
