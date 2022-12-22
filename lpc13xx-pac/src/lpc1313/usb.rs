#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Interrupt Status"]
    pub devintst: DEVINTST,
    #[doc = "0x04 - USB Device Interrupt Enable"]
    pub devinten: DEVINTEN,
    #[doc = "0x08 - USB Device Interrupt Clear"]
    pub devintctrl: DEVINTCTRL,
    #[doc = "0x0c - USB Device Interrupt Set"]
    pub devintset: DEVINTSET,
    #[doc = "0x10 - USB Command Code"]
    pub cmdcode: CMDCODE,
    #[doc = "0x14 - USB Command Data"]
    pub cmddata: CMDDATA,
    #[doc = "0x18 - USB Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x1c - USB Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x20 - USB Receive Packet Length"]
    pub rxplen: RXPLEN,
    #[doc = "0x24 - USB Transmit Packet Length"]
    pub txplenn: TXPLENN,
    #[doc = "0x28 - USB Control"]
    pub ctrl: CTRL,
    #[doc = "0x2c - USB Device FIQ select"]
    pub devfiqsel: DEVFIQSEL,
}
#[doc = "DEVINTST (r) register accessor: an alias for `Reg<DEVINTST_SPEC>`"]
pub type DEVINTST = crate::Reg<devintst::DEVINTST_SPEC>;
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "DEVINTEN (rw) register accessor: an alias for `Reg<DEVINTEN_SPEC>`"]
pub type DEVINTEN = crate::Reg<devinten::DEVINTEN_SPEC>;
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "DEVINTCTRL (w) register accessor: an alias for `Reg<DEVINTCTRL_SPEC>`"]
pub type DEVINTCTRL = crate::Reg<devintctrl::DEVINTCTRL_SPEC>;
#[doc = "USB Device Interrupt Clear"]
pub mod devintctrl;
#[doc = "DEVINTSET (w) register accessor: an alias for `Reg<DEVINTSET_SPEC>`"]
pub type DEVINTSET = crate::Reg<devintset::DEVINTSET_SPEC>;
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "CMDCODE (w) register accessor: an alias for `Reg<CMDCODE_SPEC>`"]
pub type CMDCODE = crate::Reg<cmdcode::CMDCODE_SPEC>;
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "CMDDATA (r) register accessor: an alias for `Reg<CMDDATA_SPEC>`"]
pub type CMDDATA = crate::Reg<cmddata::CMDDATA_SPEC>;
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "RXPLEN (r) register accessor: an alias for `Reg<RXPLEN_SPEC>`"]
pub type RXPLEN = crate::Reg<rxplen::RXPLEN_SPEC>;
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "TXPLENn (w) register accessor: an alias for `Reg<TXPLENN_SPEC>`"]
pub type TXPLENN = crate::Reg<txplenn::TXPLENN_SPEC>;
#[doc = "USB Transmit Packet Length"]
pub mod txplenn;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "DEVFIQSEL (w) register accessor: an alias for `Reg<DEVFIQSEL_SPEC>`"]
pub type DEVFIQSEL = crate::Reg<devfiqsel::DEVFIQSEL_SPEC>;
#[doc = "USB Device FIQ select"]
pub mod devfiqsel;
