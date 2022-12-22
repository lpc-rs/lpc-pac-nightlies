#[doc = "Register `MAC_TIMESTAMP_CTRL` reader"]
pub struct R(crate::R<MAC_TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TIMESTAMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TIMESTAMP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TIMESTAMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_TIMESTAMP_CTRL` writer"]
pub struct W(crate::W<MAC_TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TIMESTAMP_CTRL_SPEC>;
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
impl From<crate::W<MAC_TIMESTAMP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TIMESTAMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENA` reader - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
pub type TSENA_R = crate::BitReader<bool>;
#[doc = "Field `TSENA` writer - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
pub type TSENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSCFUPDT` reader - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
pub type TSCFUPDT_R = crate::BitReader<bool>;
#[doc = "Field `TSCFUPDT` writer - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
pub type TSCFUPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSINIT` reader - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
pub type TSINIT_R = crate::BitReader<bool>;
#[doc = "Field `TSINIT` writer - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
pub type TSINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSUPDT` reader - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
pub type TSUPDT_R = crate::BitReader<bool>;
#[doc = "Field `TSUPDT` writer - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
pub type TSUPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSTRIG` reader - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
pub type TSTRIG_R = crate::BitReader<bool>;
#[doc = "Field `TSTRIG` writer - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
pub type TSTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TADDREG` reader - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
pub type TADDREG_R = crate::BitReader<bool>;
#[doc = "Field `TADDREG` writer - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
pub type TADDREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub type TSENALL_R = crate::BitReader<bool>;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub type TSENALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
pub type TSCTRLSSR_R = crate::BitReader<bool>;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
pub type TSCTRLSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
pub type TSVER2ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSVER2ENA` writer - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
pub type TSVER2ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
pub type TSIPENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
pub type TSIPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
pub type TSIPV6ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
pub type TSIPV6ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
pub type TSIPV4ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
pub type TSIPV4ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSEVTENA` reader - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
pub type TSEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `TSEVTENA` writer - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
pub type TSEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
pub type TSMSTRENA_R = crate::BitReader<bool>;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
pub type TSMSTRENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
pub type SNAPTYPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
pub type SNAPTYPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
pub type TSENMACADDR_R = crate::BitReader<bool>;
#[doc = "Field `TSENMACADDR` writer - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
pub type TSENMACADDR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `TXTTSSTSM` reader - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
pub type TXTTSSTSM_R = crate::BitReader<bool>;
#[doc = "Field `TXTTSSTSM` writer - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
pub type TXTTSSTSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
#[doc = "Field `AV8021ASMEN` reader - AV 802."]
pub type AV8021ASMEN_R = crate::BitReader<bool>;
#[doc = "Field `AV8021ASMEN` writer - AV 802."]
pub type AV8021ASMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAC_TIMESTAMP_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&self) -> TADDREG_R {
        TADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&self) -> TSEVTENA_R {
        TSEVTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&self) -> TXTTSSTSM_R {
        TXTTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W<0> {
        TSENA_W::new(self)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<1> {
        TSCFUPDT_W::new(self)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W<2> {
        TSINIT_W::new(self)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W<3> {
        TSUPDT_W::new(self)
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TSTRIG_W<4> {
        TSTRIG_W::new(self)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&mut self) -> TADDREG_W<5> {
        TADDREG_W::new(self)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W<8> {
        TSENALL_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<9> {
        TSCTRLSSR_W::new(self)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<10> {
        TSVER2ENA_W::new(self)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W<11> {
        TSIPENA_W::new(self)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<12> {
        TSIPV6ENA_W::new(self)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<13> {
        TSIPV4ENA_W::new(self)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&mut self) -> TSEVTENA_W<14> {
        TSEVTENA_W::new(self)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<15> {
        TSMSTRENA_W::new(self)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<16> {
        SNAPTYPSEL_W::new(self)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<18> {
        TSENMACADDR_W::new(self)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&mut self) -> TXTTSSTSM_W<24> {
        TXTTSSTSM_W::new(self)
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W<28> {
        AV8021ASMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_timestamp_ctrl](index.html) module"]
pub struct MAC_TIMESTAMP_CTRL_SPEC;
impl crate::RegisterSpec for MAC_TIMESTAMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_timestamp_ctrl::R](R) reader structure"]
impl crate::Readable for MAC_TIMESTAMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_timestamp_ctrl::W](W) writer structure"]
impl crate::Writable for MAC_TIMESTAMP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_TIMESTAMP_CTRL to value 0x2000"]
impl crate::Resettable for MAC_TIMESTAMP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
