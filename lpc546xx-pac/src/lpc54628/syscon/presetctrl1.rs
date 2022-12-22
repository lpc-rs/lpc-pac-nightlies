#[doc = "Register `PRESETCTRL1` reader"]
pub struct R(crate::R<PRESETCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL1` writer"]
pub struct W(crate::W<PRESETCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL1_SPEC>;
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
impl From<crate::W<PRESETCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT_RST` reader - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type MRT_RST_R = crate::BitReader<bool>;
#[doc = "Field `MRT_RST` writer - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type MRT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `SCT0_RST` reader - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type SCT0_RST_R = crate::BitReader<bool>;
#[doc = "Field `SCT0_RST` writer - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type SCT0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `MCAN0_RST` reader - 0 = Clear reset to this function."]
pub type MCAN0_RST_R = crate::BitReader<bool>;
#[doc = "Field `MCAN0_RST` writer - 0 = Clear reset to this function."]
pub type MCAN0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `MCAN1_RST` reader - 0 = Clear reset to this function."]
pub type MCAN1_RST_R = crate::BitReader<bool>;
#[doc = "Field `MCAN1_RST` writer - 0 = Clear reset to this function."]
pub type MCAN1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `UTICK_RST` reader - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type UTICK_RST_R = crate::BitReader<bool>;
#[doc = "Field `UTICK_RST` writer - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type UTICK_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC0_RST` reader - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC0_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC0_RST` writer - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC1_RST` reader - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC1_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC1_RST` writer - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC2_RST` reader - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC2_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC2_RST` writer - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC3_RST` reader - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC3_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC3_RST` writer - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC3_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC4_RST` reader - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC4_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC4_RST` writer - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC4_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC5_RST` reader - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC5_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC5_RST` writer - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC5_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC6_RST` reader - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC6_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC6_RST` writer - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC6_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `FC7_RST` reader - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC7_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC7_RST` writer - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FC7_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `DMIC_RST` reader - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type DMIC_RST_R = crate::BitReader<bool>;
#[doc = "Field `DMIC_RST` writer - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type DMIC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER2_RST` reader - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
pub type CTIMER2_RST_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER2_RST` writer - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
pub type CTIMER2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `USB0D_RST` reader - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type USB0D_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB0D_RST` writer - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type USB0D_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER0_RST` reader - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CTIMER0_RST_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER0_RST` writer - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CTIMER0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
#[doc = "Field `CTIMER1_RST` reader - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CTIMER1_RST_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER1_RST` writer - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CTIMER1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MRT_RST_R {
        MRT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&self) -> SCT0_RST_R {
        SCT0_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&self) -> MCAN0_RST_R {
        MCAN0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&self) -> MCAN1_RST_R {
        MCAN1_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UTICK_RST_R {
        UTICK_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> FC0_RST_R {
        FC0_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> FC1_RST_R {
        FC1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> FC2_RST_R {
        FC2_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> FC3_RST_R {
        FC3_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> FC4_RST_R {
        FC4_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> FC5_RST_R {
        FC5_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> FC6_RST_R {
        FC6_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> FC7_RST_R {
        FC7_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&self) -> DMIC_RST_R {
        DMIC_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&self) -> CTIMER2_RST_R {
        CTIMER2_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&self) -> USB0D_RST_R {
        USB0D_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&self) -> CTIMER0_RST_R {
        CTIMER0_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&self) -> CTIMER1_RST_R {
        CTIMER1_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&mut self) -> MRT_RST_W<0> {
        MRT_RST_W::new(self)
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&mut self) -> SCT0_RST_W<2> {
        SCT0_RST_W::new(self)
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&mut self) -> MCAN0_RST_W<7> {
        MCAN0_RST_W::new(self)
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&mut self) -> MCAN1_RST_W<8> {
        MCAN1_RST_W::new(self)
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&mut self) -> UTICK_RST_W<10> {
        UTICK_RST_W::new(self)
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&mut self) -> FC0_RST_W<11> {
        FC0_RST_W::new(self)
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&mut self) -> FC1_RST_W<12> {
        FC1_RST_W::new(self)
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&mut self) -> FC2_RST_W<13> {
        FC2_RST_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&mut self) -> FC3_RST_W<14> {
        FC3_RST_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&mut self) -> FC4_RST_W<15> {
        FC4_RST_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&mut self) -> FC5_RST_W<16> {
        FC5_RST_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&mut self) -> FC6_RST_W<17> {
        FC6_RST_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&mut self) -> FC7_RST_W<18> {
        FC7_RST_W::new(self)
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&mut self) -> DMIC_RST_W<19> {
        DMIC_RST_W::new(self)
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&mut self) -> CTIMER2_RST_W<22> {
        CTIMER2_RST_W::new(self)
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&mut self) -> USB0D_RST_W<25> {
        USB0D_RST_W::new(self)
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&mut self) -> CTIMER0_RST_W<26> {
        CTIMER0_RST_W::new(self)
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&mut self) -> CTIMER1_RST_W<27> {
        CTIMER1_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl1](index.html) module"]
pub struct PRESETCTRL1_SPEC;
impl crate::RegisterSpec for PRESETCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl1::R](R) reader structure"]
impl crate::Readable for PRESETCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](W) writer structure"]
impl crate::Writable for PRESETCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0"]
impl crate::Resettable for PRESETCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
