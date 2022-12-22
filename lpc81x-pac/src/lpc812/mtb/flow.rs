#[doc = "Register `FLOW` reader"]
pub struct R(crate::R<FLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW` writer"]
pub struct W(crate::W<FLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_SPEC>;
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
impl From<crate::W<FLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOSTOP` reader - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.EN bit is automatically set to 0. This stops tracing."]
pub type AUTOSTOP_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTOP` writer - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.EN bit is automatically set to 0. This stops tracing."]
pub type AUTOSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_SPEC, bool, O>;
#[doc = "Field `AUTOHALT` reader - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.HALTREQ bit is automatically set to 1. If the DBGEN signal is HIGH, the MTB asserts this halt request to the Cortex-M0+ processor by asserting the EDBGRQ signal."]
pub type AUTOHALT_R = crate::BitReader<bool>;
#[doc = "Field `AUTOHALT` writer - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.HALTREQ bit is automatically set to 1. If the DBGEN signal is HIGH, the MTB asserts this halt request to the Cortex-M0+ processor by asserting the EDBGRQ signal."]
pub type AUTOHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_SPEC, bool, O>;
#[doc = "Field `WATERMARK` reader - WATERMARK value. This field contains an address in the same format as the POSITION.POINTER field. When the POSITION.POINTER matches the WATERMARK field value, actions defined by the AUTOHALT and AUTOSTOP bits are performed."]
pub type WATERMARK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WATERMARK` writer - WATERMARK value. This field contains an address in the same format as the POSITION.POINTER field. When the POSITION.POINTER matches the WATERMARK field value, actions defined by the AUTOHALT and AUTOSTOP bits are performed."]
pub type WATERMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOW_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.EN bit is automatically set to 0. This stops tracing."]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.HALTREQ bit is automatically set to 1. If the DBGEN signal is HIGH, the MTB asserts this halt request to the Cortex-M0+ processor by asserting the EDBGRQ signal."]
    #[inline(always)]
    pub fn autohalt(&self) -> AUTOHALT_R {
        AUTOHALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:31 - WATERMARK value. This field contains an address in the same format as the POSITION.POINTER field. When the POSITION.POINTER matches the WATERMARK field value, actions defined by the AUTOHALT and AUTOSTOP bits are performed."]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.EN bit is automatically set to 0. This stops tracing."]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W<0> {
        AUTOSTOP_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is 1 and WATERMARK is equal to POSITION.POINTER, then the MASTER.HALTREQ bit is automatically set to 1. If the DBGEN signal is HIGH, the MTB asserts this halt request to the Cortex-M0+ processor by asserting the EDBGRQ signal."]
    #[inline(always)]
    pub fn autohalt(&mut self) -> AUTOHALT_W<1> {
        AUTOHALT_W::new(self)
    }
    #[doc = "Bits 3:31 - WATERMARK value. This field contains an address in the same format as the POSITION.POINTER field. When the POSITION.POINTER matches the WATERMARK field value, actions defined by the AUTOHALT and AUTOSTOP bits are performed."]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W<3> {
        WATERMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLOW Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow](index.html) module"]
pub struct FLOW_SPEC;
impl crate::RegisterSpec for FLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow::R](R) reader structure"]
impl crate::Readable for FLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow::W](W) writer structure"]
impl crate::Writable for FLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOW to value 0"]
impl crate::Resettable for FLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
