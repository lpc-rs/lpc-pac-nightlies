#[doc = "Register `PID` reader"]
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PID` writer"]
pub struct W(crate::W<PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_SPEC>;
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
impl From<crate::W<PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Minor_Rev` reader - Minor revision of module implementation."]
pub type MINOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Major_Rev` reader - Major revision of module implementation."]
pub type MAJOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` reader - Module identifier for the selected function."]
pub type ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ID` writer - Module identifier for the selected function."]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 8:11 - Minor revision of module implementation."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<16> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral identification register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](index.html) module"]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid::R](R) reader structure"]
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pid::W](W) writer structure"]
impl crate::Writable for PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
