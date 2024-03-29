#[doc = "Register `FIFOWR` reader"]
pub struct R(crate::R<FIFOWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOWR` writer"]
pub struct W(crate::W<FIFOWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR_SPEC>;
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
impl From<crate::W<FIFOWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - Transmit data to the FIFO."]
pub type TXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO."]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO write data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr](index.html) module"]
pub struct FIFOWR_SPEC;
impl crate::RegisterSpec for FIFOWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifowr::R](R) reader structure"]
impl crate::Readable for FIFOWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifowr::W](W) writer structure"]
impl crate::Writable for FIFOWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOWR to value 0"]
impl crate::Resettable for FIFOWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
