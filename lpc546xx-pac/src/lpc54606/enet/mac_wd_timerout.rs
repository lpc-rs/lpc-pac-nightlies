#[doc = "Register `MAC_WD_TIMEROUT` reader"]
pub struct R(crate::R<MAC_WD_TIMEROUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_WD_TIMEROUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_WD_TIMEROUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_WD_TIMEROUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_WD_TIMEROUT` writer"]
pub struct W(crate::W<MAC_WD_TIMEROUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_WD_TIMEROUT_SPEC>;
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
impl From<crate::W<MAC_WD_TIMEROUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_WD_TIMEROUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTO` reader - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
pub type WTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WTO` writer - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
pub type WTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_WD_TIMEROUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
pub type PWE_R = crate::BitReader<bool>;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
pub type PWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_WD_TIMEROUT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W<0> {
        WTO_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W<8> {
        PWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC watchdog Timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_wd_timerout](index.html) module"]
pub struct MAC_WD_TIMEROUT_SPEC;
impl crate::RegisterSpec for MAC_WD_TIMEROUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_wd_timerout::R](R) reader structure"]
impl crate::Readable for MAC_WD_TIMEROUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_wd_timerout::W](W) writer structure"]
impl crate::Writable for MAC_WD_TIMEROUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_WD_TIMEROUT to value 0"]
impl crate::Resettable for MAC_WD_TIMEROUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
