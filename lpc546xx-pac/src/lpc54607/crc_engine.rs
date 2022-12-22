#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode"]
    pub mode: MODE,
    #[doc = "0x04 - CRC seed"]
    pub seed: SEED,
    _reserved_2_sum: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x08 - Data register - word sized"]
    #[inline(always)]
    pub fn data32(&self) -> &DATA32 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const DATA32) }
    }
    #[doc = "0x08 - Data register - half-word sized"]
    #[inline(always)]
    pub fn data16(&self) -> &DATA16 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const DATA16) }
    }
    #[doc = "0x08 - Data register - byte sized"]
    #[inline(always)]
    pub fn data8(&self) -> &DATA8 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const DATA8) }
    }
    #[doc = "0x08 - Data written to this register will be taken to perform CRC calculation with selected bit order and 1’s complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const DATA) }
    }
    #[doc = "0x08 - The most recent CRC sum can be read through this register with selected bit order and 1’s complement post-processes."]
    #[inline(always)]
    pub fn sum(&self) -> &SUM {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const SUM) }
    }
}
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "CRC mode"]
pub mod mode;
#[doc = "SEED (rw) register accessor: an alias for `Reg<SEED_SPEC>`"]
pub type SEED = crate::Reg<seed::SEED_SPEC>;
#[doc = "CRC seed"]
pub mod seed;
#[doc = "SUM (r) register accessor: an alias for `Reg<SUM_SPEC>`"]
pub type SUM = crate::Reg<sum::SUM_SPEC>;
#[doc = "The most recent CRC sum can be read through this register with selected bit order and 1’s complement post-processes."]
pub mod sum;
#[doc = "DATA (w) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1’s complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
pub mod data;
#[doc = "DATA8 (w) register accessor: an alias for `Reg<DATA8_SPEC>`"]
pub type DATA8 = crate::Reg<data8::DATA8_SPEC>;
#[doc = "Data register - byte sized"]
pub mod data8;
#[doc = "DATA16 (w) register accessor: an alias for `Reg<DATA16_SPEC>`"]
pub type DATA16 = crate::Reg<data16::DATA16_SPEC>;
#[doc = "Data register - half-word sized"]
pub mod data16;
#[doc = "DATA32 (w) register accessor: an alias for `Reg<DATA32_SPEC>`"]
pub type DATA32 = crate::Reg<data32::DATA32_SPEC>;
#[doc = "Data register - word sized"]
pub mod data32;
