#[doc = "Register `MAC_ADDR_HIGH` reader"]
pub struct R(crate::R<MAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_HIGH` writer"]
pub struct W(crate::W<MAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDR_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A47_32` reader - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
pub type A47_32_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A47_32` writer - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
pub type A47_32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_ADDR_HIGH_SPEC, u16, u16, 16, O>;
#[doc = "Field `DCS` reader - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
pub type DCS_R = crate::BitReader<bool>;
#[doc = "Field `DCS` writer - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
pub type DCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_ADDR_HIGH_SPEC, bool, O>;
#[doc = "Field `AE` reader - Address Enable."]
pub type AE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&self) -> A47_32_R {
        A47_32_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&mut self) -> A47_32_W<0> {
        A47_32_W::new(self)
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W<16> {
        DCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC address0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_high](index.html) module"]
pub struct MAC_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_HIGH to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDR_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
