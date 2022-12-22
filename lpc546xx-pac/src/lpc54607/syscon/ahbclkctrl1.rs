#[doc = "Register `AHBCLKCTRL1` reader"]
pub struct R(crate::R<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL1` writer"]
pub struct W(crate::W<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL1_SPEC>;
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
impl From<crate::W<AHBCLKCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT` reader - Enables the clock for the Multi-Rate Timer."]
pub type MRT_R = crate::BitReader<bool>;
#[doc = "Field `MRT` writer - Enables the clock for the Multi-Rate Timer."]
pub type MRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `RIT` reader - Enables the clock for the Repetitive Interrupt Timer."]
pub type RIT_R = crate::BitReader<bool>;
#[doc = "Field `RIT` writer - Enables the clock for the Repetitive Interrupt Timer."]
pub type RIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `SCT0` reader - Enables the clock for SCT0."]
pub type SCT0_R = crate::BitReader<bool>;
#[doc = "Field `SCT0` writer - Enables the clock for SCT0."]
pub type SCT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `MCAN0` reader - Enables the clock for MCAN0."]
pub type MCAN0_R = crate::BitReader<bool>;
#[doc = "Field `MCAN0` writer - Enables the clock for MCAN0."]
pub type MCAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `MCAN1` reader - Enables the clock for MCAN1."]
pub type MCAN1_R = crate::BitReader<bool>;
#[doc = "Field `MCAN1` writer - Enables the clock for MCAN1."]
pub type MCAN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `UTICK` reader - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
pub type UTICK_R = crate::BitReader<bool>;
#[doc = "Field `UTICK` writer - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
pub type UTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM0` reader - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM0_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM0` writer - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM1` reader - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM1_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM1` writer - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM2` reader - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM2_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM2` writer - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM3` reader - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM3_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM3` writer - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM4` reader - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM4_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM4` writer - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM5` reader - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM5_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM5` writer - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM6` reader - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM6_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM6` writer - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM7` reader - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM7_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM7` writer - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
pub type FLEXCOMM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `DMIC` reader - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
pub type DMIC_R = crate::BitReader<bool>;
#[doc = "Field `DMIC` writer - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
pub type DMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER2` reader - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
pub type CTIMER2_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER2` writer - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
pub type CTIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `USB0D` reader - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
pub type USB0D_R = crate::BitReader<bool>;
#[doc = "Field `USB0D` writer - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
pub type USB0D_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER0` reader - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
pub type CTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER0` writer - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
pub type CTIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER1` reader - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
pub type CTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER1` writer - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
pub type CTIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&self) -> RIT_R {
        RIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&self) -> MCAN1_R {
        MCAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&self) -> USB0D_R {
        USB0D_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W<0> {
        MRT_W::new(self)
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&mut self) -> RIT_W<1> {
        RIT_W::new(self)
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W<2> {
        SCT0_W::new(self)
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&mut self) -> MCAN0_W<7> {
        MCAN0_W::new(self)
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&mut self) -> MCAN1_W<8> {
        MCAN1_W::new(self)
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W<10> {
        UTICK_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<11> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<12> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<13> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<14> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<15> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<16> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<17> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<18> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W<19> {
        DMIC_W::new(self)
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W<22> {
        CTIMER2_W::new(self)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&mut self) -> USB0D_W<25> {
        USB0D_W::new(self)
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W<26> {
        CTIMER0_W::new(self)
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W<27> {
        CTIMER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl1](index.html) module"]
pub struct AHBCLKCTRL1_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl1::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl1::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL1 to value 0"]
impl crate::Resettable for AHBCLKCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
