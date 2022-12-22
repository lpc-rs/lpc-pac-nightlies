#[doc = r"Register block"]
#[repr(C)]
pub struct DYNAMIC {
    #[doc = "0x00 - Configuration information for EMC_DYCSx"]
    pub dynamicconfig: DYNAMICCONFIG,
    #[doc = "0x04 - RAS and CAS latencies for EMC_DYCSx"]
    pub dynamicrascas: DYNAMICRASCAS,
}
#[doc = "DYNAMICCONFIG (rw) register accessor: an alias for `Reg<DYNAMICCONFIG_SPEC>`"]
pub type DYNAMICCONFIG = crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>;
#[doc = "Configuration information for EMC_DYCSx"]
pub mod dynamicconfig;
#[doc = "DYNAMICRASCAS (rw) register accessor: an alias for `Reg<DYNAMICRASCAS_SPEC>`"]
pub type DYNAMICRASCAS = crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>;
#[doc = "RAS and CAS latencies for EMC_DYCSx"]
pub mod dynamicrascas;
