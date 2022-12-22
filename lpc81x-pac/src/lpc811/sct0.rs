#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register"]
    pub start: START,
    _reserved6: [u8; 0x28],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    _reserved13: [u8; 0x94],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    _reserved_17_cap_match: [u8; 0x04],
    _reserved_18_cap_match: [u8; 0x04],
    _reserved_19_cap_match: [u8; 0x04],
    _reserved_20_cap_match: [u8; 0x04],
    _reserved_21_cap_match: [u8; 0x04],
    _reserved22: [u8; 0xec],
    _reserved_22_capctrl_matchrel: [u8; 0x04],
    _reserved_23_capctrl_matchrel: [u8; 0x04],
    _reserved_24_capctrl_matchrel: [u8; 0x04],
    _reserved_25_capctrl_matchrel: [u8; 0x04],
    _reserved_26_capctrl_matchrel: [u8; 0x04],
    _reserved27: [u8; 0xec],
    #[doc = "0x300..0x330 - no description available"]
    pub ev: [EV; 6],
    _reserved28: [u8; 0x01d0],
    #[doc = "0x500..0x520 - no description available"]
    pub out: [OUT; 4],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_match0(&self) -> &CAP_MATCH_MATCH0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CAP_MATCH_MATCH0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_cap0(&self) -> &CAP_MATCH_CAP0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CAP_MATCH_CAP0) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_match1(&self) -> &CAP_MATCH_MATCH1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CAP_MATCH_MATCH1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_cap1(&self) -> &CAP_MATCH_CAP1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CAP_MATCH_CAP1) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_match2(&self) -> &CAP_MATCH_MATCH2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CAP_MATCH_MATCH2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_cap2(&self) -> &CAP_MATCH_CAP2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CAP_MATCH_CAP2) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_match3(&self) -> &CAP_MATCH_MATCH3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CAP_MATCH_MATCH3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_cap3(&self) -> &CAP_MATCH_CAP3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CAP_MATCH_CAP3) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_match4(&self) -> &CAP_MATCH_MATCH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const CAP_MATCH_MATCH4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_cap4(&self) -> &CAP_MATCH_CAP4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const CAP_MATCH_CAP4) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel0(&self) -> &CAPCTRL_MATCHREL_MATCHREL0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const CAPCTRL_MATCHREL_MATCHREL0)
        }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl0(&self) -> &CAPCTRL_MATCHREL_CAPCTRL0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const CAPCTRL_MATCHREL_CAPCTRL0)
        }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel1(&self) -> &CAPCTRL_MATCHREL_MATCHREL1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const CAPCTRL_MATCHREL_MATCHREL1)
        }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl1(&self) -> &CAPCTRL_MATCHREL_CAPCTRL1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const CAPCTRL_MATCHREL_CAPCTRL1)
        }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel2(&self) -> &CAPCTRL_MATCHREL_MATCHREL2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const CAPCTRL_MATCHREL_MATCHREL2)
        }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl2(&self) -> &CAPCTRL_MATCHREL_CAPCTRL2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const CAPCTRL_MATCHREL_CAPCTRL2)
        }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel3(&self) -> &CAPCTRL_MATCHREL_MATCHREL3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const CAPCTRL_MATCHREL_MATCHREL3)
        }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl3(&self) -> &CAPCTRL_MATCHREL_CAPCTRL3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const CAPCTRL_MATCHREL_CAPCTRL3)
        }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel4(&self) -> &CAPCTRL_MATCHREL_MATCHREL4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const CAPCTRL_MATCHREL_MATCHREL4)
        }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl4(&self) -> &CAPCTRL_MATCHREL_CAPCTRL4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const CAPCTRL_MATCHREL_CAPCTRL4)
        }
    }
}
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "HALT (rw) register accessor: an alias for `Reg<HALT_SPEC>`"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "STOP (rw) register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "SCT counter register"]
pub mod count;
#[doc = "STATE (rw) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "SCT state register"]
pub mod state;
#[doc = "INPUT (rw) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "SCT input register"]
pub mod input;
#[doc = "REGMODE (rw) register accessor: an alias for `Reg<REGMODE_SPEC>`"]
pub type REGMODE = crate::Reg<regmode::REGMODE_SPEC>;
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "OUTPUT (rw) register accessor: an alias for `Reg<OUTPUT_SPEC>`"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "SCT output register"]
pub mod output;
#[doc = "OUTPUTDIRCTRL (rw) register accessor: an alias for `Reg<OUTPUTDIRCTRL_SPEC>`"]
pub type OUTPUTDIRCTRL = crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>;
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "RES (rw) register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "EVEN (rw) register accessor: an alias for `Reg<EVEN_SPEC>`"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "EVFLAG (rw) register accessor: an alias for `Reg<EVFLAG_SPEC>`"]
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "CONEN (rw) register accessor: an alias for `Reg<CONEN_SPEC>`"]
pub type CONEN = crate::Reg<conen::CONEN_SPEC>;
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "CONFLAG (rw) register accessor: an alias for `Reg<CONFLAG_SPEC>`"]
pub type CONFLAG = crate::Reg<conflag::CONFLAG_SPEC>;
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "CAP_MATCH_CAP0 (rw) register accessor: an alias for `Reg<CAP_MATCH_CAP0_SPEC>`"]
pub type CAP_MATCH_CAP0 = crate::Reg<cap_match_cap0::CAP_MATCH_CAP0_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap0;
#[doc = "CAP_MATCH_MATCH0 (rw) register accessor: an alias for `Reg<CAP_MATCH_MATCH0_SPEC>`"]
pub type CAP_MATCH_MATCH0 = crate::Reg<cap_match_match0::CAP_MATCH_MATCH0_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match0;
#[doc = "CAP_MATCH_CAP1 (rw) register accessor: an alias for `Reg<CAP_MATCH_CAP1_SPEC>`"]
pub type CAP_MATCH_CAP1 = crate::Reg<cap_match_cap1::CAP_MATCH_CAP1_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap1;
#[doc = "CAP_MATCH_MATCH1 (rw) register accessor: an alias for `Reg<CAP_MATCH_MATCH1_SPEC>`"]
pub type CAP_MATCH_MATCH1 = crate::Reg<cap_match_match1::CAP_MATCH_MATCH1_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match1;
#[doc = "CAP_MATCH_CAP2 (rw) register accessor: an alias for `Reg<CAP_MATCH_CAP2_SPEC>`"]
pub type CAP_MATCH_CAP2 = crate::Reg<cap_match_cap2::CAP_MATCH_CAP2_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap2;
#[doc = "CAP_MATCH_MATCH2 (rw) register accessor: an alias for `Reg<CAP_MATCH_MATCH2_SPEC>`"]
pub type CAP_MATCH_MATCH2 = crate::Reg<cap_match_match2::CAP_MATCH_MATCH2_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match2;
#[doc = "CAP_MATCH_CAP3 (rw) register accessor: an alias for `Reg<CAP_MATCH_CAP3_SPEC>`"]
pub type CAP_MATCH_CAP3 = crate::Reg<cap_match_cap3::CAP_MATCH_CAP3_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap3;
#[doc = "CAP_MATCH_MATCH3 (rw) register accessor: an alias for `Reg<CAP_MATCH_MATCH3_SPEC>`"]
pub type CAP_MATCH_MATCH3 = crate::Reg<cap_match_match3::CAP_MATCH_MATCH3_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match3;
#[doc = "CAP_MATCH_CAP4 (rw) register accessor: an alias for `Reg<CAP_MATCH_CAP4_SPEC>`"]
pub type CAP_MATCH_CAP4 = crate::Reg<cap_match_cap4::CAP_MATCH_CAP4_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap4;
#[doc = "CAP_MATCH_MATCH4 (rw) register accessor: an alias for `Reg<CAP_MATCH_MATCH4_SPEC>`"]
pub type CAP_MATCH_MATCH4 = crate::Reg<cap_match_match4::CAP_MATCH_MATCH4_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match4;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL0 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL0_SPEC>`"]
pub type CAPCTRL_MATCHREL_CAPCTRL0 =
    crate::Reg<capctrl_matchrel_capctrl0::CAPCTRL_MATCHREL_CAPCTRL0_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl0;
#[doc = "CAPCTRL_MATCHREL_MATCHREL0 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL0_SPEC>`"]
pub type CAPCTRL_MATCHREL_MATCHREL0 =
    crate::Reg<capctrl_matchrel_matchrel0::CAPCTRL_MATCHREL_MATCHREL0_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel0;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL1 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL1_SPEC>`"]
pub type CAPCTRL_MATCHREL_CAPCTRL1 =
    crate::Reg<capctrl_matchrel_capctrl1::CAPCTRL_MATCHREL_CAPCTRL1_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl1;
#[doc = "CAPCTRL_MATCHREL_MATCHREL1 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL1_SPEC>`"]
pub type CAPCTRL_MATCHREL_MATCHREL1 =
    crate::Reg<capctrl_matchrel_matchrel1::CAPCTRL_MATCHREL_MATCHREL1_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel1;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL2 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL2_SPEC>`"]
pub type CAPCTRL_MATCHREL_CAPCTRL2 =
    crate::Reg<capctrl_matchrel_capctrl2::CAPCTRL_MATCHREL_CAPCTRL2_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl2;
#[doc = "CAPCTRL_MATCHREL_MATCHREL2 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL2_SPEC>`"]
pub type CAPCTRL_MATCHREL_MATCHREL2 =
    crate::Reg<capctrl_matchrel_matchrel2::CAPCTRL_MATCHREL_MATCHREL2_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel2;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL3 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL3_SPEC>`"]
pub type CAPCTRL_MATCHREL_CAPCTRL3 =
    crate::Reg<capctrl_matchrel_capctrl3::CAPCTRL_MATCHREL_CAPCTRL3_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl3;
#[doc = "CAPCTRL_MATCHREL_MATCHREL3 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL3_SPEC>`"]
pub type CAPCTRL_MATCHREL_MATCHREL3 =
    crate::Reg<capctrl_matchrel_matchrel3::CAPCTRL_MATCHREL_MATCHREL3_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel3;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL4 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL4_SPEC>`"]
pub type CAPCTRL_MATCHREL_CAPCTRL4 =
    crate::Reg<capctrl_matchrel_capctrl4::CAPCTRL_MATCHREL_CAPCTRL4_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl4;
#[doc = "CAPCTRL_MATCHREL_MATCHREL4 (rw) register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL4_SPEC>`"]
pub type CAPCTRL_MATCHREL_MATCHREL4 =
    crate::Reg<capctrl_matchrel_matchrel4::CAPCTRL_MATCHREL_MATCHREL4_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel4;
#[doc = "no description available"]
pub use ev::EV;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod ev;
#[doc = "no description available"]
pub use out::OUT;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod out;
