#[doc = "Register `TESSEICR` reader"]
pub struct R(crate::R<TESSEICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESSEICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESSEICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESSEICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TESSEICR` writer"]
pub struct W(crate::W<TESSEICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESSEICR_SPEC>;
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
impl From<crate::W<TESSEICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESSEICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `StartResourceSelection` reader - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
pub type START_RESOURCE_SELECTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `StartResourceSelection` writer - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
pub type START_RESOURCE_SELECTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TESSEICR_SPEC, u8, u8, 4, O>;
#[doc = "Field `StopResourceSelection` reader - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
pub type STOP_RESOURCE_SELECTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `StopResourceSelection` writer - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
pub type STOP_RESOURCE_SELECTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TESSEICR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&self) -> START_RESOURCE_SELECTION_R {
        START_RESOURCE_SELECTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&self) -> STOP_RESOURCE_SELECTION_R {
        STOP_RESOURCE_SELECTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&mut self) -> START_RESOURCE_SELECTION_W<0> {
        START_RESOURCE_SELECTION_W::new(self)
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&mut self) -> STOP_RESOURCE_SELECTION_W<16> {
        STOP_RESOURCE_SELECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tesseicr](index.html) module"]
pub struct TESSEICR_SPEC;
impl crate::RegisterSpec for TESSEICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tesseicr::R](R) reader structure"]
impl crate::Readable for TESSEICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tesseicr::W](W) writer structure"]
impl crate::Writable for TESSEICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TESSEICR to value 0"]
impl crate::Resettable for TESSEICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
