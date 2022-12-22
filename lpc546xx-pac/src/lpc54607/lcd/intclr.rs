#[doc = "Register `INTCLR` reader"]
pub struct R(crate::R<INTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCLR` writer"]
pub struct W(crate::W<INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLR_SPEC>;
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
impl From<crate::W<INTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUFIC` writer - FIFO underflow interrupt clear."]
pub type FUFIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCLR_SPEC, bool, O>;
#[doc = "Field `LNBUIC` writer - LCD next address base update interrupt clear."]
pub type LNBUIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCLR_SPEC, bool, O>;
#[doc = "Field `VCOMPIC` writer - Vertical compare interrupt clear."]
pub type VCOMPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCLR_SPEC, bool, O>;
#[doc = "Field `BERIC` reader - AHB master error interrupt clear."]
pub type BERIC_R = crate::BitReader<bool>;
#[doc = "Field `BERIC` writer - AHB master error interrupt clear."]
pub type BERIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - AHB master error interrupt clear."]
    #[inline(always)]
    pub fn beric(&self) -> BERIC_R {
        BERIC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO underflow interrupt clear."]
    #[inline(always)]
    pub fn fufic(&mut self) -> FUFIC_W<1> {
        FUFIC_W::new(self)
    }
    #[doc = "Bit 2 - LCD next address base update interrupt clear."]
    #[inline(always)]
    pub fn lnbuic(&mut self) -> LNBUIC_W<2> {
        LNBUIC_W::new(self)
    }
    #[doc = "Bit 3 - Vertical compare interrupt clear."]
    #[inline(always)]
    pub fn vcompic(&mut self) -> VCOMPIC_W<3> {
        VCOMPIC_W::new(self)
    }
    #[doc = "Bit 4 - AHB master error interrupt clear."]
    #[inline(always)]
    pub fn beric(&mut self) -> BERIC_W<4> {
        BERIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](index.html) module"]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intclr::R](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intclr::W](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
