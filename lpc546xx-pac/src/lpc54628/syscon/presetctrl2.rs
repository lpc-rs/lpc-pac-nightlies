#[doc = "Register `PRESETCTRL2` reader"]
pub struct R(crate::R<PRESETCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL2` writer"]
pub struct W(crate::W<PRESETCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL2_SPEC>;
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
impl From<crate::W<PRESETCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_RST` reader - LCD reset control."]
pub type LCD_RST_R = crate::BitReader<bool>;
#[doc = "Field `LCD_RST` writer - LCD reset control."]
pub type LCD_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `SDIO_RST` reader - SDIO reset control."]
pub type SDIO_RST_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_RST` writer - SDIO reset control."]
pub type SDIO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1H_RST` reader - USB1 Host reset control."]
pub type USB1H_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB1H_RST` writer - USB1 Host reset control."]
pub type USB1H_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1D_RST` reader - USB1 Device reset control."]
pub type USB1D_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB1D_RST` writer - USB1 Device reset control."]
pub type USB1D_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `USB1RAM_RST` reader - USB1 RAM reset control."]
pub type USB1RAM_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB1RAM_RST` writer - USB1 RAM reset control."]
pub type USB1RAM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `EMC_RESET` reader - EMC reset control."]
pub type EMC_RESET_R = crate::BitReader<bool>;
#[doc = "Field `EMC_RESET` writer - EMC reset control."]
pub type EMC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `ETH_RST` reader - Ethernet reset control."]
pub type ETH_RST_R = crate::BitReader<bool>;
#[doc = "Field `ETH_RST` writer - Ethernet reset control."]
pub type ETH_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `GPIO4_RST` reader - GPIO4 reset control."]
pub type GPIO4_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_RST` writer - GPIO4 reset control."]
pub type GPIO4_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `GPIO5_RST` reader - GPIO5 reset control."]
pub type GPIO5_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_RST` writer - GPIO5 reset control."]
pub type GPIO5_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `AES_RST` reader - AES reset control."]
pub type AES_RST_R = crate::BitReader<bool>;
#[doc = "Field `AES_RST` writer - AES reset control."]
pub type AES_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `OTP_RST` reader - OTP reset control."]
pub type OTP_RST_R = crate::BitReader<bool>;
#[doc = "Field `OTP_RST` writer - OTP reset control."]
pub type OTP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `RNG_RST` reader - RNG reset control."]
pub type RNG_RST_R = crate::BitReader<bool>;
#[doc = "Field `RNG_RST` writer - RNG reset control."]
pub type RNG_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `FC8_RST` reader - Flexcomm 8 reset control."]
pub type FC8_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC8_RST` writer - Flexcomm 8 reset control."]
pub type FC8_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `FC9_RST` reader - Flexcomm 9 reset control."]
pub type FC9_RST_R = crate::BitReader<bool>;
#[doc = "Field `FC9_RST` writer - Flexcomm 9 reset control."]
pub type FC9_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `USB0HMR_RST` reader - USB0 HOST master reset control."]
pub type USB0HMR_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB0HMR_RST` writer - USB0 HOST master reset control."]
pub type USB0HMR_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `USB0HSL_RST` reader - USB0 HOST slave reset control."]
pub type USB0HSL_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB0HSL_RST` writer - USB0 HOST slave reset control."]
pub type USB0HSL_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `SHA_RST` reader - SHA reset control."]
pub type SHA_RST_R = crate::BitReader<bool>;
#[doc = "Field `SHA_RST` writer - SHA reset control."]
pub type SHA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `SC0_RST` reader - Smart card 0 reset control."]
pub type SC0_RST_R = crate::BitReader<bool>;
#[doc = "Field `SC0_RST` writer - Smart card 0 reset control."]
pub type SC0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
#[doc = "Field `SC1_RST` reader - Smart card 1 reset control."]
pub type SC1_RST_R = crate::BitReader<bool>;
#[doc = "Field `SC1_RST` writer - Smart card 1 reset control."]
pub type SC1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&self) -> LCD_RST_R {
        LCD_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&self) -> USB1H_RST_R {
        USB1H_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&self) -> USB1D_RST_R {
        USB1D_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&self) -> USB1RAM_RST_R {
        USB1RAM_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&self) -> EMC_RESET_R {
        EMC_RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&self) -> ETH_RST_R {
        ETH_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&self) -> GPIO4_RST_R {
        GPIO4_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&self) -> GPIO5_RST_R {
        GPIO5_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&self) -> AES_RST_R {
        AES_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&self) -> OTP_RST_R {
        OTP_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&self) -> FC8_RST_R {
        FC8_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&self) -> FC9_RST_R {
        FC9_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&self) -> USB0HMR_RST_R {
        USB0HMR_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&self) -> USB0HSL_RST_R {
        USB0HSL_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&self) -> SHA_RST_R {
        SHA_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&self) -> SC0_RST_R {
        SC0_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&self) -> SC1_RST_R {
        SC1_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&mut self) -> LCD_RST_W<2> {
        LCD_RST_W::new(self)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<3> {
        SDIO_RST_W::new(self)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&mut self) -> USB1H_RST_W<4> {
        USB1H_RST_W::new(self)
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&mut self) -> USB1D_RST_W<5> {
        USB1D_RST_W::new(self)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&mut self) -> USB1RAM_RST_W<6> {
        USB1RAM_RST_W::new(self)
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&mut self) -> EMC_RESET_W<7> {
        EMC_RESET_W::new(self)
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&mut self) -> ETH_RST_W<8> {
        ETH_RST_W::new(self)
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&mut self) -> GPIO4_RST_W<9> {
        GPIO4_RST_W::new(self)
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&mut self) -> GPIO5_RST_W<10> {
        GPIO5_RST_W::new(self)
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&mut self) -> AES_RST_W<11> {
        AES_RST_W::new(self)
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&mut self) -> OTP_RST_W<12> {
        OTP_RST_W::new(self)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W<13> {
        RNG_RST_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&mut self) -> FC8_RST_W<14> {
        FC8_RST_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&mut self) -> FC9_RST_W<15> {
        FC9_RST_W::new(self)
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&mut self) -> USB0HMR_RST_W<16> {
        USB0HMR_RST_W::new(self)
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&mut self) -> USB0HSL_RST_W<17> {
        USB0HSL_RST_W::new(self)
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&mut self) -> SHA_RST_W<18> {
        SHA_RST_W::new(self)
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&mut self) -> SC0_RST_W<19> {
        SC0_RST_W::new(self)
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&mut self) -> SC1_RST_W<20> {
        SC1_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl2](index.html) module"]
pub struct PRESETCTRL2_SPEC;
impl crate::RegisterSpec for PRESETCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl2::R](R) reader structure"]
impl crate::Readable for PRESETCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl2::W](W) writer structure"]
impl crate::Writable for PRESETCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL2 to value 0"]
impl crate::Resettable for PRESETCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
