#[doc = "Register `MAC_FRAME_FILTER` reader"]
pub struct R(crate::R<MAC_FRAME_FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FRAME_FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FRAME_FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FRAME_FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FRAME_FILTER` writer"]
pub struct W(crate::W<MAC_FRAME_FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FRAME_FILTER_SPEC>;
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
impl From<crate::W<MAC_FRAME_FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FRAME_FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_FRAME_FILTER_SPEC, bool, O>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_FRAME_FILTER_SPEC, bool, O>;
#[doc = "Field `PM` reader - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_FRAME_FILTER_SPEC, bool, O>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_FRAME_FILTER_SPEC, bool, O>;
#[doc = "Field `PCF` reader - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
pub type PCF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF` writer - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
pub type PCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_FRAME_FILTER_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison."]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` reader - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers."]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `RA` reader - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
pub type RA_R = crate::BitReader<bool>;
#[doc = "Field `RA` writer - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_FRAME_FILTER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison."]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers."]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<4> {
        PM_W::new(self)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<5> {
        DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_frame_filter](index.html) module"]
pub struct MAC_FRAME_FILTER_SPEC;
impl crate::RegisterSpec for MAC_FRAME_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_frame_filter::R](R) reader structure"]
impl crate::Readable for MAC_FRAME_FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_frame_filter::W](W) writer structure"]
impl crate::Writable for MAC_FRAME_FILTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FRAME_FILTER to value 0"]
impl crate::Resettable for MAC_FRAME_FILTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
