#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Oversample Rate register 0"]
    pub osr: OSR,
    #[doc = "0x04 - DMIC Clock Register 0"]
    pub divhfclk: DIVHFCLK,
    #[doc = "0x08 - Pre-Emphasis Filter Coefficient for 2 FS register"]
    pub preac2fscoef: PREAC2FSCOEF,
    #[doc = "0x0c - Pre-Emphasis Filter Coefficient for 4 FS register"]
    pub preac4fscoef: PREAC4FSCOEF,
    #[doc = "0x10 - Decimator Gain Shift register"]
    pub gainshift: GAINSHIFT,
    _reserved5: [u8; 0x6c],
    #[doc = "0x80 - FIFO Control register 0"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x84 - FIFO Status register 0"]
    pub fifo_status: FIFO_STATUS,
    #[doc = "0x88 - FIFO Data Register 0"]
    pub fifo_data: FIFO_DATA,
    #[doc = "0x8c - PDM Source Configuration register 0"]
    pub phy_ctrl: PHY_CTRL,
    #[doc = "0x90 - DC Control register 0"]
    pub dc_ctrl: DC_CTRL,
}
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversample Rate register 0"]
pub mod osr;
#[doc = "DIVHFCLK (rw) register accessor: an alias for `Reg<DIVHFCLK_SPEC>`"]
pub type DIVHFCLK = crate::Reg<divhfclk::DIVHFCLK_SPEC>;
#[doc = "DMIC Clock Register 0"]
pub mod divhfclk;
#[doc = "PREAC2FSCOEF (rw) register accessor: an alias for `Reg<PREAC2FSCOEF_SPEC>`"]
pub type PREAC2FSCOEF = crate::Reg<preac2fscoef::PREAC2FSCOEF_SPEC>;
#[doc = "Pre-Emphasis Filter Coefficient for 2 FS register"]
pub mod preac2fscoef;
#[doc = "PREAC4FSCOEF (rw) register accessor: an alias for `Reg<PREAC4FSCOEF_SPEC>`"]
pub type PREAC4FSCOEF = crate::Reg<preac4fscoef::PREAC4FSCOEF_SPEC>;
#[doc = "Pre-Emphasis Filter Coefficient for 4 FS register"]
pub mod preac4fscoef;
#[doc = "GAINSHIFT (rw) register accessor: an alias for `Reg<GAINSHIFT_SPEC>`"]
pub type GAINSHIFT = crate::Reg<gainshift::GAINSHIFT_SPEC>;
#[doc = "Decimator Gain Shift register"]
pub mod gainshift;
#[doc = "FIFO_CTRL (rw) register accessor: an alias for `Reg<FIFO_CTRL_SPEC>`"]
pub type FIFO_CTRL = crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>;
#[doc = "FIFO Control register 0"]
pub mod fifo_ctrl;
#[doc = "FIFO_STATUS (rw) register accessor: an alias for `Reg<FIFO_STATUS_SPEC>`"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "FIFO Status register 0"]
pub mod fifo_status;
#[doc = "FIFO_DATA (rw) register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "FIFO Data Register 0"]
pub mod fifo_data;
#[doc = "PHY_CTRL (rw) register accessor: an alias for `Reg<PHY_CTRL_SPEC>`"]
pub type PHY_CTRL = crate::Reg<phy_ctrl::PHY_CTRL_SPEC>;
#[doc = "PDM Source Configuration register 0"]
pub mod phy_ctrl;
#[doc = "DC_CTRL (rw) register accessor: an alias for `Reg<DC_CTRL_SPEC>`"]
pub type DC_CTRL = crate::Reg<dc_ctrl::DC_CTRL_SPEC>;
#[doc = "DC Control register 0"]
pub mod dc_ctrl;
