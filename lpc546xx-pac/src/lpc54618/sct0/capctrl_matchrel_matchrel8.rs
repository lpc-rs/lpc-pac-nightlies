#[doc = "Register `MATCHREL8` reader"]
pub struct R(crate::R<CAPCTRL_MATCHREL_MATCHREL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_MATCHREL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_MATCHREL_MATCHREL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_MATCHREL_MATCHREL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCHREL8` writer"]
pub struct W(crate::W<CAPCTRL_MATCHREL_MATCHREL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_MATCHREL8_SPEC>;
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
impl From<crate::W<CAPCTRL_MATCHREL_MATCHREL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_MATCHREL_MATCHREL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOADn_L` reader - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type RELOADN_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELOADn_L` writer - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type RELOADN_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAPCTRL_MATCHREL_MATCHREL8_SPEC, u16, u16, 16, O>;
#[doc = "Field `RELOADn_H` reader - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type RELOADN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELOADn_H` writer - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type RELOADN_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAPCTRL_MATCHREL_MATCHREL8_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&self) -> RELOADN_L_R {
        RELOADN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&self) -> RELOADN_H_R {
        RELOADN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&mut self) -> RELOADN_L_W<0> {
        RELOADN_L_W::new(self)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&mut self) -> RELOADN_H_W<16> {
        RELOADN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl_matchrel_matchrel8](index.html) module"]
pub struct CAPCTRL_MATCHREL_MATCHREL8_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_MATCHREL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capctrl_matchrel_matchrel8::R](R) reader structure"]
impl crate::Readable for CAPCTRL_MATCHREL_MATCHREL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capctrl_matchrel_matchrel8::W](W) writer structure"]
impl crate::Writable for CAPCTRL_MATCHREL_MATCHREL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATCHREL8 to value 0"]
impl crate::Resettable for CAPCTRL_MATCHREL_MATCHREL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
