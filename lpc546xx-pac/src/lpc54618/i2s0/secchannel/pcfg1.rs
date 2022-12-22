#[doc = "Register `PCFG1` reader"]
pub struct R(crate::R<PCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG1` writer"]
pub struct W(crate::W<PCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG1_SPEC>;
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
impl From<crate::W<PCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIRENABLE` reader - Enable for this channel pair.."]
pub type PAIRENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAIRENABLE` writer - Enable for this channel pair.."]
pub type PAIRENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG1_SPEC, bool, O>;
#[doc = "Field `ONECHANNEL` reader - Single channel mode."]
pub type ONECHANNEL_R = crate::BitReader<bool>;
#[doc = "Field `ONECHANNEL` writer - Single channel mode."]
pub type ONECHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&self) -> PAIRENABLE_R {
        PAIRENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&mut self) -> PAIRENABLE_W<0> {
        PAIRENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> ONECHANNEL_W<10> {
        ONECHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg1](index.html) module"]
pub struct PCFG1_SPEC;
impl crate::RegisterSpec for PCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg1::R](R) reader structure"]
impl crate::Readable for PCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg1::W](W) writer structure"]
impl crate::Writable for PCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG1 to value 0"]
impl crate::Resettable for PCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
