#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for pins PIO0_17"]
    pub pio0_17: PIO0_17,
    #[doc = "0x04 - Digital I/O control for pins PIO0_13"]
    pub pio0_13: PIO0_13,
    #[doc = "0x08 - Digital I/O control for pins PIO0_12"]
    pub pio0_12: PIO0_12,
    #[doc = "0x0c - Digital I/O control for pins PIO0_5"]
    pub pio0_5: PIO0_5,
    #[doc = "0x10 - Digital I/O control for pins PIO0_4"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - Digital I/O control for pins PIO0_3"]
    pub pio0_3: PIO0_3,
    #[doc = "0x18 - Digital I/O control for pins PIO0_2"]
    pub pio0_2: PIO0_2,
    #[doc = "0x1c - Digital I/O control for pins PIO0_11"]
    pub pio0_11: PIO0_11,
    #[doc = "0x20 - Digital I/O control for pins PIO0_10"]
    pub pio0_10: PIO0_10,
    #[doc = "0x24 - Digital I/O control for pins PIO0_16"]
    pub pio0_16: PIO0_16,
    #[doc = "0x28 - Digital I/O control for pins PIO0_15"]
    pub pio0_15: PIO0_15,
    #[doc = "0x2c - Digital I/O control for pins PIO0_1"]
    pub pio0_1: PIO0_1,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Digital I/O control for pins PIO0_9"]
    pub pio0_9: PIO0_9,
    #[doc = "0x38 - Digital I/O control for pins PIO0_8"]
    pub pio0_8: PIO0_8,
    #[doc = "0x3c - Digital I/O control for pins PIO0_7"]
    pub pio0_7: PIO0_7,
    #[doc = "0x40 - Digital I/O control for pins PIO0_6"]
    pub pio0_6: PIO0_6,
    #[doc = "0x44 - Digital I/O control for pins PIO0_0"]
    pub pio0_0: PIO0_0,
    #[doc = "0x48 - Digital I/O control for pins PIO0_14"]
    pub pio0_14: PIO0_14,
}
#[doc = "PIO0_17 (rw) register accessor: an alias for `Reg<PIO0_17_SPEC>`"]
pub type PIO0_17 = crate::Reg<pio0_17::PIO0_17_SPEC>;
#[doc = "Digital I/O control for pins PIO0_17"]
pub mod pio0_17;
#[doc = "PIO0_13 (rw) register accessor: an alias for `Reg<PIO0_13_SPEC>`"]
pub type PIO0_13 = crate::Reg<pio0_13::PIO0_13_SPEC>;
#[doc = "Digital I/O control for pins PIO0_13"]
pub mod pio0_13;
#[doc = "PIO0_12 (rw) register accessor: an alias for `Reg<PIO0_12_SPEC>`"]
pub type PIO0_12 = crate::Reg<pio0_12::PIO0_12_SPEC>;
#[doc = "Digital I/O control for pins PIO0_12"]
pub mod pio0_12;
#[doc = "PIO0_5 (rw) register accessor: an alias for `Reg<PIO0_5_SPEC>`"]
pub type PIO0_5 = crate::Reg<pio0_5::PIO0_5_SPEC>;
#[doc = "Digital I/O control for pins PIO0_5"]
pub mod pio0_5;
#[doc = "PIO0_4 (rw) register accessor: an alias for `Reg<PIO0_4_SPEC>`"]
pub type PIO0_4 = crate::Reg<pio0_4::PIO0_4_SPEC>;
#[doc = "Digital I/O control for pins PIO0_4"]
pub mod pio0_4;
#[doc = "PIO0_3 (rw) register accessor: an alias for `Reg<PIO0_3_SPEC>`"]
pub type PIO0_3 = crate::Reg<pio0_3::PIO0_3_SPEC>;
#[doc = "Digital I/O control for pins PIO0_3"]
pub mod pio0_3;
#[doc = "PIO0_2 (rw) register accessor: an alias for `Reg<PIO0_2_SPEC>`"]
pub type PIO0_2 = crate::Reg<pio0_2::PIO0_2_SPEC>;
#[doc = "Digital I/O control for pins PIO0_2"]
pub mod pio0_2;
#[doc = "PIO0_11 (rw) register accessor: an alias for `Reg<PIO0_11_SPEC>`"]
pub type PIO0_11 = crate::Reg<pio0_11::PIO0_11_SPEC>;
#[doc = "Digital I/O control for pins PIO0_11"]
pub mod pio0_11;
#[doc = "PIO0_10 (rw) register accessor: an alias for `Reg<PIO0_10_SPEC>`"]
pub type PIO0_10 = crate::Reg<pio0_10::PIO0_10_SPEC>;
#[doc = "Digital I/O control for pins PIO0_10"]
pub mod pio0_10;
#[doc = "PIO0_16 (rw) register accessor: an alias for `Reg<PIO0_16_SPEC>`"]
pub type PIO0_16 = crate::Reg<pio0_16::PIO0_16_SPEC>;
#[doc = "Digital I/O control for pins PIO0_16"]
pub mod pio0_16;
#[doc = "PIO0_15 (rw) register accessor: an alias for `Reg<PIO0_15_SPEC>`"]
pub type PIO0_15 = crate::Reg<pio0_15::PIO0_15_SPEC>;
#[doc = "Digital I/O control for pins PIO0_15"]
pub mod pio0_15;
#[doc = "PIO0_1 (rw) register accessor: an alias for `Reg<PIO0_1_SPEC>`"]
pub type PIO0_1 = crate::Reg<pio0_1::PIO0_1_SPEC>;
#[doc = "Digital I/O control for pins PIO0_1"]
pub mod pio0_1;
#[doc = "PIO0_9 (rw) register accessor: an alias for `Reg<PIO0_9_SPEC>`"]
pub type PIO0_9 = crate::Reg<pio0_9::PIO0_9_SPEC>;
#[doc = "Digital I/O control for pins PIO0_9"]
pub mod pio0_9;
#[doc = "PIO0_8 (rw) register accessor: an alias for `Reg<PIO0_8_SPEC>`"]
pub type PIO0_8 = crate::Reg<pio0_8::PIO0_8_SPEC>;
#[doc = "Digital I/O control for pins PIO0_8"]
pub mod pio0_8;
#[doc = "PIO0_7 (rw) register accessor: an alias for `Reg<PIO0_7_SPEC>`"]
pub type PIO0_7 = crate::Reg<pio0_7::PIO0_7_SPEC>;
#[doc = "Digital I/O control for pins PIO0_7"]
pub mod pio0_7;
#[doc = "PIO0_6 (rw) register accessor: an alias for `Reg<PIO0_6_SPEC>`"]
pub type PIO0_6 = crate::Reg<pio0_6::PIO0_6_SPEC>;
#[doc = "Digital I/O control for pins PIO0_6"]
pub mod pio0_6;
#[doc = "PIO0_0 (rw) register accessor: an alias for `Reg<PIO0_0_SPEC>`"]
pub type PIO0_0 = crate::Reg<pio0_0::PIO0_0_SPEC>;
#[doc = "Digital I/O control for pins PIO0_0"]
pub mod pio0_0;
#[doc = "PIO0_14 (rw) register accessor: an alias for `Reg<PIO0_14_SPEC>`"]
pub type PIO0_14 = crate::Reg<pio0_14::PIO0_14_SPEC>;
#[doc = "Digital I/O control for pins PIO0_14"]
pub mod pio0_14;
