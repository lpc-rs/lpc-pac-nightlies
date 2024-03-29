#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - System Configuration Register"]
    pub scr: SCR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Trace Enable Event Register"]
    pub eevr: EEVR,
    #[doc = "0x24 - Trace Enable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - FIFOFULL Level Register"]
    pub fflr: FFLR,
    _reserved8: [u8; 0x0114],
    #[doc = "0x140 - Free-running counter reload value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved9: [u8; 0x9c],
    #[doc = "0x1e0 - Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved12: [u8; 0x04],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved13: [u8; 0x04],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub tsevr: TSEVR,
    _reserved14: [u8; 0x04],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved15: [u8; 0x04],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved16: [u8; 0x0108],
    #[doc = "0x314 - Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved17: [u8; 0x0bc8],
    #[doc = "0xee0 - Integration Test Miscelaneous Inputs Register"]
    pub _itmiscin: _ITMISCIN,
    _reserved18: [u8; 0x04],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub _ittrigout: _ITTRIGOUT,
    _reserved19: [u8; 0x04],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub _itatbctr2: _ITATBCTR2,
    _reserved20: [u8; 0x04],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub _itatbctr0: _ITATBCTR0,
    _reserved21: [u8; 0x04],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved22: [u8; 0x9c],
    #[doc = "0xfa0 - Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved24: [u8; 0x08],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved27: [u8; 0x10],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Peripheral Identification Register 4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Peripheral Identification Register 5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Peripheral Identification Register 6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Peripheral Identification Register 7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Peripheral Identification Register 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral Identification Register 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral Identification Register 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral Identification Register 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component Identification Register 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component Identification Register 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component Identification Register 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component Identification Register 3"]
    pub cidr3: CIDR3,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Main Control Register"]
pub mod cr;
#[doc = "CCR (r) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration Code Register"]
pub mod ccr;
#[doc = "TRIGGER (rw) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Trigger Event Register"]
pub mod trigger;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "SCR (r) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Configuration Register"]
pub mod scr;
#[doc = "EEVR (rw) register accessor: an alias for `Reg<EEVR_SPEC>`"]
pub type EEVR = crate::Reg<eevr::EEVR_SPEC>;
#[doc = "Trace Enable Event Register"]
pub mod eevr;
#[doc = "TECR1 (rw) register accessor: an alias for `Reg<TECR1_SPEC>`"]
pub type TECR1 = crate::Reg<tecr1::TECR1_SPEC>;
#[doc = "Trace Enable Control 1 Register"]
pub mod tecr1;
#[doc = "FFLR (rw) register accessor: an alias for `Reg<FFLR_SPEC>`"]
pub type FFLR = crate::Reg<fflr::FFLR_SPEC>;
#[doc = "FIFOFULL Level Register"]
pub mod fflr;
#[doc = "CNTRLDVR1 (rw) register accessor: an alias for `Reg<CNTRLDVR1_SPEC>`"]
pub type CNTRLDVR1 = crate::Reg<cntrldvr1::CNTRLDVR1_SPEC>;
#[doc = "Free-running counter reload value"]
pub mod cntrldvr1;
#[doc = "SYNCFR (r) register accessor: an alias for `Reg<SYNCFR_SPEC>`"]
pub type SYNCFR = crate::Reg<syncfr::SYNCFR_SPEC>;
#[doc = "Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "ID Register"]
pub mod idr;
#[doc = "CCER (r) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TESSEICR (rw) register accessor: an alias for `Reg<TESSEICR_SPEC>`"]
pub type TESSEICR = crate::Reg<tesseicr::TESSEICR_SPEC>;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "TSEVR (rw) register accessor: an alias for `Reg<TSEVR_SPEC>`"]
pub type TSEVR = crate::Reg<tsevr::TSEVR_SPEC>;
#[doc = "Timestamp Event Register"]
pub mod tsevr;
#[doc = "TRACEIDR (rw) register accessor: an alias for `Reg<TRACEIDR_SPEC>`"]
pub type TRACEIDR = crate::Reg<traceidr::TRACEIDR_SPEC>;
#[doc = "CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "IDR2 (r) register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "PDSR (r) register accessor: an alias for `Reg<PDSR_SPEC>`"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "_ITMISCIN (r) register accessor: an alias for `Reg<_ITMISCIN_SPEC>`"]
pub type _ITMISCIN = crate::Reg<_itmiscin::_ITMISCIN_SPEC>;
#[doc = "Integration Test Miscelaneous Inputs Register"]
pub mod _itmiscin;
#[doc = "_ITTRIGOUT (rw) register accessor: an alias for `Reg<_ITTRIGOUT_SPEC>`"]
pub type _ITTRIGOUT = crate::Reg<_ittrigout::_ITTRIGOUT_SPEC>;
#[doc = "Integration Test Trigger Out Register"]
pub mod _ittrigout;
#[doc = "_ITATBCTR2 (r) register accessor: an alias for `Reg<_ITATBCTR2_SPEC>`"]
pub type _ITATBCTR2 = crate::Reg<_itatbctr2::_ITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod _itatbctr2;
#[doc = "_ITATBCTR0 (rw) register accessor: an alias for `Reg<_ITATBCTR0_SPEC>`"]
pub type _ITATBCTR0 = crate::Reg<_itatbctr0::_ITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod _itatbctr0;
#[doc = "ITCTRL (rw) register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR (rw) register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "Lock Access Register"]
pub mod lar;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS (r) register accessor: an alias for `Reg<AUTHSTATUS_SPEC>`"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVTYPE (r) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "CoreSight Device Type Register"]
pub mod devtype;
#[doc = "PIDR4 (r) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "Peripheral Identification Register 4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "Peripheral Identification Register 5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "Peripheral Identification Register 6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "Peripheral Identification Register 7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "Peripheral Identification Register 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "Peripheral Identification Register 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "Peripheral Identification Register 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "Peripheral Identification Register 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "Component Identification Register 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "Component Identification Register 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "Component Identification Register 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "Component Identification Register 3"]
pub mod cidr3;
