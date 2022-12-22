#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x04 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x08 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
    pub stat: STAT,
    #[doc = "0x0c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x10 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - SPI Receive Data"]
    pub rxdat: RXDAT,
    #[doc = "0x18 - SPI Transmit Data with Control"]
    pub txdatctl: TXDATCTL,
    #[doc = "0x1c - SPI Transmit Data."]
    pub txdat: TXDAT,
    #[doc = "0x20 - SPI Transmit Control"]
    pub txctl: TXCTL,
    #[doc = "0x24 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x28 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SPI Configuration register"]
pub mod cfg;
#[doc = "DLY (rw) register accessor: an alias for `Reg<DLY_SPEC>`"]
pub type DLY = crate::Reg<dly::DLY_SPEC>;
#[doc = "SPI Delay register"]
pub mod dly;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub mod intenclr;
#[doc = "RXDAT (r) register accessor: an alias for `Reg<RXDAT_SPEC>`"]
pub type RXDAT = crate::Reg<rxdat::RXDAT_SPEC>;
#[doc = "SPI Receive Data"]
pub mod rxdat;
#[doc = "TXDATCTL (rw) register accessor: an alias for `Reg<TXDATCTL_SPEC>`"]
pub type TXDATCTL = crate::Reg<txdatctl::TXDATCTL_SPEC>;
#[doc = "SPI Transmit Data with Control"]
pub mod txdatctl;
#[doc = "TXDAT (rw) register accessor: an alias for `Reg<TXDAT_SPEC>`"]
pub type TXDAT = crate::Reg<txdat::TXDAT_SPEC>;
#[doc = "SPI Transmit Data."]
pub mod txdat;
#[doc = "TXCTL (rw) register accessor: an alias for `Reg<TXCTL_SPEC>`"]
pub type TXCTL = crate::Reg<txctl::TXCTL_SPEC>;
#[doc = "SPI Transmit Control"]
pub mod txctl;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "SPI clock Divider"]
pub mod div;
#[doc = "INTSTAT (rw) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "SPI Interrupt Status"]
pub mod intstat;
