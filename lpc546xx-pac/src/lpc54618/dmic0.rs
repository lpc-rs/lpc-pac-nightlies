#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x94 - no description available"]
    pub channel0: CHANNEL,
    _reserved1: [u8; 0x6c],
    #[doc = "0x100..0x194 - no description available"]
    pub channel1: CHANNEL,
    _reserved2: [u8; 0x0d6c],
    #[doc = "0xf00 - Channel Enable register"]
    pub chanen: CHANEN,
    _reserved3: [u8; 0x08],
    #[doc = "0xf0c - I/O Configuration register"]
    pub iocfg: IOCFG,
    #[doc = "0xf10 - Use 2FS register"]
    pub use2fs: USE2FS,
    _reserved5: [u8; 0x6c],
    #[doc = "0xf80 - HWVAD input gain register"]
    pub hwvadgain: HWVADGAIN,
    #[doc = "0xf84 - HWVAD filter control register"]
    pub hwvadhpfs: HWVADHPFS,
    #[doc = "0xf88 - HWVAD control register"]
    pub hwvadst10: HWVADST10,
    #[doc = "0xf8c - HWVAD filter reset register"]
    pub hwvadrstt: HWVADRSTT,
    #[doc = "0xf90 - HWVAD noise estimator gain register"]
    pub hwvadthgn: HWVADTHGN,
    #[doc = "0xf94 - HWVAD signal estimator gain register"]
    pub hwvadthgs: HWVADTHGS,
    #[doc = "0xf98 - HWVAD noise envelope estimator register"]
    pub hwvadlowz: HWVADLOWZ,
    _reserved12: [u8; 0x60],
    #[doc = "0xffc - Module Identification register"]
    pub id: ID,
}
#[doc = "no description available"]
pub use channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod channel;
#[doc = "CHANEN (rw) register accessor: an alias for `Reg<CHANEN_SPEC>`"]
pub type CHANEN = crate::Reg<chanen::CHANEN_SPEC>;
#[doc = "Channel Enable register"]
pub mod chanen;
#[doc = "IOCFG (rw) register accessor: an alias for `Reg<IOCFG_SPEC>`"]
pub type IOCFG = crate::Reg<iocfg::IOCFG_SPEC>;
#[doc = "I/O Configuration register"]
pub mod iocfg;
#[doc = "USE2FS (rw) register accessor: an alias for `Reg<USE2FS_SPEC>`"]
pub type USE2FS = crate::Reg<use2fs::USE2FS_SPEC>;
#[doc = "Use 2FS register"]
pub mod use2fs;
#[doc = "HWVADGAIN (rw) register accessor: an alias for `Reg<HWVADGAIN_SPEC>`"]
pub type HWVADGAIN = crate::Reg<hwvadgain::HWVADGAIN_SPEC>;
#[doc = "HWVAD input gain register"]
pub mod hwvadgain;
#[doc = "HWVADHPFS (rw) register accessor: an alias for `Reg<HWVADHPFS_SPEC>`"]
pub type HWVADHPFS = crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>;
#[doc = "HWVAD filter control register"]
pub mod hwvadhpfs;
#[doc = "HWVADST10 (rw) register accessor: an alias for `Reg<HWVADST10_SPEC>`"]
pub type HWVADST10 = crate::Reg<hwvadst10::HWVADST10_SPEC>;
#[doc = "HWVAD control register"]
pub mod hwvadst10;
#[doc = "HWVADRSTT (rw) register accessor: an alias for `Reg<HWVADRSTT_SPEC>`"]
pub type HWVADRSTT = crate::Reg<hwvadrstt::HWVADRSTT_SPEC>;
#[doc = "HWVAD filter reset register"]
pub mod hwvadrstt;
#[doc = "HWVADTHGN (rw) register accessor: an alias for `Reg<HWVADTHGN_SPEC>`"]
pub type HWVADTHGN = crate::Reg<hwvadthgn::HWVADTHGN_SPEC>;
#[doc = "HWVAD noise estimator gain register"]
pub mod hwvadthgn;
#[doc = "HWVADTHGS (rw) register accessor: an alias for `Reg<HWVADTHGS_SPEC>`"]
pub type HWVADTHGS = crate::Reg<hwvadthgs::HWVADTHGS_SPEC>;
#[doc = "HWVAD signal estimator gain register"]
pub mod hwvadthgs;
#[doc = "HWVADLOWZ (r) register accessor: an alias for `Reg<HWVADLOWZ_SPEC>`"]
pub type HWVADLOWZ = crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>;
#[doc = "HWVAD noise envelope estimator register"]
pub mod hwvadlowz;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification register"]
pub mod id;
