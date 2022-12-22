#[doc = "Register `FMSSTOP` reader"]
pub struct R(crate::R<FMSSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSSTOP` writer"]
pub struct W(crate::W<FMSSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTOP_SPEC>;
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
impl From<crate::W<FMSSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub type STOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STOP` writer - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMSSTOP_SPEC, u32, u32, 17, O>;
#[doc = "Field `SIG_START` reader - Start control bit for signature generation."]
pub type SIG_START_R = crate::BitReader<SIG_START_A>;
#[doc = "Start control bit for signature generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_START_A {
    #[doc = "0: Signature generation is stopped"]
    SIGNATURE_GENERATION = 0,
    #[doc = "1: Initiate signature generation"]
    INITIATE_SIGNATURE_G = 1,
}
impl From<SIG_START_A> for bool {
    #[inline(always)]
    fn from(variant: SIG_START_A) -> Self {
        variant as u8 != 0
    }
}
impl SIG_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIG_START_A {
        match self.bits {
            false => SIG_START_A::SIGNATURE_GENERATION,
            true => SIG_START_A::INITIATE_SIGNATURE_G,
        }
    }
    #[doc = "Checks if the value of the field is `SIGNATURE_GENERATION`"]
    #[inline(always)]
    pub fn is_signature_generation(&self) -> bool {
        *self == SIG_START_A::SIGNATURE_GENERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_SIGNATURE_G`"]
    #[inline(always)]
    pub fn is_initiate_signature_g(&self) -> bool {
        *self == SIG_START_A::INITIATE_SIGNATURE_G
    }
}
#[doc = "Field `SIG_START` writer - Start control bit for signature generation."]
pub type SIG_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSSTOP_SPEC, SIG_START_A, O>;
impl<'a, const O: u8> SIG_START_W<'a, O> {
    #[doc = "Signature generation is stopped"]
    #[inline(always)]
    pub fn signature_generation(self) -> &'a mut W {
        self.variant(SIG_START_A::SIGNATURE_GENERATION)
    }
    #[doc = "Initiate signature generation"]
    #[inline(always)]
    pub fn initiate_signature_g(self) -> &'a mut W {
        self.variant(SIG_START_A::INITIATE_SIGNATURE_G)
    }
}
impl R {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<0> {
        STOP_W::new(self)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W<17> {
        SIG_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature stop-address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](index.html) module"]
pub struct FMSSTOP_SPEC;
impl crate::RegisterSpec for FMSSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsstop::R](R) reader structure"]
impl crate::Readable for FMSSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](W) writer structure"]
impl crate::Writable for FMSSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMSSTOP to value 0"]
impl crate::Resettable for FMSSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
