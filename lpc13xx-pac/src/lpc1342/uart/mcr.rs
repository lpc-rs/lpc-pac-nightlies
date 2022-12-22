#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTRCTRL` reader - Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DTRCTRL_R = crate::BitReader<bool>;
#[doc = "Field `DTRCTRL` writer - Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DTRCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RTSCTRL` reader - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RTSCTRL_R = crate::BitReader<bool>;
#[doc = "Field `RTSCTRL` writer - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RTSCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `LMS` reader - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
pub type LMS_R = crate::BitReader<bool>;
#[doc = "Field `LMS` writer - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
pub type LMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RTSEN_R = crate::BitReader<RTSEN_A>;
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "0: Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO = 0,
    #[doc = "1: Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW = 1,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLE_AUTO_RTS_FLO,
            true => RTSEN_A::ENABLE_AUTO_RTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSEN_A::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSEN_A::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RTSEN_A, O>;
impl<'a, const O: u8> RTSEN_W<'a, O> {
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut W {
        self.variant(RTSEN_A::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut W {
        self.variant(RTSEN_A::ENABLE_AUTO_RTS_FLOW)
    }
}
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO = 0,
    #[doc = "1: Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE_AUTO_CTS_FLO,
            true => CTSEN_A::ENABLE_AUTO_CTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSEN_A::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSEN_A::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, CTSEN_A, O>;
impl<'a, const O: u8> CTSEN_W<'a, O> {
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE_AUTO_CTS_FLOW)
    }
}
impl R {
    #[doc = "Bit 0 - Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&self) -> DTRCTRL_R {
        DTRCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&self) -> RTSCTRL_R {
        RTSCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&mut self) -> DTRCTRL_W<0> {
        DTRCTRL_W::new(self)
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&mut self) -> RTSCTRL_W<1> {
        RTSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&mut self) -> LMS_W<4> {
        LMS_W::new(self)
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W<6> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W<7> {
        CTSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
