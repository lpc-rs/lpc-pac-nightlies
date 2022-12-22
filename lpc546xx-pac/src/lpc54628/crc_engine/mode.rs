#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_POLY` reader - CRC polynomial"]
pub type CRC_POLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_POLY` writer - CRC polynomial"]
pub type CRC_POLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIT_RVS_DATA` reader - Data bit order"]
pub type BIT_RVS_DATA_R = crate::BitReader<bool>;
#[doc = "Field `BIT_RVS_DATA` writer - Data bit order"]
pub type BIT_RVS_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CMPL_DATA` reader - Data complement"]
pub type CMPL_DATA_R = crate::BitReader<bool>;
#[doc = "Field `CMPL_DATA` writer - Data complement"]
pub type CMPL_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `BIT_RVS_SUM` reader - Sum bit order"]
pub type BIT_RVS_SUM_R = crate::BitReader<bool>;
#[doc = "Field `BIT_RVS_SUM` writer - Sum bit order"]
pub type BIT_RVS_SUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CMPL_SUM` reader - Sum complement"]
pub type CMPL_SUM_R = crate::BitReader<bool>;
#[doc = "Field `CMPL_SUM` writer - Sum complement"]
pub type CMPL_SUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CRC_POLY_R {
        CRC_POLY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Data bit order"]
    #[inline(always)]
    pub fn bit_rvs_data(&self) -> BIT_RVS_DATA_R {
        BIT_RVS_DATA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data complement"]
    #[inline(always)]
    pub fn cmpl_data(&self) -> CMPL_DATA_R {
        CMPL_DATA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sum bit order"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUM_R {
        BIT_RVS_SUM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sum complement"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CMPL_SUM_R {
        CMPL_SUM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> CRC_POLY_W<0> {
        CRC_POLY_W::new(self)
    }
    #[doc = "Bit 2 - Data bit order"]
    #[inline(always)]
    pub fn bit_rvs_data(&mut self) -> BIT_RVS_DATA_W<2> {
        BIT_RVS_DATA_W::new(self)
    }
    #[doc = "Bit 3 - Data complement"]
    #[inline(always)]
    pub fn cmpl_data(&mut self) -> CMPL_DATA_W<3> {
        CMPL_DATA_W::new(self)
    }
    #[doc = "Bit 4 - Sum bit order"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> BIT_RVS_SUM_W<4> {
        BIT_RVS_SUM_W::new(self)
    }
    #[doc = "Bit 5 - Sum complement"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> CMPL_SUM_W<5> {
        CMPL_SUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
