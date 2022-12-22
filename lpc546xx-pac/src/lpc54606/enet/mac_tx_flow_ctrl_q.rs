#[doc = "Register `MAC_TX_FLOW_CTRL_Q[%s]` reader"]
pub struct R(crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_TX_FLOW_CTRL_Q[%s]` writer"]
pub struct W(crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>;
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
impl From<crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB` reader - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
pub type FCB_R = crate::BitReader<bool>;
#[doc = "Field `FCB` writer - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
pub type FCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TX_FLOW_CTRL_Q_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TX_FLOW_CTRL_Q_SPEC, bool, O>;
#[doc = "Field `PLT` reader - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
pub type PLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_TX_FLOW_CTRL_Q_SPEC, u8, u8, 3, O>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TX_FLOW_CTRL_Q_SPEC, bool, O>;
#[doc = "Field `PT` reader - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
pub type PT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_TX_FLOW_CTRL_Q_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W<0> {
        FCB_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_flow_ctrl_q](index.html) module"]
pub struct MAC_TX_FLOW_CTRL_Q_SPEC;
impl crate::RegisterSpec for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_flow_ctrl_q::R](R) reader structure"]
impl crate::Readable for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_tx_flow_ctrl_q::W](W) writer structure"]
impl crate::Writable for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_TX_FLOW_CTRL_Q[%s]
to value 0"]
impl crate::Resettable for MAC_TX_FLOW_CTRL_Q_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
