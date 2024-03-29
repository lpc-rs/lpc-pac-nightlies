#[doc = "Register `INT_PTD_BASE_ADDR` reader"]
pub struct R(crate::R<INT_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_PTD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_PTD_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_PTD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_PTD_BASE_ADDR` writer"]
pub struct W(crate::W<INT_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_PTD_BASE_ADDR_SPEC>;
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
impl From<crate::W<INT_PTD_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_PTD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub type INT_FIRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub type INT_FIRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_PTD_BASE_ADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `INT_BASE` reader - Base address to be used by the hardware to find the start of the INT list."]
pub type INT_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_BASE` writer - Base address to be used by the hardware to find the start of the INT list."]
pub type INT_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_PTD_BASE_ADDR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub fn int_first(&self) -> INT_FIRST_R {
        INT_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub fn int_base(&self) -> INT_BASE_R {
        INT_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub fn int_first(&mut self) -> INT_FIRST_W<5> {
        INT_FIRST_W::new(self)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub fn int_base(&mut self) -> INT_BASE_W<10> {
        INT_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address where INT PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_base_addr](index.html) module"]
pub struct INT_PTD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for INT_PTD_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ptd_base_addr::R](R) reader structure"]
impl crate::Readable for INT_PTD_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ptd_base_addr::W](W) writer structure"]
impl crate::Writable for INT_PTD_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_PTD_BASE_ADDR to value 0"]
impl crate::Resettable for INT_PTD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
