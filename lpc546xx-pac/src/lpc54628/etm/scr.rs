#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MaximumPortSize` reader - Maximum ETM port size bits \\[2:0\\]. These bits are used in conjunction with bit \\[9\\]. The value of these bits is b001."]
pub type MAXIMUM_PORT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOFULLsupported` reader - FIFOFULL supported. The value of this bit is 1, indicating that FIFOFULL is supported. This bit is used in conjunction with bit \\[23\\]
of the ETMCCR."]
pub type FIFOFULLSUPPORTED_R = crate::BitReader<bool>;
#[doc = "Field `MaximumPortSize3` reader - Maximum ETM port size bit \\[3\\]. This bit is used in conjunction with bits \\[2:0\\]. Its value is 0. This has no effect on the TPIU trace port."]
pub type MAXIMUM_PORT_SIZE3_R = crate::BitReader<bool>;
#[doc = "Field `PortSizeSupported` reader - Port size supported. This bit reads as 1 if the currently selected port size is supported. This has no effect on the TPIU trace port."]
pub type PORT_SIZE_SUPPORTED_R = crate::BitReader<bool>;
#[doc = "Field `PortModeSupported` reader - Port mode supported. This bit reads as 1 if the currently selected port mode is supported. This has no effect on the TPIU trace port."]
pub type PORT_MODE_SUPPORTED_R = crate::BitReader<bool>;
#[doc = "Field `N` reader - These bits give the number of supported processors minus 1. The value of these bits is b000, indicating that there is only one processor connected."]
pub type N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NoFetchComparisons` reader - No Fetch comparisons. The value of this bit is 1, indicating that fetch comparisons are not implemented."]
pub type NO_FETCH_COMPARISONS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Maximum ETM port size bits \\[2:0\\]. These bits are used in conjunction with bit \\[9\\]. The value of these bits is b001."]
    #[inline(always)]
    pub fn maximum_port_size(&self) -> MAXIMUM_PORT_SIZE_R {
        MAXIMUM_PORT_SIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - FIFOFULL supported. The value of this bit is 1, indicating that FIFOFULL is supported. This bit is used in conjunction with bit \\[23\\]
of the ETMCCR."]
    #[inline(always)]
    pub fn fifofullsupported(&self) -> FIFOFULLSUPPORTED_R {
        FIFOFULLSUPPORTED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Maximum ETM port size bit \\[3\\]. This bit is used in conjunction with bits \\[2:0\\]. Its value is 0. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn maximum_port_size3(&self) -> MAXIMUM_PORT_SIZE3_R {
        MAXIMUM_PORT_SIZE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port size supported. This bit reads as 1 if the currently selected port size is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_size_supported(&self) -> PORT_SIZE_SUPPORTED_R {
        PORT_SIZE_SUPPORTED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port mode supported. This bit reads as 1 if the currently selected port mode is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_mode_supported(&self) -> PORT_MODE_SUPPORTED_R {
        PORT_MODE_SUPPORTED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - These bits give the number of supported processors minus 1. The value of these bits is b000, indicating that there is only one processor connected."]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - No Fetch comparisons. The value of this bit is 1, indicating that fetch comparisons are not implemented."]
    #[inline(always)]
    pub fn no_fetch_comparisons(&self) -> NO_FETCH_COMPARISONS_R {
        NO_FETCH_COMPARISONS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR to value 0x0002_0d09"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0d09
    }
}
