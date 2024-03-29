#[doc = "Register `OUT_CLR` reader"]
pub struct R(crate::R<OUT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CLR` writer"]
pub struct W(crate::W<OUT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CLR_SPEC>;
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
impl From<crate::W<OUT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type CLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLR` writer - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CLR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output 0 clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_clr](index.html) module"]
pub struct OUT_CLR_SPEC;
impl crate::RegisterSpec for OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_clr::R](R) reader structure"]
impl crate::Readable for OUT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_clr::W](W) writer structure"]
impl crate::Writable for OUT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OUT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
