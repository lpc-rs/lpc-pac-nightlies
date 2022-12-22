#[doc = "Register `AHBCLKDIV` reader"]
pub struct R(crate::R<AHBCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKDIV` writer"]
pub struct W(crate::W<AHBCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKDIV_SPEC>;
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
impl From<crate::W<AHBCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value. 0: Divide by 1 up to 255: Divide by 256."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Clock divider value. 0: Divide by 1 up to 255: Divide by 256."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBCLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub type REQFLAG_R = crate::BitReader<bool>;
#[doc = "Field `REQFLAG` writer - Divider status flag."]
pub type REQFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divider value. 0: Divide by 1 up to 255: Divide by 256."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider value. 0: Divide by 1 up to 255: Divide by 256."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&mut self) -> REQFLAG_W<31> {
        REQFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkdiv](index.html) module"]
pub struct AHBCLKDIV_SPEC;
impl crate::RegisterSpec for AHBCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkdiv::R](R) reader structure"]
impl crate::Readable for AHBCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkdiv::W](W) writer structure"]
impl crate::Writable for AHBCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKDIV to value 0"]
impl crate::Resettable for AHBCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
