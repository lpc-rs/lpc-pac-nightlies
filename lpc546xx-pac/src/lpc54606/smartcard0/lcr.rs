#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLS` reader - Word Length Select."]
pub type WLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLS` writer - Word Length Select."]
pub type WLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SBS` reader - Stop Bit Select."]
pub type SBS_R = crate::BitReader<bool>;
#[doc = "Field `SBS` writer - Stop Bit Select."]
pub type SBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `PE` reader - Parity Enable."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Parity Enable."]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `PS` reader - Parity Select."]
pub type PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS` writer - Parity Select."]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit."]
pub type DLAB_R = crate::BitReader<bool>;
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit."]
pub type DLAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W<0> {
        WLS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&mut self) -> SBS_W<2> {
        SBS_W::new(self)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<3> {
        PE_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<4> {
        PS_W::new(self)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W<7> {
        DLAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
