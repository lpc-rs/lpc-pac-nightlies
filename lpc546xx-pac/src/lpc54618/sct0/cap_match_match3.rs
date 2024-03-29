#[doc = "Register `MATCH3` reader"]
pub struct R(crate::R<CAP_MATCH_MATCH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_MATCH_MATCH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_MATCH_MATCH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_MATCH_MATCH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH3` writer"]
pub struct W(crate::W<CAP_MATCH_MATCH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_MATCH_MATCH3_SPEC>;
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
impl From<crate::W<CAP_MATCH_MATCH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_MATCH_MATCH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCHn_L` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MATCHN_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCHn_L` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MATCHN_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAP_MATCH_MATCH3_SPEC, u16, u16, 16, O>;
#[doc = "Field `MATCHn_H` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MATCHN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCHn_H` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MATCHN_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAP_MATCH_MATCH3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&self) -> MATCHN_L_R {
        MATCHN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&self) -> MATCHN_H_R {
        MATCHN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&mut self) -> MATCHN_L_W<0> {
        MATCHN_L_W::new(self)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&mut self) -> MATCHN_H_W<16> {
        MATCHN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_match_match3](index.html) module"]
pub struct CAP_MATCH_MATCH3_SPEC;
impl crate::RegisterSpec for CAP_MATCH_MATCH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_match_match3::R](R) reader structure"]
impl crate::Readable for CAP_MATCH_MATCH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_match_match3::W](W) writer structure"]
impl crate::Writable for CAP_MATCH_MATCH3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATCH3 to value 0"]
impl crate::Resettable for CAP_MATCH_MATCH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
