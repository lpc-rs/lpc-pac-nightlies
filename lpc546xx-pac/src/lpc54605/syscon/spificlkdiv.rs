#[doc = "Register `SPIFICLKDIV` reader"]
pub struct R(crate::R<SPIFICLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIFICLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIFICLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIFICLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIFICLKDIV` writer"]
pub struct W(crate::W<SPIFICLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIFICLKDIV_SPEC>;
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
impl From<crate::W<SPIFICLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIFICLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Clock divider value."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIFICLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESET` reader - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIFICLKDIV_SPEC, bool, O>;
#[doc = "Field `HALT` reader - Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIFICLKDIV_SPEC, bool, O>;
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub type REQFLAG_R = crate::BitReader<bool>;
#[doc = "Field `REQFLAG` writer - Divider status flag."]
pub type REQFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIFICLKDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<29> {
        RESET_W::new(self)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W<30> {
        HALT_W::new(self)
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
#[doc = "SPIFI clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spificlkdiv](index.html) module"]
pub struct SPIFICLKDIV_SPEC;
impl crate::RegisterSpec for SPIFICLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spificlkdiv::R](R) reader structure"]
impl crate::Readable for SPIFICLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spificlkdiv::W](W) writer structure"]
impl crate::Writable for SPIFICLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIFICLKDIV to value 0x4000_0000"]
impl crate::Resettable for SPIFICLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
