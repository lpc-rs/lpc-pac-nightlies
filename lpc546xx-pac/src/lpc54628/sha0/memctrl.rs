#[doc = "Register `MEMCTRL` reader"]
pub struct R(crate::R<MEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMCTRL` writer"]
pub struct W(crate::W<MEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCTRL_SPEC>;
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
impl From<crate::W<MEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - This field is used to enable SHA block as AHB bus master."]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - This field is used to enable SHA block as AHB bus master."]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMCTRL_SPEC, bool, O>;
#[doc = "Field `COUNT` reader - This field indicates the number of 512-bit blocks to copy starting at MEMADDR."]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - This field indicates the number of 512-bit blocks to copy starting at MEMADDR."]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMCTRL_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - This field is used to enable SHA block as AHB bus master."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:26 - This field indicates the number of 512-bit blocks to copy starting at MEMADDR."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to enable SHA block as AHB bus master."]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bits 16:26 - This field indicates the number of 512-bit blocks to copy starting at MEMADDR."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<16> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memctrl](index.html) module"]
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memctrl::R](R) reader structure"]
impl crate::Readable for MEMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memctrl::W](W) writer structure"]
impl crate::Writable for MEMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MEMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
