#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller"]
    pub control: CONTROL,
    #[doc = "0x04 - Provides EMC status information"]
    pub status: STATUS,
    #[doc = "0x08 - Configures operation of the memory controller"]
    pub config: CONFIG,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - Controls dynamic memory operation"]
    pub dynamiccontrol: DYNAMICCONTROL,
    #[doc = "0x24 - Configures dynamic memory refresh"]
    pub dynamicrefresh: DYNAMICREFRESH,
    #[doc = "0x28 - Configures dynamic memory read strategy"]
    pub dynamicreadconfig: DYNAMICREADCONFIG,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - Precharge command period"]
    pub dynamicrp: DYNAMICRP,
    #[doc = "0x34 - Active to precharge command period"]
    pub dynamicras: DYNAMICRAS,
    #[doc = "0x38 - Self-refresh exit time"]
    pub dynamicsrex: DYNAMICSREX,
    #[doc = "0x3c - Last-data-out to active command time"]
    pub dynamicapr: DYNAMICAPR,
    #[doc = "0x40 - Data-in to active command time"]
    pub dynamicdal: DYNAMICDAL,
    #[doc = "0x44 - Write recovery time"]
    pub dynamicwr: DYNAMICWR,
    #[doc = "0x48 - Selects the active to active command period"]
    pub dynamicrc: DYNAMICRC,
    #[doc = "0x4c - Selects the auto-refresh period"]
    pub dynamicrfc: DYNAMICRFC,
    #[doc = "0x50 - Time for exit self-refresh to active command"]
    pub dynamicxsr: DYNAMICXSR,
    #[doc = "0x54 - Latency for active bank A to active bank B"]
    pub dynamicrrd: DYNAMICRRD,
    #[doc = "0x58 - Time for load mode register to active command"]
    pub dynamicmrd: DYNAMICMRD,
    _reserved17: [u8; 0x24],
    #[doc = "0x80 - Time for long static memory read and write transfers"]
    pub staticextendedwait: STATICEXTENDEDWAIT,
    _reserved18: [u8; 0x7c],
    #[doc = "0x100..0x108 - no description available"]
    pub dynamic0: DYNAMIC,
    _reserved19: [u8; 0x18],
    #[doc = "0x120..0x128 - no description available"]
    pub dynamic1: DYNAMIC,
    _reserved20: [u8; 0x18],
    #[doc = "0x140..0x148 - no description available"]
    pub dynamic2: DYNAMIC,
    _reserved21: [u8; 0x18],
    #[doc = "0x160..0x168 - no description available"]
    pub dynamic3: DYNAMIC,
    _reserved22: [u8; 0x98],
    #[doc = "0x200..0x21c - no description available"]
    pub static0: STATIC,
    _reserved23: [u8; 0x04],
    #[doc = "0x220..0x23c - no description available"]
    pub static1: STATIC,
    _reserved24: [u8; 0x04],
    #[doc = "0x240..0x25c - no description available"]
    pub static2: STATIC,
    _reserved25: [u8; 0x04],
    #[doc = "0x260..0x27c - no description available"]
    pub static3: STATIC,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Controls operation of the memory controller"]
pub mod control;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Provides EMC status information"]
pub mod status;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configures operation of the memory controller"]
pub mod config;
#[doc = "DYNAMICCONTROL (rw) register accessor: an alias for `Reg<DYNAMICCONTROL_SPEC>`"]
pub type DYNAMICCONTROL = crate::Reg<dynamiccontrol::DYNAMICCONTROL_SPEC>;
#[doc = "Controls dynamic memory operation"]
pub mod dynamiccontrol;
#[doc = "DYNAMICREFRESH (rw) register accessor: an alias for `Reg<DYNAMICREFRESH_SPEC>`"]
pub type DYNAMICREFRESH = crate::Reg<dynamicrefresh::DYNAMICREFRESH_SPEC>;
#[doc = "Configures dynamic memory refresh"]
pub mod dynamicrefresh;
#[doc = "DYNAMICREADCONFIG (rw) register accessor: an alias for `Reg<DYNAMICREADCONFIG_SPEC>`"]
pub type DYNAMICREADCONFIG = crate::Reg<dynamicreadconfig::DYNAMICREADCONFIG_SPEC>;
#[doc = "Configures dynamic memory read strategy"]
pub mod dynamicreadconfig;
#[doc = "DYNAMICRP (rw) register accessor: an alias for `Reg<DYNAMICRP_SPEC>`"]
pub type DYNAMICRP = crate::Reg<dynamicrp::DYNAMICRP_SPEC>;
#[doc = "Precharge command period"]
pub mod dynamicrp;
#[doc = "DYNAMICRAS (rw) register accessor: an alias for `Reg<DYNAMICRAS_SPEC>`"]
pub type DYNAMICRAS = crate::Reg<dynamicras::DYNAMICRAS_SPEC>;
#[doc = "Active to precharge command period"]
pub mod dynamicras;
#[doc = "DYNAMICSREX (rw) register accessor: an alias for `Reg<DYNAMICSREX_SPEC>`"]
pub type DYNAMICSREX = crate::Reg<dynamicsrex::DYNAMICSREX_SPEC>;
#[doc = "Self-refresh exit time"]
pub mod dynamicsrex;
#[doc = "DYNAMICAPR (rw) register accessor: an alias for `Reg<DYNAMICAPR_SPEC>`"]
pub type DYNAMICAPR = crate::Reg<dynamicapr::DYNAMICAPR_SPEC>;
#[doc = "Last-data-out to active command time"]
pub mod dynamicapr;
#[doc = "DYNAMICDAL (rw) register accessor: an alias for `Reg<DYNAMICDAL_SPEC>`"]
pub type DYNAMICDAL = crate::Reg<dynamicdal::DYNAMICDAL_SPEC>;
#[doc = "Data-in to active command time"]
pub mod dynamicdal;
#[doc = "DYNAMICWR (rw) register accessor: an alias for `Reg<DYNAMICWR_SPEC>`"]
pub type DYNAMICWR = crate::Reg<dynamicwr::DYNAMICWR_SPEC>;
#[doc = "Write recovery time"]
pub mod dynamicwr;
#[doc = "DYNAMICRC (rw) register accessor: an alias for `Reg<DYNAMICRC_SPEC>`"]
pub type DYNAMICRC = crate::Reg<dynamicrc::DYNAMICRC_SPEC>;
#[doc = "Selects the active to active command period"]
pub mod dynamicrc;
#[doc = "DYNAMICRFC (rw) register accessor: an alias for `Reg<DYNAMICRFC_SPEC>`"]
pub type DYNAMICRFC = crate::Reg<dynamicrfc::DYNAMICRFC_SPEC>;
#[doc = "Selects the auto-refresh period"]
pub mod dynamicrfc;
#[doc = "DYNAMICXSR (rw) register accessor: an alias for `Reg<DYNAMICXSR_SPEC>`"]
pub type DYNAMICXSR = crate::Reg<dynamicxsr::DYNAMICXSR_SPEC>;
#[doc = "Time for exit self-refresh to active command"]
pub mod dynamicxsr;
#[doc = "DYNAMICRRD (rw) register accessor: an alias for `Reg<DYNAMICRRD_SPEC>`"]
pub type DYNAMICRRD = crate::Reg<dynamicrrd::DYNAMICRRD_SPEC>;
#[doc = "Latency for active bank A to active bank B"]
pub mod dynamicrrd;
#[doc = "DYNAMICMRD (rw) register accessor: an alias for `Reg<DYNAMICMRD_SPEC>`"]
pub type DYNAMICMRD = crate::Reg<dynamicmrd::DYNAMICMRD_SPEC>;
#[doc = "Time for load mode register to active command"]
pub mod dynamicmrd;
#[doc = "STATICEXTENDEDWAIT (rw) register accessor: an alias for `Reg<STATICEXTENDEDWAIT_SPEC>`"]
pub type STATICEXTENDEDWAIT = crate::Reg<staticextendedwait::STATICEXTENDEDWAIT_SPEC>;
#[doc = "Time for long static memory read and write transfers"]
pub mod staticextendedwait;
#[doc = "no description available"]
pub use dynamic::DYNAMIC;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod dynamic;
#[doc = "no description available"]
pub use static_::STATIC;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod static_;
