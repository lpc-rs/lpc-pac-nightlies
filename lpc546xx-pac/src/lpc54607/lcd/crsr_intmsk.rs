#[doc = "Register `CRSR_INTMSK` reader"]
pub struct R(crate::R<CRSR_INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_INTMSK` writer"]
pub struct W(crate::W<CRSR_INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_INTMSK_SPEC>;
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
impl From<crate::W<CRSR_INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRIM` reader - Cursor interrupt mask."]
pub type CRSRIM_R = crate::BitReader<bool>;
#[doc = "Field `CRSRIM` writer - Cursor interrupt mask."]
pub type CRSRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRSR_INTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cursor interrupt mask."]
    #[inline(always)]
    pub fn crsrim(&self) -> CRSRIM_R {
        CRSRIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor interrupt mask."]
    #[inline(always)]
    pub fn crsrim(&mut self) -> CRSRIM_W<0> {
        CRSRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intmsk](index.html) module"]
pub struct CRSR_INTMSK_SPEC;
impl crate::RegisterSpec for CRSR_INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_intmsk::R](R) reader structure"]
impl crate::Readable for CRSR_INTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_intmsk::W](W) writer structure"]
impl crate::Writable for CRSR_INTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_INTMSK to value 0"]
impl crate::Resettable for CRSR_INTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
