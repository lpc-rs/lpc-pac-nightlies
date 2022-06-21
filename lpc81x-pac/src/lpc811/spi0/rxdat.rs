#[doc = "Register `RXDAT` reader"]
pub struct R(crate::R<RXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDAT` reader - Receiver Data. This contains the next piece of received data. The number of bits that are used depends on the LEN setting in TXCTL / TXDATCTL."]
pub type RXDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type RXSSEL0_N_R = crate::BitReader<bool>;
#[doc = "Field `SOT` reader - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bit."]
pub type SOT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Receiver Data. This contains the next piece of received data. The number of bits that are used depends on the LEN setting in TXCTL / TXDATCTL."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bit."]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "SPI Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdat](index.html) module"]
pub struct RXDAT_SPEC;
impl crate::RegisterSpec for RXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdat::R](R) reader structure"]
impl crate::Readable for RXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDAT to value 0"]
impl crate::Resettable for RXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
