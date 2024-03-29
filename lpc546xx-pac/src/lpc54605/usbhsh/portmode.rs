#[doc = "Register `PORTMODE` reader"]
pub struct R(crate::R<PORTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTMODE` writer"]
pub struct W(crate::W<PORTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTMODE_SPEC>;
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
impl From<crate::W<PORTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID0` reader - Port 0 ID pin value."]
pub type ID0_R = crate::BitReader<bool>;
#[doc = "Field `ID0` writer - Port 0 ID pin value."]
pub type ID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `ID0_EN` reader - Port 0 ID pin pull-up enable."]
pub type ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `ID0_EN` writer - Port 0 ID pin pull-up enable."]
pub type ID0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `DEV_ENABLE` reader - If this bit is set to one, one of the ports will behave as a USB device."]
pub type DEV_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DEV_ENABLE` writer - If this bit is set to one, one of the ports will behave as a USB device."]
pub type DEV_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `SW_CTRL_PDCOM` reader - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
pub type SW_CTRL_PDCOM_R = crate::BitReader<bool>;
#[doc = "Field `SW_CTRL_PDCOM` writer - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
pub type SW_CTRL_PDCOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `SW_PDCOM` reader - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
pub type SW_PDCOM_R = crate::BitReader<bool>;
#[doc = "Field `SW_PDCOM` writer - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
pub type SW_PDCOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&self) -> ID0_EN_R {
        ID0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&self) -> SW_CTRL_PDCOM_R {
        SW_CTRL_PDCOM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&self) -> SW_PDCOM_R {
        SW_PDCOM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&mut self) -> ID0_W<0> {
        ID0_W::new(self)
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&mut self) -> ID0_EN_W<8> {
        ID0_EN_W::new(self)
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W<16> {
        DEV_ENABLE_W::new(self)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&mut self) -> SW_CTRL_PDCOM_W<18> {
        SW_CTRL_PDCOM_W::new(self)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&mut self) -> SW_PDCOM_W<19> {
        SW_PDCOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](index.html) module"]
pub struct PORTMODE_SPEC;
impl crate::RegisterSpec for PORTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portmode::R](R) reader structure"]
impl crate::Readable for PORTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portmode::W](W) writer structure"]
impl crate::Writable for PORTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTMODE to value 0x0004_0000"]
impl crate::Resettable for PORTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0000
    }
}
