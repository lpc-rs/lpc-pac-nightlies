#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSITION Register"]
    pub position: POSITION,
    #[doc = "0x04 - MASTER Register"]
    pub master: MASTER,
    #[doc = "0x08 - FLOW Register"]
    pub flow: FLOW,
    #[doc = "0x0c - Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
    pub base: BASE,
}
#[doc = "POSITION (rw) register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "POSITION Register"]
pub mod position;
#[doc = "MASTER (rw) register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MASTER Register"]
pub mod master;
#[doc = "FLOW (rw) register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "FLOW Register"]
pub mod flow;
#[doc = "BASE (r) register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
pub mod base;
