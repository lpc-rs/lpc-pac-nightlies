#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - Configuration for shared functions."]
    pub cfg: CFG,
    #[doc = "0x804 - Status register for Master, Slave, and Monitor functions."]
    pub stat: STAT,
    #[doc = "0x808 - Interrupt Enable Set and read register."]
    pub intenset: INTENSET,
    #[doc = "0x80c - Interrupt Enable Clear register."]
    pub intenclr: INTENCLR,
    #[doc = "0x810 - Time-out value register."]
    pub timeout: TIMEOUT,
    #[doc = "0x814 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    pub clkdiv: CLKDIV,
    #[doc = "0x818 - Interrupt Status register for Master, Slave, and Monitor functions."]
    pub intstat: INTSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x820 - Master control register."]
    pub mstctl: MSTCTL,
    #[doc = "0x824 - Master timing configuration."]
    pub msttime: MSTTIME,
    #[doc = "0x828 - Combined Master receiver and transmitter data register."]
    pub mstdat: MSTDAT,
    _reserved10: [u8; 0x14],
    #[doc = "0x840 - Slave control register."]
    pub slvctl: SLVCTL,
    #[doc = "0x844 - Combined Slave receiver and transmitter data register."]
    pub slvdat: SLVDAT,
    #[doc = "0x848..0x858 - Slave address register."]
    pub slvadr: [SLVADR; 4],
    #[doc = "0x858 - Slave Qualification for address 0."]
    pub slvqual0: SLVQUAL0,
    _reserved14: [u8; 0x24],
    #[doc = "0x880 - Monitor receiver data register."]
    pub monrxdat: MONRXDAT,
    _reserved15: [u8; 0x0778],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "TIMEOUT (rw) register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub mod clkdiv;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "MSTCTL (rw) register accessor: an alias for `Reg<MSTCTL_SPEC>`"]
pub type MSTCTL = crate::Reg<mstctl::MSTCTL_SPEC>;
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "MSTTIME (rw) register accessor: an alias for `Reg<MSTTIME_SPEC>`"]
pub type MSTTIME = crate::Reg<msttime::MSTTIME_SPEC>;
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "MSTDAT (rw) register accessor: an alias for `Reg<MSTDAT_SPEC>`"]
pub type MSTDAT = crate::Reg<mstdat::MSTDAT_SPEC>;
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "SLVCTL (rw) register accessor: an alias for `Reg<SLVCTL_SPEC>`"]
pub type SLVCTL = crate::Reg<slvctl::SLVCTL_SPEC>;
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "SLVDAT (rw) register accessor: an alias for `Reg<SLVDAT_SPEC>`"]
pub type SLVDAT = crate::Reg<slvdat::SLVDAT_SPEC>;
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "SLVADR (rw) register accessor: an alias for `Reg<SLVADR_SPEC>`"]
pub type SLVADR = crate::Reg<slvadr::SLVADR_SPEC>;
#[doc = "Slave address register."]
pub mod slvadr;
#[doc = "SLVQUAL0 (rw) register accessor: an alias for `Reg<SLVQUAL0_SPEC>`"]
pub type SLVQUAL0 = crate::Reg<slvqual0::SLVQUAL0_SPEC>;
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "MONRXDAT (r) register accessor: an alias for `Reg<MONRXDAT_SPEC>`"]
pub type MONRXDAT = crate::Reg<monrxdat::MONRXDAT_SPEC>;
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Peripheral identification register."]
pub mod id;
