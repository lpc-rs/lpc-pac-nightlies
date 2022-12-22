#[doc = "Register `NMISRC` reader"]
pub struct R(crate::R<NMISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMISRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMISRC` writer"]
pub struct W(crate::W<NMISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMISRC_SPEC>;
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
impl From<crate::W<NMISRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQM4` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
pub type IRQM4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQM4` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
pub type IRQM4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NMISRC_SPEC, u8, u8, 6, O>;
#[doc = "Field `NMIENM4` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
pub type NMIENM4_R = crate::BitReader<bool>;
#[doc = "Field `NMIENM4` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
pub type NMIENM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NMISRC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
    #[inline(always)]
    pub fn irqm4(&self) -> IRQM4_R {
        IRQM4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
    #[inline(always)]
    pub fn nmienm4(&self) -> NMIENM4_R {
        NMIENM4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
    #[inline(always)]
    pub fn irqm4(&mut self) -> IRQM4_W<0> {
        IRQM4_W::new(self)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
    #[inline(always)]
    pub fn nmienm4(&mut self) -> NMIENM4_W<31> {
        NMIENM4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](index.html) module"]
pub struct NMISRC_SPEC;
impl crate::RegisterSpec for NMISRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmisrc::R](R) reader structure"]
impl crate::Readable for NMISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](W) writer structure"]
impl crate::Writable for NMISRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NMISRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
