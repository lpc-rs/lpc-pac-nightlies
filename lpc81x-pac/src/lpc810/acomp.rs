#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Voltage ladder register"]
    pub lad: LAD,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Comparator control register"]
pub mod ctrl;
#[doc = "LAD (rw) register accessor: an alias for `Reg<LAD_SPEC>`"]
pub type LAD = crate::Reg<lad::LAD_SPEC>;
#[doc = "Voltage ladder register"]
pub mod lad;
