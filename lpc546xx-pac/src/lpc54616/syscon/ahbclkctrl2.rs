#[doc = "Register `AHBCLKCTRL2` reader"]
pub struct R(crate::R<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL2` writer"]
pub struct W(crate::W<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL2_SPEC>;
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
impl From<crate::W<AHBCLKCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD` reader - Enables the clock for the LCD interface."]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - Enables the clock for the LCD interface."]
pub type LCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `SDIO` reader - Enables the clock for the SDIO interface."]
pub type SDIO_R = crate::BitReader<bool>;
#[doc = "Field `SDIO` writer - Enables the clock for the SDIO interface."]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1H` reader - Enables the clock for the USB1 host interface."]
pub type USB1H_R = crate::BitReader<bool>;
#[doc = "Field `USB1H` writer - Enables the clock for the USB1 host interface."]
pub type USB1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1D` reader - Enables the clock for the USB1 device interface."]
pub type USB1D_R = crate::BitReader<bool>;
#[doc = "Field `USB1D` writer - Enables the clock for the USB1 device interface."]
pub type USB1D_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1RAM` reader - Enables the clock for the USB1 RAM interface."]
pub type USB1RAM_R = crate::BitReader<bool>;
#[doc = "Field `USB1RAM` writer - Enables the clock for the USB1 RAM interface."]
pub type USB1RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `EMC` reader - Enables the clock for the EMC interface."]
pub type EMC_R = crate::BitReader<bool>;
#[doc = "Field `EMC` writer - Enables the clock for the EMC interface."]
pub type EMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `ETH` reader - Enables the clock for the ethernet interface."]
pub type ETH_R = crate::BitReader<bool>;
#[doc = "Field `ETH` writer - Enables the clock for the ethernet interface."]
pub type ETH_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `GPIO4` reader - Enables the clock for the GPIO4 interface."]
pub type GPIO4_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4` writer - Enables the clock for the GPIO4 interface."]
pub type GPIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `GPIO5` reader - Enables the clock for the GPIO5 interface."]
pub type GPIO5_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5` writer - Enables the clock for the GPIO5 interface."]
pub type GPIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `AES` reader - Enables the clock for the AES interface."]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - Enables the clock for the AES interface."]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `OTP` reader - Enables the clock for the OTP interface."]
pub type OTP_R = crate::BitReader<bool>;
#[doc = "Field `OTP` writer - Enables the clock for the OTP interface."]
pub type OTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `RNG` reader - Enables the clock for the RNG interface."]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `RNG` writer - Enables the clock for the RNG interface."]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM8` reader - Enables the clock for the Flexcomm8 interface."]
pub type FLEXCOMM8_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM8` writer - Enables the clock for the Flexcomm8 interface."]
pub type FLEXCOMM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM9` reader - Enables the clock for the Flexcomm9 interface."]
pub type FLEXCOMM9_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM9` writer - Enables the clock for the Flexcomm9 interface."]
pub type FLEXCOMM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `USB0HMR` reader - Enables the clock for the USB host master interface."]
pub type USB0HMR_R = crate::BitReader<bool>;
#[doc = "Field `USB0HMR` writer - Enables the clock for the USB host master interface."]
pub type USB0HMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `USB0HSL` reader - Enables the clock for the USB host slave interface."]
pub type USB0HSL_R = crate::BitReader<bool>;
#[doc = "Field `USB0HSL` writer - Enables the clock for the USB host slave interface."]
pub type USB0HSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `SHA0` reader - Enables the clock for the SHA interface."]
pub type SHA0_R = crate::BitReader<bool>;
#[doc = "Field `SHA0` writer - Enables the clock for the SHA interface."]
pub type SHA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `SC0` reader - Enables the clock for the Smart card0 interface."]
pub type SC0_R = crate::BitReader<bool>;
#[doc = "Field `SC0` writer - Enables the clock for the Smart card0 interface."]
pub type SC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
#[doc = "Field `SC1` reader - Enables the clock for the Smart card1 interface."]
pub type SC1_R = crate::BitReader<bool>;
#[doc = "Field `SC1` writer - Enables the clock for the Smart card1 interface."]
pub type SC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&self) -> USB1H_R {
        USB1H_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&self) -> USB1D_R {
        USB1D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&self) -> USB1RAM_R {
        USB1RAM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&self) -> USB0HMR_R {
        USB0HMR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&self) -> USB0HSL_R {
        USB0HSL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&self) -> SHA0_R {
        SHA0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&self) -> SC0_R {
        SC0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&self) -> SC1_R {
        SC1_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W<2> {
        LCD_W::new(self)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W<3> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&mut self) -> USB1H_W<4> {
        USB1H_W::new(self)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&mut self) -> USB1D_W<5> {
        USB1D_W::new(self)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&mut self) -> USB1RAM_W<6> {
        USB1RAM_W::new(self)
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&mut self) -> EMC_W<7> {
        EMC_W::new(self)
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W<8> {
        ETH_W::new(self)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W<9> {
        GPIO4_W::new(self)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W<10> {
        GPIO5_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<11> {
        AES_W::new(self)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W<12> {
        OTP_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W<13> {
        RNG_W::new(self)
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<14> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<15> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&mut self) -> USB0HMR_W<16> {
        USB0HMR_W::new(self)
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&mut self) -> USB0HSL_W<17> {
        USB0HSL_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&mut self) -> SHA0_W<18> {
        SHA0_W::new(self)
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&mut self) -> SC0_W<19> {
        SC0_W::new(self)
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&mut self) -> SC1_W<20> {
        SC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl2](index.html) module"]
pub struct AHBCLKCTRL2_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl2::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl2::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL2 to value 0"]
impl crate::Resettable for AHBCLKCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
