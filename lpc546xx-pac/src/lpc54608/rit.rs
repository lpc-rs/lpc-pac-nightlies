#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare value LSB register"]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask LSB register"]
    pub mask: MASK,
    #[doc = "0x08 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Counter LSB register"]
    pub counter: COUNTER,
    #[doc = "0x10 - Compare value MSB register"]
    pub compval_h: COMPVAL_H,
    #[doc = "0x14 - Mask MSB register"]
    pub mask_h: MASK_H,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Counter MSB register"]
    pub counter_h: COUNTER_H,
}
#[doc = "COMPVAL (rw) register accessor: an alias for `Reg<COMPVAL_SPEC>`"]
pub type COMPVAL = crate::Reg<compval::COMPVAL_SPEC>;
#[doc = "Compare value LSB register"]
pub mod compval;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask LSB register"]
pub mod mask;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "COUNTER (rw) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter LSB register"]
pub mod counter;
#[doc = "COMPVAL_H (rw) register accessor: an alias for `Reg<COMPVAL_H_SPEC>`"]
pub type COMPVAL_H = crate::Reg<compval_h::COMPVAL_H_SPEC>;
#[doc = "Compare value MSB register"]
pub mod compval_h;
#[doc = "MASK_H (rw) register accessor: an alias for `Reg<MASK_H_SPEC>`"]
pub type MASK_H = crate::Reg<mask_h::MASK_H_SPEC>;
#[doc = "Mask MSB register"]
pub mod mask_h;
#[doc = "COUNTER_H (rw) register accessor: an alias for `Reg<COUNTER_H_SPEC>`"]
pub type COUNTER_H = crate::Reg<counter_h::COUNTER_H_SPEC>;
#[doc = "Counter MSB register"]
pub mod counter_h;
