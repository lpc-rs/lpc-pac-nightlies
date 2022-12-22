#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_0: B0_0,
    #[doc = "0x01 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_1: B0_1,
    #[doc = "0x02 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_2: B0_2,
    #[doc = "0x03 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_3: B0_3,
    #[doc = "0x04 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_4: B0_4,
    #[doc = "0x05 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_5: B0_5,
    #[doc = "0x06 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_6: B0_6,
    #[doc = "0x07 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_7: B0_7,
    #[doc = "0x08 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_8: B0_8,
    #[doc = "0x09 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_9: B0_9,
    #[doc = "0x0a - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_10: B0_10,
    #[doc = "0x0b - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_11: B0_11,
    #[doc = "0x0c - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_12: B0_12,
    #[doc = "0x0d - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_13: B0_13,
    #[doc = "0x0e - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_14: B0_14,
    #[doc = "0x0f - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_15: B0_15,
    #[doc = "0x10 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_16: B0_16,
    #[doc = "0x11 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_17: B0_17,
    _reserved18: [u8; 0x0fee],
    #[doc = "0x1000 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_0: W0_0,
    #[doc = "0x1004 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_1: W0_1,
    #[doc = "0x1008 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_2: W0_2,
    #[doc = "0x100c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_3: W0_3,
    #[doc = "0x1010 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_4: W0_4,
    #[doc = "0x1014 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_5: W0_5,
    #[doc = "0x1018 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_6: W0_6,
    #[doc = "0x101c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_7: W0_7,
    #[doc = "0x1020 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_8: W0_8,
    #[doc = "0x1024 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_9: W0_9,
    #[doc = "0x1028 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_10: W0_10,
    #[doc = "0x102c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_11: W0_11,
    #[doc = "0x1030 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_12: W0_12,
    #[doc = "0x1034 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_13: W0_13,
    #[doc = "0x1038 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_14: W0_14,
    #[doc = "0x103c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_15: W0_15,
    #[doc = "0x1040 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_16: W0_16,
    #[doc = "0x1044 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_17: W0_17,
    _reserved36: [u8; 0x0fb8],
    #[doc = "0x2000 - Direction registers"]
    pub dir0: DIR0,
    _reserved37: [u8; 0x7c],
    #[doc = "0x2080 - Mask register"]
    pub mask0: MASK0,
    _reserved38: [u8; 0x7c],
    #[doc = "0x2100 - Port pin register"]
    pub pin0: PIN0,
    _reserved39: [u8; 0x7c],
    #[doc = "0x2180 - Masked port register"]
    pub mpin0: MPIN0,
    _reserved40: [u8; 0x7c],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set0: SET0,
    _reserved41: [u8; 0x7c],
    #[doc = "0x2280 - Clear port"]
    pub clr0: CLR0,
    _reserved42: [u8; 0x7c],
    #[doc = "0x2300 - Toggle port"]
    pub not0: NOT0,
}
#[doc = "B0_0 (rw) register accessor: an alias for `Reg<B0_0_SPEC>`"]
pub type B0_0 = crate::Reg<b0_0::B0_0_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_0;
#[doc = "B0_1 (rw) register accessor: an alias for `Reg<B0_1_SPEC>`"]
pub type B0_1 = crate::Reg<b0_1::B0_1_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_1;
#[doc = "B0_2 (rw) register accessor: an alias for `Reg<B0_2_SPEC>`"]
pub type B0_2 = crate::Reg<b0_2::B0_2_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_2;
#[doc = "B0_3 (rw) register accessor: an alias for `Reg<B0_3_SPEC>`"]
pub type B0_3 = crate::Reg<b0_3::B0_3_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_3;
#[doc = "B0_4 (rw) register accessor: an alias for `Reg<B0_4_SPEC>`"]
pub type B0_4 = crate::Reg<b0_4::B0_4_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_4;
#[doc = "B0_5 (rw) register accessor: an alias for `Reg<B0_5_SPEC>`"]
pub type B0_5 = crate::Reg<b0_5::B0_5_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_5;
#[doc = "B0_6 (rw) register accessor: an alias for `Reg<B0_6_SPEC>`"]
pub type B0_6 = crate::Reg<b0_6::B0_6_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_6;
#[doc = "B0_7 (rw) register accessor: an alias for `Reg<B0_7_SPEC>`"]
pub type B0_7 = crate::Reg<b0_7::B0_7_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_7;
#[doc = "B0_8 (rw) register accessor: an alias for `Reg<B0_8_SPEC>`"]
pub type B0_8 = crate::Reg<b0_8::B0_8_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_8;
#[doc = "B0_9 (rw) register accessor: an alias for `Reg<B0_9_SPEC>`"]
pub type B0_9 = crate::Reg<b0_9::B0_9_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_9;
#[doc = "B0_10 (rw) register accessor: an alias for `Reg<B0_10_SPEC>`"]
pub type B0_10 = crate::Reg<b0_10::B0_10_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_10;
#[doc = "B0_11 (rw) register accessor: an alias for `Reg<B0_11_SPEC>`"]
pub type B0_11 = crate::Reg<b0_11::B0_11_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_11;
#[doc = "B0_12 (rw) register accessor: an alias for `Reg<B0_12_SPEC>`"]
pub type B0_12 = crate::Reg<b0_12::B0_12_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_12;
#[doc = "B0_13 (rw) register accessor: an alias for `Reg<B0_13_SPEC>`"]
pub type B0_13 = crate::Reg<b0_13::B0_13_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_13;
#[doc = "B0_14 (rw) register accessor: an alias for `Reg<B0_14_SPEC>`"]
pub type B0_14 = crate::Reg<b0_14::B0_14_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_14;
#[doc = "B0_15 (rw) register accessor: an alias for `Reg<B0_15_SPEC>`"]
pub type B0_15 = crate::Reg<b0_15::B0_15_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_15;
#[doc = "B0_16 (rw) register accessor: an alias for `Reg<B0_16_SPEC>`"]
pub type B0_16 = crate::Reg<b0_16::B0_16_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_16;
#[doc = "B0_17 (rw) register accessor: an alias for `Reg<B0_17_SPEC>`"]
pub type B0_17 = crate::Reg<b0_17::B0_17_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_17;
#[doc = "W0_0 (rw) register accessor: an alias for `Reg<W0_0_SPEC>`"]
pub type W0_0 = crate::Reg<w0_0::W0_0_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_0;
#[doc = "W0_1 (rw) register accessor: an alias for `Reg<W0_1_SPEC>`"]
pub type W0_1 = crate::Reg<w0_1::W0_1_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_1;
#[doc = "W0_2 (rw) register accessor: an alias for `Reg<W0_2_SPEC>`"]
pub type W0_2 = crate::Reg<w0_2::W0_2_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_2;
#[doc = "W0_3 (rw) register accessor: an alias for `Reg<W0_3_SPEC>`"]
pub type W0_3 = crate::Reg<w0_3::W0_3_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_3;
#[doc = "W0_4 (rw) register accessor: an alias for `Reg<W0_4_SPEC>`"]
pub type W0_4 = crate::Reg<w0_4::W0_4_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_4;
#[doc = "W0_5 (rw) register accessor: an alias for `Reg<W0_5_SPEC>`"]
pub type W0_5 = crate::Reg<w0_5::W0_5_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_5;
#[doc = "W0_6 (rw) register accessor: an alias for `Reg<W0_6_SPEC>`"]
pub type W0_6 = crate::Reg<w0_6::W0_6_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_6;
#[doc = "W0_7 (rw) register accessor: an alias for `Reg<W0_7_SPEC>`"]
pub type W0_7 = crate::Reg<w0_7::W0_7_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_7;
#[doc = "W0_8 (rw) register accessor: an alias for `Reg<W0_8_SPEC>`"]
pub type W0_8 = crate::Reg<w0_8::W0_8_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_8;
#[doc = "W0_9 (rw) register accessor: an alias for `Reg<W0_9_SPEC>`"]
pub type W0_9 = crate::Reg<w0_9::W0_9_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_9;
#[doc = "W0_10 (rw) register accessor: an alias for `Reg<W0_10_SPEC>`"]
pub type W0_10 = crate::Reg<w0_10::W0_10_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_10;
#[doc = "W0_11 (rw) register accessor: an alias for `Reg<W0_11_SPEC>`"]
pub type W0_11 = crate::Reg<w0_11::W0_11_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_11;
#[doc = "W0_12 (rw) register accessor: an alias for `Reg<W0_12_SPEC>`"]
pub type W0_12 = crate::Reg<w0_12::W0_12_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_12;
#[doc = "W0_13 (rw) register accessor: an alias for `Reg<W0_13_SPEC>`"]
pub type W0_13 = crate::Reg<w0_13::W0_13_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_13;
#[doc = "W0_14 (rw) register accessor: an alias for `Reg<W0_14_SPEC>`"]
pub type W0_14 = crate::Reg<w0_14::W0_14_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_14;
#[doc = "W0_15 (rw) register accessor: an alias for `Reg<W0_15_SPEC>`"]
pub type W0_15 = crate::Reg<w0_15::W0_15_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_15;
#[doc = "W0_16 (rw) register accessor: an alias for `Reg<W0_16_SPEC>`"]
pub type W0_16 = crate::Reg<w0_16::W0_16_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_16;
#[doc = "W0_17 (rw) register accessor: an alias for `Reg<W0_17_SPEC>`"]
pub type W0_17 = crate::Reg<w0_17::W0_17_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_17;
#[doc = "DIR0 (rw) register accessor: an alias for `Reg<DIR0_SPEC>`"]
pub type DIR0 = crate::Reg<dir0::DIR0_SPEC>;
#[doc = "Direction registers"]
pub mod dir0;
#[doc = "MASK0 (rw) register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Mask register"]
pub mod mask0;
#[doc = "PIN0 (rw) register accessor: an alias for `Reg<PIN0_SPEC>`"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "Port pin register"]
pub mod pin0;
#[doc = "MPIN0 (rw) register accessor: an alias for `Reg<MPIN0_SPEC>`"]
pub type MPIN0 = crate::Reg<mpin0::MPIN0_SPEC>;
#[doc = "Masked port register"]
pub mod mpin0;
#[doc = "SET0 (rw) register accessor: an alias for `Reg<SET0_SPEC>`"]
pub type SET0 = crate::Reg<set0::SET0_SPEC>;
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set0;
#[doc = "CLR0 (w) register accessor: an alias for `Reg<CLR0_SPEC>`"]
pub type CLR0 = crate::Reg<clr0::CLR0_SPEC>;
#[doc = "Clear port"]
pub mod clr0;
#[doc = "NOT0 (w) register accessor: an alias for `Reg<NOT0_SPEC>`"]
pub type NOT0 = crate::Reg<not0::NOT0_SPEC>;
#[doc = "Toggle port"]
pub mod not0;
