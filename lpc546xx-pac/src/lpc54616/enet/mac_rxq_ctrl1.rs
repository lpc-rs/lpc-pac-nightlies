#[doc = "Register `MAC_RXQ_CTRL1` reader"]
pub struct R(crate::R<MAC_RXQ_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RXQ_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RXQ_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_RXQ_CTRL1` writer"]
pub struct W(crate::W<MAC_RXQ_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL1_SPEC>;
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
impl From<crate::W<MAC_RXQ_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RXQ_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVCPQ` reader - AV Untagged Control Packets Queue."]
pub type AVCPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVCPQ` writer - AV Untagged Control Packets Queue."]
pub type AVCPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `AVPTPQ` reader - AV PTP Packets Queue."]
pub type AVPTPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVPTPQ` writer - AV PTP Packets Queue."]
pub type AVPTPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `UPQ` reader - Untagged Packet Queue."]
pub type UPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPQ` writer - Untagged Packet Queue."]
pub type UPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCBCQ` reader - Multicast and Broadcast Queue."]
pub type MCBCQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCBCQ` writer - Multicast and Broadcast Queue."]
pub type MCBCQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCBCQEN` reader - Multicast and Broadcast Queue Enable."]
pub type MCBCQEN_R = crate::BitReader<bool>;
#[doc = "Field `MCBCQEN` writer - Multicast and Broadcast Queue Enable."]
pub type MCBCQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_RXQ_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&mut self) -> AVCPQ_W<0> {
        AVCPQ_W::new(self)
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&mut self) -> AVPTPQ_W<4> {
        AVPTPQ_W::new(self)
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W<12> {
        UPQ_W::new(self)
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W<16> {
        MCBCQ_W::new(self)
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W<20> {
        MCBCQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl1](index.html) module"]
pub struct MAC_RXQ_CTRL1_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rxq_ctrl1::R](R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl1::W](W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_RXQ_CTRL1 to value 0"]
impl crate::Resettable for MAC_RXQ_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
