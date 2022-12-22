#[doc = "Register `USB1CLKDIV` reader"]
pub struct R(crate::R<USB1CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1CLKDIV` writer"]
pub struct W(crate::W<USB1CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1CLKDIV_SPEC>;
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
impl From<crate::W<USB1CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Clock divider value."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB1CLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESET` reader - Resets the divider counter."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Resets the divider counter."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKDIV_SPEC, bool, O>;
#[doc = "Field `HALT` reader - Halts the divider counter."]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - Halts the divider counter."]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKDIV_SPEC, bool, O>;
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub type REQFLAG_R = crate::BitReader<bool>;
#[doc = "Field `REQFLAG` writer - Divider status flag."]
pub type REQFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<29> {
        RESET_W::new(self)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W<30> {
        HALT_W::new(self)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&mut self) -> REQFLAG_W<31> {
        REQFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkdiv](index.html) module"]
pub struct USB1CLKDIV_SPEC;
impl crate::RegisterSpec for USB1CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1clkdiv::R](R) reader structure"]
impl crate::Readable for USB1CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1clkdiv::W](W) writer structure"]
impl crate::Writable for USB1CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1CLKDIV to value 0x4000_0000"]
impl crate::Resettable for USB1CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
