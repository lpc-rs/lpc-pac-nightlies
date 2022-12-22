#[doc = "Register `HWVADHPFS` reader"]
pub struct R(crate::R<HWVADHPFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADHPFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADHPFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADHPFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADHPFS` writer"]
pub struct W(crate::W<HWVADHPFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADHPFS_SPEC>;
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
impl From<crate::W<HWVADHPFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADHPFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPFS` reader - High pass filter"]
pub type HPFS_R = crate::FieldReader<u8, HPFS_A>;
#[doc = "High pass filter\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPFS_A {
    #[doc = "0: First filter by-pass."]
    BYPASS = 0,
    #[doc = "1: High pass filter with -3dB cut-off at 1750Hz."]
    HIGH_PASS_1750HZ = 1,
    #[doc = "2: High pass filter with -3dB cut-off at 215Hz."]
    HIGH_PASS_215HZ = 2,
}
impl From<HPFS_A> for u8 {
    #[inline(always)]
    fn from(variant: HPFS_A) -> Self {
        variant as _
    }
}
impl HPFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPFS_A> {
        match self.bits {
            0 => Some(HPFS_A::BYPASS),
            1 => Some(HPFS_A::HIGH_PASS_1750HZ),
            2 => Some(HPFS_A::HIGH_PASS_215HZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == HPFS_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_1750HZ`"]
    #[inline(always)]
    pub fn is_high_pass_1750hz(&self) -> bool {
        *self == HPFS_A::HIGH_PASS_1750HZ
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_215HZ`"]
    #[inline(always)]
    pub fn is_high_pass_215hz(&self) -> bool {
        *self == HPFS_A::HIGH_PASS_215HZ
    }
}
#[doc = "Field `HPFS` writer - High pass filter"]
pub type HPFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVADHPFS_SPEC, u8, HPFS_A, 2, O>;
impl<'a, const O: u8> HPFS_W<'a, O> {
    #[doc = "First filter by-pass."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(HPFS_A::BYPASS)
    }
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    #[inline(always)]
    pub fn high_pass_1750hz(self) -> &'a mut W {
        self.variant(HPFS_A::HIGH_PASS_1750HZ)
    }
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    #[inline(always)]
    pub fn high_pass_215hz(self) -> &'a mut W {
        self.variant(HPFS_A::HIGH_PASS_215HZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&self) -> HPFS_R {
        HPFS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&mut self) -> HPFS_W<0> {
        HPFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadhpfs](index.html) module"]
pub struct HWVADHPFS_SPEC;
impl crate::RegisterSpec for HWVADHPFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadhpfs::R](R) reader structure"]
impl crate::Readable for HWVADHPFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadhpfs::W](W) writer structure"]
impl crate::Writable for HWVADHPFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWVADHPFS to value 0x01"]
impl crate::Resettable for HWVADHPFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
