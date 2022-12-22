#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_1_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_2_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_3_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_4_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_5_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_6_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_7_pinassign_pinassign_data_: [u8; 0x04],
    _reserved_8_pinassign_pinassign_data_: [u8; 0x04],
    _reserved9: [u8; 0x019c],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
    pub pinenable0: PINENABLE0,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data0(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0)
        }
    }
    #[doc = "0x00 - Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign0(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN0)
        }
    }
    #[doc = "0x04 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data1(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1)
        }
    }
    #[doc = "0x04 - Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign1(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN1)
        }
    }
    #[doc = "0x08 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data2(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2)
        }
    }
    #[doc = "0x08 - Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign2(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN2)
        }
    }
    #[doc = "0x0c - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data3(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3)
        }
    }
    #[doc = "0x0c - Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign3(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN3)
        }
    }
    #[doc = "0x10 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data4(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4)
        }
    }
    #[doc = "0x10 - Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO,SPI0_SSEL, SPI1_SCK."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign4(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN4)
        }
    }
    #[doc = "0x14 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data5(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5)
        }
    }
    #[doc = "0x14 - Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO,SPI1_SSEL, CTIN_0"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign5(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN5)
        }
    }
    #[doc = "0x18 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data6(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6)
        }
    }
    #[doc = "0x18 - Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3,CTOUT_0."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign6(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN6)
        }
    }
    #[doc = "0x1c - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data7(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7)
        }
    }
    #[doc = "0x1c - Pin assign register 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3,I2C_SDA."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign7(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN7)
        }
    }
    #[doc = "0x20 - Pin assign register"]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign_data8(
        &self,
    ) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8)
        }
    }
    #[doc = "0x20 - Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT,GPIO_INT_BMAT."]
    #[inline(always)]
    pub fn pinassign_pinassign_data_pinassign8(&self) -> &PINASSIGN_PINASSIGN_DATA_PINASSIGN8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const PINASSIGN_PINASSIGN_DATA_PINASSIGN8)
        }
    }
}
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN0 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN0 =
    crate::Reg<pinassign_pinassign_data_pinassign0::PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>;
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub mod pinassign_pinassign_data_pinassign0;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0 = crate::Reg<
    pinassign_pinassign_data_pinassign_data0::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA0_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data0;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN1 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN1 =
    crate::Reg<pinassign_pinassign_data_pinassign1::PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub mod pinassign_pinassign_data_pinassign1;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1 = crate::Reg<
    pinassign_pinassign_data_pinassign_data1::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA1_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data1;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN2 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN2_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN2 =
    crate::Reg<pinassign_pinassign_data_pinassign2::PINASSIGN_PINASSIGN_DATA_PINASSIGN2_SPEC>;
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub mod pinassign_pinassign_data_pinassign2;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2 = crate::Reg<
    pinassign_pinassign_data_pinassign_data2::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA2_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data2;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN3 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN3_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN3 =
    crate::Reg<pinassign_pinassign_data_pinassign3::PINASSIGN_PINASSIGN_DATA_PINASSIGN3_SPEC>;
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub mod pinassign_pinassign_data_pinassign3;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3 = crate::Reg<
    pinassign_pinassign_data_pinassign_data3::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA3_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data3;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN4 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN4_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN4 =
    crate::Reg<pinassign_pinassign_data_pinassign4::PINASSIGN_PINASSIGN_DATA_PINASSIGN4_SPEC>;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO,SPI0_SSEL, SPI1_SCK."]
pub mod pinassign_pinassign_data_pinassign4;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4 = crate::Reg<
    pinassign_pinassign_data_pinassign_data4::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA4_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data4;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN5 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN5 =
    crate::Reg<pinassign_pinassign_data_pinassign5::PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>;
#[doc = "Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO,SPI1_SSEL, CTIN_0"]
pub mod pinassign_pinassign_data_pinassign5;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5 = crate::Reg<
    pinassign_pinassign_data_pinassign_data5::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA5_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data5;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN6 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN6 =
    crate::Reg<pinassign_pinassign_data_pinassign6::PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>;
#[doc = "Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3,CTOUT_0."]
pub mod pinassign_pinassign_data_pinassign6;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6 = crate::Reg<
    pinassign_pinassign_data_pinassign_data6::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA6_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data6;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN7 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN7 =
    crate::Reg<pinassign_pinassign_data_pinassign7::PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>;
#[doc = "Pin assign register 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3,I2C_SDA."]
pub mod pinassign_pinassign_data_pinassign7;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7 = crate::Reg<
    pinassign_pinassign_data_pinassign_data7::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA7_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data7;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN8 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN8 =
    crate::Reg<pinassign_pinassign_data_pinassign8::PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>;
#[doc = "Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT,GPIO_INT_BMAT."]
pub mod pinassign_pinassign_data_pinassign8;
#[doc = "PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8 (rw) register accessor: an alias for `Reg<PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8_SPEC>`"]
pub type PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8 = crate::Reg<
    pinassign_pinassign_data_pinassign_data8::PINASSIGN_PINASSIGN_DATA_PINASSIGN_DATA8_SPEC,
>;
#[doc = "Pin assign register"]
pub mod pinassign_pinassign_data_pinassign_data8;
#[doc = "PINENABLE0 (rw) register accessor: an alias for `Reg<PINENABLE0_SPEC>`"]
pub type PINENABLE0 = crate::Reg<pinenable0::PINENABLE0_SPEC>;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
pub mod pinenable0;
