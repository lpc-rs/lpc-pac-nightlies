#[doc = "Register `MAC_PMT_CRTL_STAT` reader"]
pub struct R(crate::R<MAC_PMT_CRTL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_PMT_CRTL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_PMT_CRTL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_PMT_CRTL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_PMT_CRTL_STAT` writer"]
pub struct W(crate::W<MAC_PMT_CRTL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_PMT_CRTL_STAT_SPEC>;
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
impl From<crate::W<MAC_PMT_CRTL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_PMT_CRTL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable."]
pub type MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `RWKPKTEN` reader - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
pub type RWKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received."]
pub type MGKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `RWKPRCVD` reader - Remote Wake-Up Packet Received."]
pub type RWKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` reader - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
pub type GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` writer - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
pub type GLBLUCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_PMT_CRTL_STAT_SPEC, bool, O>;
#[doc = "Field `RWKPFE` reader - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
pub type RWKPFE_R = crate::BitReader<bool>;
#[doc = "Field `RWKPFE` writer - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
pub type RWKPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_PMT_CRTL_STAT_SPEC, bool, O>;
#[doc = "Field `RWKPTR` reader - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
pub type RWKPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWKPTR` writer - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
pub type RWKPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_PMT_CRTL_STAT_SPEC, u8, u8, 5, O>;
#[doc = "Field `RWKFILTRST` reader - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
pub type RWKFILTRST_R = crate::BitReader<bool>;
#[doc = "Field `RWKFILTRST` writer - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
pub type RWKFILTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_PMT_CRTL_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Packet Received."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<10> {
        RWKPFE_W::new(self)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RWKPTR_W<24> {
        RWKPTR_W::new(self)
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_pmt_crtl_stat](index.html) module"]
pub struct MAC_PMT_CRTL_STAT_SPEC;
impl crate::RegisterSpec for MAC_PMT_CRTL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_pmt_crtl_stat::R](R) reader structure"]
impl crate::Readable for MAC_PMT_CRTL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_pmt_crtl_stat::W](W) writer structure"]
impl crate::Writable for MAC_PMT_CRTL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_PMT_CRTL_STAT to value 0"]
impl crate::Resettable for MAC_PMT_CRTL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
