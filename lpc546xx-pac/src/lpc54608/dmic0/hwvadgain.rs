#[doc = "Register `HWVADGAIN` reader"]
pub struct R(crate::R<HWVADGAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADGAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADGAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADGAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADGAIN` writer"]
pub struct W(crate::W<HWVADGAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADGAIN_SPEC>;
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
impl From<crate::W<HWVADGAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADGAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTGAIN` reader - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
pub type INPUTGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTGAIN` writer - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
pub type INPUTGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVADGAIN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&self) -> INPUTGAIN_R {
        INPUTGAIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&mut self) -> INPUTGAIN_W<0> {
        INPUTGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD input gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadgain](index.html) module"]
pub struct HWVADGAIN_SPEC;
impl crate::RegisterSpec for HWVADGAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadgain::R](R) reader structure"]
impl crate::Readable for HWVADGAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadgain::W](W) writer structure"]
impl crate::Writable for HWVADGAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWVADGAIN to value 0x05"]
impl crate::Resettable for HWVADGAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
