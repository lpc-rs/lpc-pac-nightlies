#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Async peripheral reset control"]
    pub asyncpresetctrl: ASYNCPRESETCTRL,
    #[doc = "0x04 - Set bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlset: ASYNCPRESETCTRLSET,
    #[doc = "0x08 - Clear bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlclr: ASYNCPRESETCTRLCLR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Async peripheral clock control"]
    pub asyncapbclkctrl: ASYNCAPBCLKCTRL,
    #[doc = "0x14 - Set bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlset: ASYNCAPBCLKCTRLSET,
    #[doc = "0x18 - Clear bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlclr: ASYNCAPBCLKCTRLCLR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Async APB clock source select A"]
    pub asyncapbclksela: ASYNCAPBCLKSELA,
}
#[doc = "ASYNCPRESETCTRL (rw) register accessor: an alias for `Reg<ASYNCPRESETCTRL_SPEC>`"]
pub type ASYNCPRESETCTRL = crate::Reg<asyncpresetctrl::ASYNCPRESETCTRL_SPEC>;
#[doc = "Async peripheral reset control"]
pub mod asyncpresetctrl;
#[doc = "ASYNCPRESETCTRLSET (w) register accessor: an alias for `Reg<ASYNCPRESETCTRLSET_SPEC>`"]
pub type ASYNCPRESETCTRLSET = crate::Reg<asyncpresetctrlset::ASYNCPRESETCTRLSET_SPEC>;
#[doc = "Set bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlset;
#[doc = "ASYNCPRESETCTRLCLR (w) register accessor: an alias for `Reg<ASYNCPRESETCTRLCLR_SPEC>`"]
pub type ASYNCPRESETCTRLCLR = crate::Reg<asyncpresetctrlclr::ASYNCPRESETCTRLCLR_SPEC>;
#[doc = "Clear bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlclr;
#[doc = "ASYNCAPBCLKCTRL (rw) register accessor: an alias for `Reg<ASYNCAPBCLKCTRL_SPEC>`"]
pub type ASYNCAPBCLKCTRL = crate::Reg<asyncapbclkctrl::ASYNCAPBCLKCTRL_SPEC>;
#[doc = "Async peripheral clock control"]
pub mod asyncapbclkctrl;
#[doc = "ASYNCAPBCLKCTRLSET (w) register accessor: an alias for `Reg<ASYNCAPBCLKCTRLSET_SPEC>`"]
pub type ASYNCAPBCLKCTRLSET = crate::Reg<asyncapbclkctrlset::ASYNCAPBCLKCTRLSET_SPEC>;
#[doc = "Set bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlset;
#[doc = "ASYNCAPBCLKCTRLCLR (w) register accessor: an alias for `Reg<ASYNCAPBCLKCTRLCLR_SPEC>`"]
pub type ASYNCAPBCLKCTRLCLR = crate::Reg<asyncapbclkctrlclr::ASYNCAPBCLKCTRLCLR_SPEC>;
#[doc = "Clear bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlclr;
#[doc = "ASYNCAPBCLKSELA (rw) register accessor: an alias for `Reg<ASYNCAPBCLKSELA_SPEC>`"]
pub type ASYNCAPBCLKSELA = crate::Reg<asyncapbclksela::ASYNCAPBCLKSELA_SPEC>;
#[doc = "Async APB clock source select A"]
pub mod asyncapbclksela;
