#[doc = "Register `RXPLEN` reader"]
pub struct R(crate::R<RXPLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKT_LNGTH` reader - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
pub type PKT_LNGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DV` reader - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
pub type DV_R = crate::BitReader<DV_A>;
#[doc = "Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DV_A {
    #[doc = "0: Data is invalid."]
    DATA_IS_INVALID_ = 0,
    #[doc = "1: Data is valid."]
    DATA_IS_VALID_ = 1,
}
impl From<DV_A> for bool {
    #[inline(always)]
    fn from(variant: DV_A) -> Self {
        variant as u8 != 0
    }
}
impl DV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV_A {
        match self.bits {
            false => DV_A::DATA_IS_INVALID_,
            true => DV_A::DATA_IS_VALID_,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_IS_INVALID_`"]
    #[inline(always)]
    pub fn is_data_is_invalid_(&self) -> bool {
        *self == DV_A::DATA_IS_INVALID_
    }
    #[doc = "Checks if the value of the field is `DATA_IS_VALID_`"]
    #[inline(always)]
    pub fn is_data_is_valid_(&self) -> bool {
        *self == DV_A::DATA_IS_VALID_
    }
}
impl R {
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&self) -> PKT_LNGTH_R {
        PKT_LNGTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "USB Receive Packet Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxplen](index.html) module"]
pub struct RXPLEN_SPEC;
impl crate::RegisterSpec for RXPLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxplen::R](R) reader structure"]
impl crate::Readable for RXPLEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXPLEN to value 0"]
impl crate::Resettable for RXPLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
