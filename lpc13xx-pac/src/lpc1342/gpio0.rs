#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3ffc],
    #[doc = "0x3ffc - Port n data register for pins PIOn_0 to PIOn_11"]
    pub data: DATA,
    _reserved1: [u8; 0x4000],
    #[doc = "0x8000 - Data direction register for port n"]
    pub dir: DIR,
    #[doc = "0x8004 - Interrupt sense register for port n"]
    pub is: IS,
    #[doc = "0x8008 - Interrupt both edges register for port n"]
    pub ibe: IBE,
    #[doc = "0x800c - Interrupt event register for port n"]
    pub iev: IEV,
    #[doc = "0x8010 - Interrupt mask register for port n"]
    pub ie: IE,
    #[doc = "0x8014 - Raw interrupt status register for port n"]
    pub ris: RIS,
    #[doc = "0x8018 - Masked interrupt status register for port n"]
    pub mis: MIS,
    #[doc = "0x801c - Interrupt clear register for port n"]
    pub ic: IC,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Port n data register for pins PIOn_0 to PIOn_11"]
pub mod data;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data direction register for port n"]
pub mod dir;
#[doc = "IS (rw) register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "Interrupt sense register for port n"]
pub mod is;
#[doc = "IBE (rw) register accessor: an alias for `Reg<IBE_SPEC>`"]
pub type IBE = crate::Reg<ibe::IBE_SPEC>;
#[doc = "Interrupt both edges register for port n"]
pub mod ibe;
#[doc = "IEV (rw) register accessor: an alias for `Reg<IEV_SPEC>`"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "Interrupt event register for port n"]
pub mod iev;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt mask register for port n"]
pub mod ie;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw interrupt status register for port n"]
pub mod ris;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked interrupt status register for port n"]
pub mod mis;
#[doc = "IC (w) register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Interrupt clear register for port n"]
pub mod ic;
