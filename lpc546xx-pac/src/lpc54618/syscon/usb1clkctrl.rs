#[doc = "Register `USB1CLKCTRL` reader"]
pub struct R(crate::R<USB1CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1CLKCTRL` writer"]
pub struct W(crate::W<USB1CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1CLKCTRL_SPEC>;
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
impl From<crate::W<USB1CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_FS_DEV_CLK` reader - USB1 Device need_clock signal control."]
pub type AP_FS_DEV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `AP_FS_DEV_CLK` writer - USB1 Device need_clock signal control."]
pub type AP_FS_DEV_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKCTRL_SPEC, bool, O>;
#[doc = "Field `POL_FS_DEV_CLK` reader - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt."]
pub type POL_FS_DEV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `POL_FS_DEV_CLK` writer - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt."]
pub type POL_FS_DEV_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKCTRL_SPEC, bool, O>;
#[doc = "Field `AP_FS_HOST_CLK` reader - USB1 Host need_clock signal control."]
pub type AP_FS_HOST_CLK_R = crate::BitReader<bool>;
#[doc = "Field `AP_FS_HOST_CLK` writer - USB1 Host need_clock signal control."]
pub type AP_FS_HOST_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKCTRL_SPEC, bool, O>;
#[doc = "Field `POL_FS_HOST_CLK` reader - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt."]
pub type POL_FS_HOST_CLK_R = crate::BitReader<bool>;
#[doc = "Field `POL_FS_HOST_CLK` writer - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt."]
pub type POL_FS_HOST_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKCTRL_SPEC, bool, O>;
#[doc = "Field `HS_DEV_WAKEUP_N` reader - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic."]
pub type HS_DEV_WAKEUP_N_R = crate::BitReader<bool>;
#[doc = "Field `HS_DEV_WAKEUP_N` writer - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic."]
pub type HS_DEV_WAKEUP_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1CLKCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB1 Device need_clock signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB1 Host need_clock signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_N_R {
        HS_DEV_WAKEUP_N_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 Device need_clock signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W<0> {
        AP_FS_DEV_CLK_W::new(self)
    }
    #[doc = "Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W<1> {
        POL_FS_DEV_CLK_W::new(self)
    }
    #[doc = "Bit 2 - USB1 Host need_clock signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W<2> {
        AP_FS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W<3> {
        POL_FS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&mut self) -> HS_DEV_WAKEUP_N_W<4> {
        HS_DEV_WAKEUP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkctrl](index.html) module"]
pub struct USB1CLKCTRL_SPEC;
impl crate::RegisterSpec for USB1CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1clkctrl::R](R) reader structure"]
impl crate::Readable for USB1CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1clkctrl::W](W) writer structure"]
impl crate::Writable for USB1CLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1CLKCTRL to value 0x10"]
impl crate::Resettable for USB1CLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
