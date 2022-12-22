#[doc = "Register `FIFORD` reader"]
pub struct R(crate::R<FIFORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received data from the FIFO."]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type RXSSEL0_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL1_N` reader - Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type RXSSEL1_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL2_N` reader - Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type RXSSEL2_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL3_N` reader - Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type RXSSEL3_N_R = crate::BitReader<bool>;
#[doc = "Field `SOT` reader - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
pub type SOT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> RXSSEL1_N_R {
        RXSSEL1_N_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> RXSSEL2_N_R {
        RXSSEL2_N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> RXSSEL3_N_R {
        RXSSEL3_N_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "FIFO read data.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford](index.html) module"]
pub struct FIFORD_SPEC;
impl crate::RegisterSpec for FIFORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiford::R](R) reader structure"]
impl crate::Readable for FIFORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORD to value 0"]
impl crate::Resettable for FIFORD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
