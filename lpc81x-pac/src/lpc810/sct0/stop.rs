#[doc = "Register `STOP` reader"]
pub struct R(crate::R<STOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STOP` writer"]
pub struct W(crate::W<STOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STOP_SPEC>;
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
impl From<crate::W<STOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPMSK_L` reader - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type STOPMSK_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOPMSK_L` writer - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type STOPMSK_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STOP_SPEC, u8, u8, 6, O>;
#[doc = "Field `STOPMSK_H` reader - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type STOPMSK_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOPMSK_H` writer - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type STOPMSK_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STOP_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&self) -> STOPMSK_L_R {
        STOPMSK_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&self) -> STOPMSK_H_R {
        STOPMSK_H_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&mut self) -> STOPMSK_L_W<0> {
        STOPMSK_L_W::new(self)
    }
    #[doc = "Bits 16:21 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&mut self) -> STOPMSK_H_W<16> {
        STOPMSK_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT stop event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stop](index.html) module"]
pub struct STOP_SPEC;
impl crate::RegisterSpec for STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stop::R](R) reader structure"]
impl crate::Readable for STOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stop::W](W) writer structure"]
impl crate::Writable for STOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for STOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
