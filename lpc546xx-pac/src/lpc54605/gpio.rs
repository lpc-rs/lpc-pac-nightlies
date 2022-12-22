#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xc0 - no description available"]
    pub b: [B; 6],
    _reserved1: [u8; 0x0f40],
    #[doc = "0x1000..0x1300 - no description available"]
    pub w: [W; 6],
    _reserved2: [u8; 0x0d00],
    #[doc = "0x2000..0x2018 - Direction registers"]
    pub dir: [DIR; 6],
    _reserved3: [u8; 0x68],
    #[doc = "0x2080..0x2098 - Mask register"]
    pub mask: [MASK; 6],
    _reserved4: [u8; 0x68],
    #[doc = "0x2100..0x2118 - Port pin register"]
    pub pin: [PIN; 6],
    _reserved5: [u8; 0x68],
    #[doc = "0x2180..0x2198 - Masked port register"]
    pub mpin: [MPIN; 6],
    _reserved6: [u8; 0x68],
    #[doc = "0x2200..0x2218 - Write: Set register for port Read: output bits for port"]
    pub set: [SET; 6],
    _reserved7: [u8; 0x68],
    #[doc = "0x2280..0x2298 - Clear port"]
    pub clr: [CLR; 6],
    _reserved8: [u8; 0x68],
    #[doc = "0x2300..0x2318 - Toggle port"]
    pub not: [NOT; 6],
    _reserved9: [u8; 0x68],
    #[doc = "0x2380..0x2398 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 6],
    _reserved10: [u8; 0x68],
    #[doc = "0x2400..0x2418 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 6],
    _reserved11: [u8; 0x68],
    #[doc = "0x2480..0x2498 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 6],
}
#[doc = "no description available"]
pub use b::B;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod b;
#[doc = "no description available"]
pub use w::W;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod w;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers"]
pub mod dir;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register"]
pub mod mask;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port pin register"]
pub mod pin;
#[doc = "MPIN (rw) register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "SET (rw) register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port"]
pub mod clr;
#[doc = "NOT (w) register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port"]
pub mod not;
#[doc = "DIRSET (w) register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR (w) register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT (w) register accessor: an alias for `Reg<DIRNOT_SPEC>`"]
pub type DIRNOT = crate::Reg<dirnot::DIRNOT_SPEC>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
