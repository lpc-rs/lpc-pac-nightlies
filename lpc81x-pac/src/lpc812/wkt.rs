#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Self wake-up timer control register."]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - Counter register."]
    pub count: COUNT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Self wake-up timer control register."]
pub mod ctrl;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Counter register."]
pub mod count;
