#[doc = "Register `TXFQS` reader"]
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFQS` writer"]
pub struct W(crate::W<TXFQS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFQS_SPEC>;
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
impl From<crate::W<TXFQS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFQS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFGI` reader - Tx FIFO get index."]
pub type TFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFGI` writer - Tx FIFO get index."]
pub type TFGI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFQS_SPEC, u8, u8, 5, O>;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index."]
pub type TFQPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQPI` writer - Tx FIFO/queue put index."]
pub type TFQPI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFQS_SPEC, u8, u8, 5, O>;
#[doc = "Field `TFQF` reader - Tx FIFO/queue full."]
pub type TFQF_R = crate::BitReader<bool>;
#[doc = "Field `TFQF` writer - Tx FIFO/queue full."]
pub type TFQF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXFQS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&mut self) -> TFGI_W<8> {
        TFGI_W::new(self)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&mut self) -> TFQPI_W<16> {
        TFQPI_W::new(self)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&mut self) -> TFQF_W<21> {
        TFQF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx FIFO/Queue Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](index.html) module"]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfqs::R](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfqs::W](W) writer structure"]
impl crate::Writable for TXFQS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TXFQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
