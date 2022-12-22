#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pcon: PCON,
    #[doc = "0x04..0x14 - General purpose register N"]
    pub gpreg: [GPREG; 4],
    #[doc = "0x14 - Deep power-down control register. Also includes bits for general purpose storage."]
    pub dpdctrl: DPDCTRL,
}
#[doc = "PCON (rw) register accessor: an alias for `Reg<PCON_SPEC>`"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Power control register"]
pub mod pcon;
#[doc = "GPREG (rw) register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General purpose register N"]
pub mod gpreg;
#[doc = "DPDCTRL (rw) register accessor: an alias for `Reg<DPDCTRL_SPEC>`"]
pub type DPDCTRL = crate::Reg<dpdctrl::DPDCTRL_SPEC>;
#[doc = "Deep power-down control register. Also includes bits for general purpose storage."]
pub mod dpdctrl;
