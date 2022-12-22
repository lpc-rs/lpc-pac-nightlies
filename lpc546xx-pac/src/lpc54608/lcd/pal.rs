#[doc = "Register `PAL[%s]` reader"]
pub struct R(crate::R<PAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAL[%s]` writer"]
pub struct W(crate::W<PAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAL_SPEC>;
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
impl From<crate::W<PAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R04_0` reader - Red palette data."]
pub type R04_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R04_0` writer - Red palette data."]
pub type R04_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `G04_0` reader - Green palette data."]
pub type G04_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G04_0` writer - Green palette data."]
pub type G04_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `B04_0` reader - Blue palette data."]
pub type B04_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B04_0` writer - Blue palette data."]
pub type B04_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `I0` reader - Intensity / unused bit."]
pub type I0_R = crate::BitReader<bool>;
#[doc = "Field `I0` writer - Intensity / unused bit."]
pub type I0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAL_SPEC, bool, O>;
#[doc = "Field `R14_0` reader - Red palette data."]
pub type R14_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R14_0` writer - Red palette data."]
pub type R14_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `G14_0` reader - Green palette data."]
pub type G14_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G14_0` writer - Green palette data."]
pub type G14_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `B14_0` reader - Blue palette data."]
pub type B14_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B14_0` writer - Blue palette data."]
pub type B14_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `I1` reader - Intensity / unused bit."]
pub type I1_R = crate::BitReader<bool>;
#[doc = "Field `I1` writer - Intensity / unused bit."]
pub type I1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&self) -> R04_0_R {
        R04_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&self) -> G04_0_R {
        G04_0_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&self) -> B04_0_R {
        B04_0_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&self) -> R14_0_R {
        R14_0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&self) -> G14_0_R {
        G14_0_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&self) -> B14_0_R {
        B14_0_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&mut self) -> R04_0_W<0> {
        R04_0_W::new(self)
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&mut self) -> G04_0_W<5> {
        G04_0_W::new(self)
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&mut self) -> B04_0_W<10> {
        B04_0_W::new(self)
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&mut self) -> I0_W<15> {
        I0_W::new(self)
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&mut self) -> R14_0_W<16> {
        R14_0_W::new(self)
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&mut self) -> G14_0_W<21> {
        G14_0_W::new(self)
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&mut self) -> B14_0_W<26> {
        B14_0_W::new(self)
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&mut self) -> I1_W<31> {
        I1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "256x16-bit Color Palette registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pal](index.html) module"]
pub struct PAL_SPEC;
impl crate::RegisterSpec for PAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pal::R](R) reader structure"]
impl crate::Readable for PAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pal::W](W) writer structure"]
impl crate::Writable for PAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAL[%s]
to value 0"]
impl crate::Resettable for PAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
