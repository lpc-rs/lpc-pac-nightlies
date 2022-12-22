#[doc = "Register `MAC_CONFIG` reader"]
pub struct R(crate::R<MAC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_CONFIG` writer"]
pub struct W(crate::W<MAC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CONFIG_SPEC>;
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
impl From<crate::W<MAC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
pub type PRELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
pub type PRELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `BL` reader - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DR` reader - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
pub type DCRS_R = crate::BitReader<bool>;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
pub type DCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `DO` reader - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
pub type DO_R = crate::BitReader<bool>;
#[doc = "Field `DO` writer - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
pub type DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `ECRSFD` reader - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
pub type ECRSFD_R = crate::BitReader<bool>;
#[doc = "Field `ECRSFD` writer - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
pub type ECRSFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `LM` reader - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `DM` reader - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `FES` reader - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `PS` reader - Portselect."]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `JE` reader - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JE_R = crate::BitReader<bool>;
#[doc = "Field `JE` writer - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `JD` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `BE` reader - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `WD` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
pub type ACS_R = crate::BitReader<bool>;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
pub type ACS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `CST` reader - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `S2KP` reader - IEEE 802."]
pub type S2KP_R = crate::BitReader<bool>;
#[doc = "Field `S2KP` writer - IEEE 802."]
pub type S2KP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `GPSLCE` reader - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
pub type GPSLCE_R = crate::BitReader<bool>;
#[doc = "Field `GPSLCE` writer - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
pub type GPSLCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `IPG` reader - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
pub type IPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPG` writer - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IPC` reader - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Portselect."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<0> {
        RE_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<1> {
        TE_W::new(self)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W<2> {
        PRELEN_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<8> {
        DR_W::new(self)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W<9> {
        DCRS_W::new(self)
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W<10> {
        DO_W::new(self)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<11> {
        ECRSFD_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<13> {
        DM_W::new(self)
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W<16> {
        JE_W::new(self)
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<17> {
        JD_W::new(self)
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<18> {
        BE_W::new(self)
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<19> {
        WD_W::new(self)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<20> {
        ACS_W::new(self)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<21> {
        CST_W::new(self)
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&mut self) -> S2KP_W<22> {
        S2KP_W::new(self)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&mut self) -> GPSLCE_W<23> {
        GPSLCE_W::new(self)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W<24> {
        IPG_W::new(self)
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<27> {
        IPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_config](index.html) module"]
pub struct MAC_CONFIG_SPEC;
impl crate::RegisterSpec for MAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_config::R](R) reader structure"]
impl crate::Readable for MAC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_config::W](W) writer structure"]
impl crate::Writable for MAC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_CONFIG to value 0x8000"]
impl crate::Resettable for MAC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
