#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1c - Trigger select register for DMA channel"]
    pub sct0_inmux: [SCT0_INMUX; 7],
    _reserved1: [u8; 0xa4],
    #[doc = "0xc0..0xe0 - Pin interrupt select register"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0xe0..0x158 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 30],
    _reserved3: [u8; 0x08],
    #[doc = "0x160..0x170 - DMA output trigger selection to become DMA trigger"]
    pub dma_otrig_inmux: [DMA_OTRIG_INMUX; 4],
    _reserved4: [u8; 0x10],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: FREQMEAS_REF,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: FREQMEAS_TARGET,
}
#[doc = "SCT0_INMUX (rw) register accessor: an alias for `Reg<SCT0_INMUX_SPEC>`"]
pub type SCT0_INMUX = crate::Reg<sct0_inmux::SCT0_INMUX_SPEC>;
#[doc = "Trigger select register for DMA channel"]
pub mod sct0_inmux;
#[doc = "PINTSEL (rw) register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "DMA_ITRIG_INMUX (rw) register accessor: an alias for `Reg<DMA_ITRIG_INMUX_SPEC>`"]
pub type DMA_ITRIG_INMUX = crate::Reg<dma_itrig_inmux::DMA_ITRIG_INMUX_SPEC>;
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
#[doc = "DMA_OTRIG_INMUX (rw) register accessor: an alias for `Reg<DMA_OTRIG_INMUX_SPEC>`"]
pub type DMA_OTRIG_INMUX = crate::Reg<dma_otrig_inmux::DMA_OTRIG_INMUX_SPEC>;
#[doc = "DMA output trigger selection to become DMA trigger"]
pub mod dma_otrig_inmux;
#[doc = "FREQMEAS_REF (rw) register accessor: an alias for `Reg<FREQMEAS_REF_SPEC>`"]
pub type FREQMEAS_REF = crate::Reg<freqmeas_ref::FREQMEAS_REF_SPEC>;
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "FREQMEAS_TARGET (rw) register accessor: an alias for `Reg<FREQMEAS_TARGET_SPEC>`"]
pub type FREQMEAS_TARGET = crate::Reg<freqmeas_target::FREQMEAS_TARGET_SPEC>;
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
