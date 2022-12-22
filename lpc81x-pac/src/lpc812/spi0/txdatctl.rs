#[doc = "Register `TXDATCTL` reader"]
pub struct R(crate::R<TXDATCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATCTL` writer"]
pub struct W(crate::W<TXDATCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXDATCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDAT` reader - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
pub type TXDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXDAT` writer - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
pub type TXDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATCTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `TXSSEL0_N` reader - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
pub type TXSSEL0_N_R = crate::BitReader<TXSSEL0_N_A>;
#[doc = "Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL0_N_A {
    #[doc = "0: SSEL0 asserted."]
    TXSSEL0_N_0 = 0,
    #[doc = "1: SSEL0 not asserted."]
    TXSSEL0_N_1 = 1,
}
impl From<TXSSEL0_N_A> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL0_N_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSSEL0_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSSEL0_N_A {
        match self.bits {
            false => TXSSEL0_N_A::TXSSEL0_N_0,
            true => TXSSEL0_N_A::TXSSEL0_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXSSEL0_N_0`"]
    #[inline(always)]
    pub fn is_txssel0_n_0(&self) -> bool {
        *self == TXSSEL0_N_A::TXSSEL0_N_0
    }
    #[doc = "Checks if the value of the field is `TXSSEL0_N_1`"]
    #[inline(always)]
    pub fn is_txssel0_n_1(&self) -> bool {
        *self == TXSSEL0_N_A::TXSSEL0_N_1
    }
}
#[doc = "Field `TXSSEL0_N` writer - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
pub type TXSSEL0_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATCTL_SPEC, TXSSEL0_N_A, O>;
impl<'a, const O: u8> TXSSEL0_N_W<'a, O> {
    #[doc = "SSEL0 asserted."]
    #[inline(always)]
    pub fn txssel0_n_0(self) -> &'a mut W {
        self.variant(TXSSEL0_N_A::TXSSEL0_N_0)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline(always)]
    pub fn txssel0_n_1(self) -> &'a mut W {
        self.variant(TXSSEL0_N_A::TXSSEL0_N_1)
    }
}
#[doc = "Field `EOT` reader - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
pub type EOT_R = crate::BitReader<EOT_A>;
#[doc = "End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_A {
    #[doc = "0: This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    SSEL_DEASSERTED = 0,
    #[doc = "1: This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    SSEL_NOT_DEASSERTED = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
impl EOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::SSEL_DEASSERTED,
            true => EOT_A::SSEL_NOT_DEASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `SSEL_DEASSERTED`"]
    #[inline(always)]
    pub fn is_ssel_deasserted(&self) -> bool {
        *self == EOT_A::SSEL_DEASSERTED
    }
    #[doc = "Checks if the value of the field is `SSEL_NOT_DEASSERTED`"]
    #[inline(always)]
    pub fn is_ssel_not_deasserted(&self) -> bool {
        *self == EOT_A::SSEL_NOT_DEASSERTED
    }
}
#[doc = "Field `EOT` writer - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATCTL_SPEC, EOT_A, O>;
impl<'a, const O: u8> EOT_W<'a, O> {
    #[doc = "This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn ssel_deasserted(self) -> &'a mut W {
        self.variant(EOT_A::SSEL_DEASSERTED)
    }
    #[doc = "This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn ssel_not_deasserted(self) -> &'a mut W {
        self.variant(EOT_A::SSEL_NOT_DEASSERTED)
    }
}
#[doc = "Field `EOF` reader - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
pub type EOF_R = crate::BitReader<EOF_A>;
#[doc = "End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_A {
    #[doc = "0: This piece of data transmitted is not treated as the end of a frame."]
    DATA_NOT_EOF = 0,
    #[doc = "1: This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    DATA_EOF = 1,
}
impl From<EOF_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOF_A {
        match self.bits {
            false => EOF_A::DATA_NOT_EOF,
            true => EOF_A::DATA_EOF,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_NOT_EOF`"]
    #[inline(always)]
    pub fn is_data_not_eof(&self) -> bool {
        *self == EOF_A::DATA_NOT_EOF
    }
    #[doc = "Checks if the value of the field is `DATA_EOF`"]
    #[inline(always)]
    pub fn is_data_eof(&self) -> bool {
        *self == EOF_A::DATA_EOF
    }
}
#[doc = "Field `EOF` writer - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
pub type EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATCTL_SPEC, EOF_A, O>;
impl<'a, const O: u8> EOF_W<'a, O> {
    #[doc = "This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn data_not_eof(self) -> &'a mut W {
        self.variant(EOF_A::DATA_NOT_EOF)
    }
    #[doc = "This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn data_eof(self) -> &'a mut W {
        self.variant(EOF_A::DATA_EOF)
    }
}
#[doc = "Field `RXIGNORE` reader - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
pub type RXIGNORE_R = crate::BitReader<RXIGNORE_A>;
#[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNORE_A {
    #[doc = "0: Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ_RECEIVED_DATA = 0,
    #[doc = "1: Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE_RECEIVED_DATA = 1,
}
impl From<RXIGNORE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIGNORE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIGNORE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIGNORE_A {
        match self.bits {
            false => RXIGNORE_A::READ_RECEIVED_DATA,
            true => RXIGNORE_A::IGNORE_RECEIVED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `READ_RECEIVED_DATA`"]
    #[inline(always)]
    pub fn is_read_received_data(&self) -> bool {
        *self == RXIGNORE_A::READ_RECEIVED_DATA
    }
    #[doc = "Checks if the value of the field is `IGNORE_RECEIVED_DATA`"]
    #[inline(always)]
    pub fn is_ignore_received_data(&self) -> bool {
        *self == RXIGNORE_A::IGNORE_RECEIVED_DATA
    }
}
#[doc = "Field `RXIGNORE` writer - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
pub type RXIGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATCTL_SPEC, RXIGNORE_A, O>;
impl<'a, const O: u8> RXIGNORE_W<'a, O> {
    #[doc = "Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read_received_data(self) -> &'a mut W {
        self.variant(RXIGNORE_A::READ_RECEIVED_DATA)
    }
    #[doc = "Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore_received_data(self) -> &'a mut W {
        self.variant(RXIGNORE_A::IGNORE_RECEIVED_DATA)
    }
}
#[doc = "Field `LEN` reader - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
pub type LEN_R = crate::FieldReader<u8, LEN_A>;
#[doc = "Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: no description available"]
    LEN_0 = 0,
    #[doc = "1: Data transfer is 1 bit in length."]
    LEN_1 = 1,
    #[doc = "2: Data transfer is 2 bit in length."]
    LEN_2 = 2,
    #[doc = "3: Data transfer is 3 bit in length."]
    LEN_3 = 3,
    #[doc = "4: Data transfer is 4 bit in length."]
    LEN_4 = 4,
    #[doc = "5: Data transfer is 5 bit in length."]
    LEN_5 = 5,
    #[doc = "6: Data transfer is 6 bit in length."]
    LEN_6 = 6,
    #[doc = "7: Data transfer is 7 bit in length."]
    LEN_7 = 7,
    #[doc = "8: Data transfer is 8 bit in length."]
    LEN_8 = 8,
    #[doc = "9: Data transfer is 9 bit in length."]
    LEN_9 = 9,
    #[doc = "10: Data transfer is 10 bit in length."]
    LEN_10 = 10,
    #[doc = "11: Data transfer is 11 bit in length."]
    LEN_11 = 11,
    #[doc = "12: Data transfer is 12 bit in length."]
    LEN_12 = 12,
    #[doc = "13: Data transfer is 13 bit in length."]
    LEN_13 = 13,
    #[doc = "14: Data transfer is 14 bit in length."]
    LEN_14 = 14,
    #[doc = "15: Data transfer is 15 bit in length."]
    LEN_15 = 15,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
impl LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            0 => LEN_A::LEN_0,
            1 => LEN_A::LEN_1,
            2 => LEN_A::LEN_2,
            3 => LEN_A::LEN_3,
            4 => LEN_A::LEN_4,
            5 => LEN_A::LEN_5,
            6 => LEN_A::LEN_6,
            7 => LEN_A::LEN_7,
            8 => LEN_A::LEN_8,
            9 => LEN_A::LEN_9,
            10 => LEN_A::LEN_10,
            11 => LEN_A::LEN_11,
            12 => LEN_A::LEN_12,
            13 => LEN_A::LEN_13,
            14 => LEN_A::LEN_14,
            15 => LEN_A::LEN_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEN_0`"]
    #[inline(always)]
    pub fn is_len_0(&self) -> bool {
        *self == LEN_A::LEN_0
    }
    #[doc = "Checks if the value of the field is `LEN_1`"]
    #[inline(always)]
    pub fn is_len_1(&self) -> bool {
        *self == LEN_A::LEN_1
    }
    #[doc = "Checks if the value of the field is `LEN_2`"]
    #[inline(always)]
    pub fn is_len_2(&self) -> bool {
        *self == LEN_A::LEN_2
    }
    #[doc = "Checks if the value of the field is `LEN_3`"]
    #[inline(always)]
    pub fn is_len_3(&self) -> bool {
        *self == LEN_A::LEN_3
    }
    #[doc = "Checks if the value of the field is `LEN_4`"]
    #[inline(always)]
    pub fn is_len_4(&self) -> bool {
        *self == LEN_A::LEN_4
    }
    #[doc = "Checks if the value of the field is `LEN_5`"]
    #[inline(always)]
    pub fn is_len_5(&self) -> bool {
        *self == LEN_A::LEN_5
    }
    #[doc = "Checks if the value of the field is `LEN_6`"]
    #[inline(always)]
    pub fn is_len_6(&self) -> bool {
        *self == LEN_A::LEN_6
    }
    #[doc = "Checks if the value of the field is `LEN_7`"]
    #[inline(always)]
    pub fn is_len_7(&self) -> bool {
        *self == LEN_A::LEN_7
    }
    #[doc = "Checks if the value of the field is `LEN_8`"]
    #[inline(always)]
    pub fn is_len_8(&self) -> bool {
        *self == LEN_A::LEN_8
    }
    #[doc = "Checks if the value of the field is `LEN_9`"]
    #[inline(always)]
    pub fn is_len_9(&self) -> bool {
        *self == LEN_A::LEN_9
    }
    #[doc = "Checks if the value of the field is `LEN_10`"]
    #[inline(always)]
    pub fn is_len_10(&self) -> bool {
        *self == LEN_A::LEN_10
    }
    #[doc = "Checks if the value of the field is `LEN_11`"]
    #[inline(always)]
    pub fn is_len_11(&self) -> bool {
        *self == LEN_A::LEN_11
    }
    #[doc = "Checks if the value of the field is `LEN_12`"]
    #[inline(always)]
    pub fn is_len_12(&self) -> bool {
        *self == LEN_A::LEN_12
    }
    #[doc = "Checks if the value of the field is `LEN_13`"]
    #[inline(always)]
    pub fn is_len_13(&self) -> bool {
        *self == LEN_A::LEN_13
    }
    #[doc = "Checks if the value of the field is `LEN_14`"]
    #[inline(always)]
    pub fn is_len_14(&self) -> bool {
        *self == LEN_A::LEN_14
    }
    #[doc = "Checks if the value of the field is `LEN_15`"]
    #[inline(always)]
    pub fn is_len_15(&self) -> bool {
        *self == LEN_A::LEN_15
    }
}
#[doc = "Field `LEN` writer - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
pub type LEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TXDATCTL_SPEC, u8, LEN_A, 4, O>;
impl<'a, const O: u8> LEN_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn len_0(self) -> &'a mut W {
        self.variant(LEN_A::LEN_0)
    }
    #[doc = "Data transfer is 1 bit in length."]
    #[inline(always)]
    pub fn len_1(self) -> &'a mut W {
        self.variant(LEN_A::LEN_1)
    }
    #[doc = "Data transfer is 2 bit in length."]
    #[inline(always)]
    pub fn len_2(self) -> &'a mut W {
        self.variant(LEN_A::LEN_2)
    }
    #[doc = "Data transfer is 3 bit in length."]
    #[inline(always)]
    pub fn len_3(self) -> &'a mut W {
        self.variant(LEN_A::LEN_3)
    }
    #[doc = "Data transfer is 4 bit in length."]
    #[inline(always)]
    pub fn len_4(self) -> &'a mut W {
        self.variant(LEN_A::LEN_4)
    }
    #[doc = "Data transfer is 5 bit in length."]
    #[inline(always)]
    pub fn len_5(self) -> &'a mut W {
        self.variant(LEN_A::LEN_5)
    }
    #[doc = "Data transfer is 6 bit in length."]
    #[inline(always)]
    pub fn len_6(self) -> &'a mut W {
        self.variant(LEN_A::LEN_6)
    }
    #[doc = "Data transfer is 7 bit in length."]
    #[inline(always)]
    pub fn len_7(self) -> &'a mut W {
        self.variant(LEN_A::LEN_7)
    }
    #[doc = "Data transfer is 8 bit in length."]
    #[inline(always)]
    pub fn len_8(self) -> &'a mut W {
        self.variant(LEN_A::LEN_8)
    }
    #[doc = "Data transfer is 9 bit in length."]
    #[inline(always)]
    pub fn len_9(self) -> &'a mut W {
        self.variant(LEN_A::LEN_9)
    }
    #[doc = "Data transfer is 10 bit in length."]
    #[inline(always)]
    pub fn len_10(self) -> &'a mut W {
        self.variant(LEN_A::LEN_10)
    }
    #[doc = "Data transfer is 11 bit in length."]
    #[inline(always)]
    pub fn len_11(self) -> &'a mut W {
        self.variant(LEN_A::LEN_11)
    }
    #[doc = "Data transfer is 12 bit in length."]
    #[inline(always)]
    pub fn len_12(self) -> &'a mut W {
        self.variant(LEN_A::LEN_12)
    }
    #[doc = "Data transfer is 13 bit in length."]
    #[inline(always)]
    pub fn len_13(self) -> &'a mut W {
        self.variant(LEN_A::LEN_13)
    }
    #[doc = "Data transfer is 14 bit in length."]
    #[inline(always)]
    pub fn len_14(self) -> &'a mut W {
        self.variant(LEN_A::LEN_14)
    }
    #[doc = "Data transfer is 15 bit in length."]
    #[inline(always)]
    pub fn len_15(self) -> &'a mut W {
        self.variant(LEN_A::LEN_15)
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline(always)]
    pub fn txdat(&self) -> TXDAT_R {
        TXDAT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
    #[inline(always)]
    pub fn txssel0_n(&self) -> TXSSEL0_N_R {
        TXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub fn rxignore(&self) -> RXIGNORE_R {
        RXIGNORE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline(always)]
    pub fn txdat(&mut self) -> TXDAT_W<0> {
        TXDAT_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W<16> {
        TXSSEL0_N_W::new(self)
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W<20> {
        EOT_W::new(self)
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W<21> {
        EOF_W::new(self)
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W<22> {
        RXIGNORE_W::new(self)
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<24> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit Data with Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatctl](index.html) module"]
pub struct TXDATCTL_SPEC;
impl crate::RegisterSpec for TXDATCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdatctl::R](R) reader structure"]
impl crate::Readable for TXDATCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdatctl::W](W) writer structure"]
impl crate::Writable for TXDATCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATCTL to value 0"]
impl crate::Resettable for TXDATCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
