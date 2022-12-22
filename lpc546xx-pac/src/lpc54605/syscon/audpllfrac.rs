#[doc = "Register `AUDPLLFRAC` reader"]
pub struct R(crate::R<AUDPLLFRAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLFRAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDPLLFRAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDPLLFRAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLFRAC` writer"]
pub struct W(crate::W<AUDPLLFRAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLFRAC_SPEC>;
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
impl From<crate::W<AUDPLLFRAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDPLLFRAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL` reader - PLL fractional divider control word"]
pub type CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CTRL` writer - PLL fractional divider control word"]
pub type CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDPLLFRAC_SPEC, u32, u32, 22, O>;
#[doc = "Field `REQ` reader - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDPLLFRAC_SPEC, bool, O>;
#[doc = "Field `SEL_EXT` reader - Select fractional divider."]
pub type SEL_EXT_R = crate::BitReader<bool>;
#[doc = "Field `SEL_EXT` writer - Select fractional divider."]
pub type SEL_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDPLLFRAC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W<0> {
        CTRL_W::new(self)
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W<22> {
        REQ_W::new(self)
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> SEL_EXT_W<23> {
        SEL_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL fractional divider control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllfrac](index.html) module"]
pub struct AUDPLLFRAC_SPEC;
impl crate::RegisterSpec for AUDPLLFRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllfrac::R](R) reader structure"]
impl crate::Readable for AUDPLLFRAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllfrac::W](W) writer structure"]
impl crate::Writable for AUDPLLFRAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLFRAC to value 0"]
impl crate::Resettable for AUDPLLFRAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
