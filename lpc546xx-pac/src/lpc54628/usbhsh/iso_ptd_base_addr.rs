#[doc = "Register `ISO_PTD_BASE_ADDR` reader"]
pub struct R(crate::R<ISO_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISO_PTD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISO_PTD_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISO_PTD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISO_PTD_BASE_ADDR` writer"]
pub struct W(crate::W<ISO_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISO_PTD_BASE_ADDR_SPEC>;
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
impl From<crate::W<ISO_PTD_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISO_PTD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub type ISO_FIRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISO_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub type ISO_FIRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISO_PTD_BASE_ADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `ISO_BASE` reader - Base address to be used by the hardware to find the start of the ISO list."]
pub type ISO_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISO_BASE` writer - Base address to be used by the hardware to find the start of the ISO list."]
pub type ISO_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISO_PTD_BASE_ADDR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&mut self) -> ISO_FIRST_W<5> {
        ISO_FIRST_W::new(self)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&mut self) -> ISO_BASE_W<10> {
        ISO_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address where ISO PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_ptd_base_addr](index.html) module"]
pub struct ISO_PTD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for ISO_PTD_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iso_ptd_base_addr::R](R) reader structure"]
impl crate::Readable for ISO_PTD_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iso_ptd_base_addr::W](W) writer structure"]
impl crate::Writable for ISO_PTD_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISO_PTD_BASE_ADDR to value 0"]
impl crate::Resettable for ISO_PTD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
