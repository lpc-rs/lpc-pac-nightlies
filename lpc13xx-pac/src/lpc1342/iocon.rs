#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin PIO2_6"]
    pub pio2_6: PIO2_6,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - I/O configuration for pin PIO2_0/DTR/SSEL1"]
    pub pio2_0: PIO2_0,
    #[doc = "0x0c - I/O configuration for pin RESET/PIO0_0"]
    pub reset_pio0_0: RESET_PIO0_0,
    #[doc = "0x10 - I/O configuration for pin PIO0_1/CLKOUT/ CT32B0_MAT2/USB_FTOGGLE"]
    pub pio0_1: PIO0_1,
    #[doc = "0x14 - I/O configuration for pin PIO1_8/CT16B1_CAP0"]
    pub pio1_8: PIO1_8,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - I/O configuration for pin PIO0_2/SSEL0/ CT16B0_CAP0"]
    pub pio0_2: PIO0_2,
    #[doc = "0x20 - I/O configuration for pin PIO2_7"]
    pub pio2_7: PIO2_7,
    #[doc = "0x24 - I/O configuration for pin PIO2_8"]
    pub pio2_8: PIO2_8,
    #[doc = "0x28 - I/O configuration for pin PIO2_1/DSR/SCK1"]
    pub pio2_1: PIO2_1,
    #[doc = "0x2c - I/O configuration for pin PIO0_3/USB_VBUS"]
    pub pio0_3: PIO0_3,
    #[doc = "0x30 - I/O configuration for pin PIO0_4/SCL"]
    pub pio0_4: PIO0_4,
    #[doc = "0x34 - I/O configuration for pin PIO0_5/SDA"]
    pub pio0_5: PIO0_5,
    #[doc = "0x38 - I/O configuration for pin PIO1_9/CT16B1_MAT0"]
    pub pio1_9: PIO1_9,
    #[doc = "0x3c - I/O configuration for pin PIO3_4"]
    pub pio3_4: PIO3_4,
    #[doc = "0x40 - I/O configuration for pin PIO2_4"]
    pub pio2_4: PIO2_4,
    #[doc = "0x44 - I/O configuration for pin PIO2_5"]
    pub pio2_5: PIO2_5,
    #[doc = "0x48 - I/O configuration for pin PIO3_5"]
    pub pio3_5: PIO3_5,
    #[doc = "0x4c - I/O configuration for pin PIO0_6/USB_CONNECT/SCK"]
    pub pio0_6: PIO0_6,
    #[doc = "0x50 - I/O configuration for pin PIO0_7/CTS"]
    pub pio0_7: PIO0_7,
    #[doc = "0x54 - I/O configuration for pin PIO2_9"]
    pub pio2_9: PIO2_9,
    #[doc = "0x58 - I/O configuration for pin PIO2_10"]
    pub pio2_10: PIO2_10,
    #[doc = "0x5c - I/O configuration for pin PIO2_2/DCD/MISO1"]
    pub pio2_2: PIO2_2,
    #[doc = "0x60 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    pub pio0_8: PIO0_8,
    #[doc = "0x64 - I/O configuration for pin PIO0_9/MOSI0/ CT16B0_MAT1/SWO"]
    pub pio0_9: PIO0_9,
    #[doc = "0x68 - I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2"]
    pub swclk_pio0_10: SWCLK_PIO0_10,
    #[doc = "0x6c - I/O configuration for pin PIO1_10/AD6/ CT16B1_MAT1"]
    pub pio1_10: PIO1_10,
    #[doc = "0x70 - I/O configuration for pin PIO2_11/SCK"]
    pub pio2_11: PIO2_11,
    #[doc = "0x74 - I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
    pub r_pio0_11: R_PIO0_11,
    #[doc = "0x78 - I/O configuration for pin R/PIO1_0/AD1/ CT32B1_CAP0"]
    pub r_pio1_0: R_PIO1_0,
    #[doc = "0x7c - I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
    pub r_pio1_1: R_PIO1_1,
    #[doc = "0x80 - I/O configuration for pin R/PIO1_2/AD3/ CT32B1_MAT1"]
    pub r_pio1_2: R_PIO1_2,
    #[doc = "0x84 - I/O configuration for pin PIO3_0/DTR"]
    pub pio3_0: PIO3_0,
    #[doc = "0x88 - I/O configuration for pin PIO3_1/DSR"]
    pub pio3_1: PIO3_1,
    #[doc = "0x8c - I/O configuration for pin PIO2_3/RI/MOSI1"]
    pub pio2_3: PIO2_3,
    #[doc = "0x90 - I/O configuration for pin SWDIO/PIO1_3/AD4/ CT32B1_MAT2"]
    pub swdio_pio1_3: SWDIO_PIO1_3,
    #[doc = "0x94 - I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
    pub pio1_4: PIO1_4,
    #[doc = "0x98 - I/O configuration for pin PIO1_11/AD7"]
    pub pio1_11: PIO1_11,
    #[doc = "0x9c - I/O configuration for pin PIO3_2/DCD"]
    pub pio3_2: PIO3_2,
    #[doc = "0xa0 - I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
    pub pio1_5: PIO1_5,
    #[doc = "0xa4 - I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
    pub pio1_6: PIO1_6,
    #[doc = "0xa8 - I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
    pub pio1_7: PIO1_7,
    #[doc = "0xac - I/O configuration for pin PIO3_3/RI"]
    pub pio3_3: PIO3_3,
    #[doc = "0xb0 - SCK0 pin location register"]
    pub sck0_loc: SCK0_LOC,
    #[doc = "0xb4 - DSR pin location select register"]
    pub dsr_loc: DSR_LOC,
    #[doc = "0xb8 - DCD pin location select register"]
    pub dcd_loc: DCD_LOC,
    #[doc = "0xbc - RI pin location register"]
    pub ri_loc: RI_LOC,
}
#[doc = "PIO2_6 (rw) register accessor: an alias for `Reg<PIO2_6_SPEC>`"]
pub type PIO2_6 = crate::Reg<pio2_6::PIO2_6_SPEC>;
#[doc = "I/O configuration for pin PIO2_6"]
pub mod pio2_6;
#[doc = "PIO2_0 (rw) register accessor: an alias for `Reg<PIO2_0_SPEC>`"]
pub type PIO2_0 = crate::Reg<pio2_0::PIO2_0_SPEC>;
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub mod pio2_0;
#[doc = "RESET_PIO0_0 (rw) register accessor: an alias for `Reg<RESET_PIO0_0_SPEC>`"]
pub type RESET_PIO0_0 = crate::Reg<reset_pio0_0::RESET_PIO0_0_SPEC>;
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "PIO0_1 (rw) register accessor: an alias for `Reg<PIO0_1_SPEC>`"]
pub type PIO0_1 = crate::Reg<pio0_1::PIO0_1_SPEC>;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/ CT32B0_MAT2/USB_FTOGGLE"]
pub mod pio0_1;
#[doc = "PIO1_8 (rw) register accessor: an alias for `Reg<PIO1_8_SPEC>`"]
pub type PIO1_8 = crate::Reg<pio1_8::PIO1_8_SPEC>;
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub mod pio1_8;
#[doc = "PIO0_2 (rw) register accessor: an alias for `Reg<PIO0_2_SPEC>`"]
pub type PIO0_2 = crate::Reg<pio0_2::PIO0_2_SPEC>;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/ CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "PIO2_7 (rw) register accessor: an alias for `Reg<PIO2_7_SPEC>`"]
pub type PIO2_7 = crate::Reg<pio2_7::PIO2_7_SPEC>;
#[doc = "I/O configuration for pin PIO2_7"]
pub mod pio2_7;
#[doc = "PIO2_8 (rw) register accessor: an alias for `Reg<PIO2_8_SPEC>`"]
pub type PIO2_8 = crate::Reg<pio2_8::PIO2_8_SPEC>;
#[doc = "I/O configuration for pin PIO2_8"]
pub mod pio2_8;
#[doc = "PIO2_1 (rw) register accessor: an alias for `Reg<PIO2_1_SPEC>`"]
pub type PIO2_1 = crate::Reg<pio2_1::PIO2_1_SPEC>;
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub mod pio2_1;
#[doc = "PIO0_3 (rw) register accessor: an alias for `Reg<PIO0_3_SPEC>`"]
pub type PIO0_3 = crate::Reg<pio0_3::PIO0_3_SPEC>;
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub mod pio0_3;
#[doc = "PIO0_4 (rw) register accessor: an alias for `Reg<PIO0_4_SPEC>`"]
pub type PIO0_4 = crate::Reg<pio0_4::PIO0_4_SPEC>;
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "PIO0_5 (rw) register accessor: an alias for `Reg<PIO0_5_SPEC>`"]
pub type PIO0_5 = crate::Reg<pio0_5::PIO0_5_SPEC>;
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "PIO1_9 (rw) register accessor: an alias for `Reg<PIO1_9_SPEC>`"]
pub type PIO1_9 = crate::Reg<pio1_9::PIO1_9_SPEC>;
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0"]
pub mod pio1_9;
#[doc = "PIO3_4 (rw) register accessor: an alias for `Reg<PIO3_4_SPEC>`"]
pub type PIO3_4 = crate::Reg<pio3_4::PIO3_4_SPEC>;
#[doc = "I/O configuration for pin PIO3_4"]
pub mod pio3_4;
#[doc = "PIO2_4 (rw) register accessor: an alias for `Reg<PIO2_4_SPEC>`"]
pub type PIO2_4 = crate::Reg<pio2_4::PIO2_4_SPEC>;
#[doc = "I/O configuration for pin PIO2_4"]
pub mod pio2_4;
#[doc = "PIO2_5 (rw) register accessor: an alias for `Reg<PIO2_5_SPEC>`"]
pub type PIO2_5 = crate::Reg<pio2_5::PIO2_5_SPEC>;
#[doc = "I/O configuration for pin PIO2_5"]
pub mod pio2_5;
#[doc = "PIO3_5 (rw) register accessor: an alias for `Reg<PIO3_5_SPEC>`"]
pub type PIO3_5 = crate::Reg<pio3_5::PIO3_5_SPEC>;
#[doc = "I/O configuration for pin PIO3_5"]
pub mod pio3_5;
#[doc = "PIO0_6 (rw) register accessor: an alias for `Reg<PIO0_6_SPEC>`"]
pub type PIO0_6 = crate::Reg<pio0_6::PIO0_6_SPEC>;
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK"]
pub mod pio0_6;
#[doc = "PIO0_7 (rw) register accessor: an alias for `Reg<PIO0_7_SPEC>`"]
pub type PIO0_7 = crate::Reg<pio0_7::PIO0_7_SPEC>;
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "PIO2_9 (rw) register accessor: an alias for `Reg<PIO2_9_SPEC>`"]
pub type PIO2_9 = crate::Reg<pio2_9::PIO2_9_SPEC>;
#[doc = "I/O configuration for pin PIO2_9"]
pub mod pio2_9;
#[doc = "PIO2_10 (rw) register accessor: an alias for `Reg<PIO2_10_SPEC>`"]
pub type PIO2_10 = crate::Reg<pio2_10::PIO2_10_SPEC>;
#[doc = "I/O configuration for pin PIO2_10"]
pub mod pio2_10;
#[doc = "PIO2_2 (rw) register accessor: an alias for `Reg<PIO2_2_SPEC>`"]
pub type PIO2_2 = crate::Reg<pio2_2::PIO2_2_SPEC>;
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub mod pio2_2;
#[doc = "PIO0_8 (rw) register accessor: an alias for `Reg<PIO0_8_SPEC>`"]
pub type PIO0_8 = crate::Reg<pio0_8::PIO0_8_SPEC>;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "PIO0_9 (rw) register accessor: an alias for `Reg<PIO0_9_SPEC>`"]
pub type PIO0_9 = crate::Reg<pio0_9::PIO0_9_SPEC>;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/ CT16B0_MAT1/SWO"]
pub mod pio0_9;
#[doc = "SWCLK_PIO0_10 (rw) register accessor: an alias for `Reg<SWCLK_PIO0_10_SPEC>`"]
pub type SWCLK_PIO0_10 = crate::Reg<swclk_pio0_10::SWCLK_PIO0_10_SPEC>;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "PIO1_10 (rw) register accessor: an alias for `Reg<PIO1_10_SPEC>`"]
pub type PIO1_10 = crate::Reg<pio1_10::PIO1_10_SPEC>;
#[doc = "I/O configuration for pin PIO1_10/AD6/ CT16B1_MAT1"]
pub mod pio1_10;
#[doc = "PIO2_11 (rw) register accessor: an alias for `Reg<PIO2_11_SPEC>`"]
pub type PIO2_11 = crate::Reg<pio2_11::PIO2_11_SPEC>;
#[doc = "I/O configuration for pin PIO2_11/SCK"]
pub mod pio2_11;
#[doc = "R_PIO0_11 (rw) register accessor: an alias for `Reg<R_PIO0_11_SPEC>`"]
pub type R_PIO0_11 = crate::Reg<r_pio0_11::R_PIO0_11_SPEC>;
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub mod r_pio0_11;
#[doc = "R_PIO1_0 (rw) register accessor: an alias for `Reg<R_PIO1_0_SPEC>`"]
pub type R_PIO1_0 = crate::Reg<r_pio1_0::R_PIO1_0_SPEC>;
#[doc = "I/O configuration for pin R/PIO1_0/AD1/ CT32B1_CAP0"]
pub mod r_pio1_0;
#[doc = "R_PIO1_1 (rw) register accessor: an alias for `Reg<R_PIO1_1_SPEC>`"]
pub type R_PIO1_1 = crate::Reg<r_pio1_1::R_PIO1_1_SPEC>;
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub mod r_pio1_1;
#[doc = "R_PIO1_2 (rw) register accessor: an alias for `Reg<R_PIO1_2_SPEC>`"]
pub type R_PIO1_2 = crate::Reg<r_pio1_2::R_PIO1_2_SPEC>;
#[doc = "I/O configuration for pin R/PIO1_2/AD3/ CT32B1_MAT1"]
pub mod r_pio1_2;
#[doc = "PIO3_0 (rw) register accessor: an alias for `Reg<PIO3_0_SPEC>`"]
pub type PIO3_0 = crate::Reg<pio3_0::PIO3_0_SPEC>;
#[doc = "I/O configuration for pin PIO3_0/DTR"]
pub mod pio3_0;
#[doc = "PIO3_1 (rw) register accessor: an alias for `Reg<PIO3_1_SPEC>`"]
pub type PIO3_1 = crate::Reg<pio3_1::PIO3_1_SPEC>;
#[doc = "I/O configuration for pin PIO3_1/DSR"]
pub mod pio3_1;
#[doc = "PIO2_3 (rw) register accessor: an alias for `Reg<PIO2_3_SPEC>`"]
pub type PIO2_3 = crate::Reg<pio2_3::PIO2_3_SPEC>;
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub mod pio2_3;
#[doc = "SWDIO_PIO1_3 (rw) register accessor: an alias for `Reg<SWDIO_PIO1_3_SPEC>`"]
pub type SWDIO_PIO1_3 = crate::Reg<swdio_pio1_3::SWDIO_PIO1_3_SPEC>;
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/ CT32B1_MAT2"]
pub mod swdio_pio1_3;
#[doc = "PIO1_4 (rw) register accessor: an alias for `Reg<PIO1_4_SPEC>`"]
pub type PIO1_4 = crate::Reg<pio1_4::PIO1_4_SPEC>;
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub mod pio1_4;
#[doc = "PIO1_11 (rw) register accessor: an alias for `Reg<PIO1_11_SPEC>`"]
pub type PIO1_11 = crate::Reg<pio1_11::PIO1_11_SPEC>;
#[doc = "I/O configuration for pin PIO1_11/AD7"]
pub mod pio1_11;
#[doc = "PIO3_2 (rw) register accessor: an alias for `Reg<PIO3_2_SPEC>`"]
pub type PIO3_2 = crate::Reg<pio3_2::PIO3_2_SPEC>;
#[doc = "I/O configuration for pin PIO3_2/DCD"]
pub mod pio3_2;
#[doc = "PIO1_5 (rw) register accessor: an alias for `Reg<PIO1_5_SPEC>`"]
pub type PIO1_5 = crate::Reg<pio1_5::PIO1_5_SPEC>;
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub mod pio1_5;
#[doc = "PIO1_6 (rw) register accessor: an alias for `Reg<PIO1_6_SPEC>`"]
pub type PIO1_6 = crate::Reg<pio1_6::PIO1_6_SPEC>;
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub mod pio1_6;
#[doc = "PIO1_7 (rw) register accessor: an alias for `Reg<PIO1_7_SPEC>`"]
pub type PIO1_7 = crate::Reg<pio1_7::PIO1_7_SPEC>;
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub mod pio1_7;
#[doc = "PIO3_3 (rw) register accessor: an alias for `Reg<PIO3_3_SPEC>`"]
pub type PIO3_3 = crate::Reg<pio3_3::PIO3_3_SPEC>;
#[doc = "I/O configuration for pin PIO3_3/RI"]
pub mod pio3_3;
#[doc = "SCK0_LOC (rw) register accessor: an alias for `Reg<SCK0_LOC_SPEC>`"]
pub type SCK0_LOC = crate::Reg<sck0_loc::SCK0_LOC_SPEC>;
#[doc = "SCK0 pin location register"]
pub mod sck0_loc;
#[doc = "DSR_LOC (rw) register accessor: an alias for `Reg<DSR_LOC_SPEC>`"]
pub type DSR_LOC = crate::Reg<dsr_loc::DSR_LOC_SPEC>;
#[doc = "DSR pin location select register"]
pub mod dsr_loc;
#[doc = "DCD_LOC (rw) register accessor: an alias for `Reg<DCD_LOC_SPEC>`"]
pub type DCD_LOC = crate::Reg<dcd_loc::DCD_LOC_SPEC>;
#[doc = "DCD pin location select register"]
pub mod dcd_loc;
#[doc = "RI_LOC (rw) register accessor: an alias for `Reg<RI_LOC_SPEC>`"]
pub type RI_LOC = crate::Reg<ri_loc::RI_LOC_SPEC>;
#[doc = "RI pin location register"]
pub mod ri_loc;
