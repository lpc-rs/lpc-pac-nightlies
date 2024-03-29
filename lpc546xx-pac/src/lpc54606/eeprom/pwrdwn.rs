#[doc = "Register `PWRDWN` reader"]
pub struct R(crate::R<PWRDWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDWN` writer"]
pub struct W(crate::W<PWRDWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDWN_SPEC>;
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
impl From<crate::W<PWRDWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power down mode bit."]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power down mode bit."]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRDWN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power down mode bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down mode bit."]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwn](index.html) module"]
pub struct PWRDWN_SPEC;
impl crate::RegisterSpec for PWRDWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdwn::R](R) reader structure"]
impl crate::Readable for PWRDWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdwn::W](W) writer structure"]
impl crate::Writable for PWRDWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRDWN to value 0"]
impl crate::Resettable for PWRDWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
