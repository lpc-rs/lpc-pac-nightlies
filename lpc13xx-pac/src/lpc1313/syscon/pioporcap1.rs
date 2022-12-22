#[doc = "Register `PIOPORCAP1` reader"]
pub struct R(crate::R<PIOPORCAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOPORCAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOPORCAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOPORCAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPPIO2_8` reader - Raw reset status input PIO2_8"]
pub type CAPPIO2_8_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO2_9` reader - Raw reset status input PIO2_9"]
pub type CAPPIO2_9_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO2_10` reader - Raw reset status input PIO2_10"]
pub type CAPPIO2_10_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO2_11` reader - Raw reset status input PIO2_11"]
pub type CAPPIO2_11_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_0` reader - Raw reset status input PIO3_0"]
pub type CAPPIO3_0_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_1` reader - Raw reset status input PIO3_1"]
pub type CAPPIO3_1_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_2` reader - Raw reset status input PIO3_2"]
pub type CAPPIO3_2_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_3` reader - Raw reset status input PIO3_3"]
pub type CAPPIO3_3_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_4` reader - Raw reset status input PIO3_4"]
pub type CAPPIO3_4_R = crate::BitReader<bool>;
#[doc = "Field `CAPPIO3_5` reader - Raw reset status input PIO3_5"]
pub type CAPPIO3_5_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Raw reset status input PIO2_8"]
    #[inline(always)]
    pub fn cappio2_8(&self) -> CAPPIO2_8_R {
        CAPPIO2_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw reset status input PIO2_9"]
    #[inline(always)]
    pub fn cappio2_9(&self) -> CAPPIO2_9_R {
        CAPPIO2_9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw reset status input PIO2_10"]
    #[inline(always)]
    pub fn cappio2_10(&self) -> CAPPIO2_10_R {
        CAPPIO2_10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw reset status input PIO2_11"]
    #[inline(always)]
    pub fn cappio2_11(&self) -> CAPPIO2_11_R {
        CAPPIO2_11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw reset status input PIO3_0"]
    #[inline(always)]
    pub fn cappio3_0(&self) -> CAPPIO3_0_R {
        CAPPIO3_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw reset status input PIO3_1"]
    #[inline(always)]
    pub fn cappio3_1(&self) -> CAPPIO3_1_R {
        CAPPIO3_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw reset status input PIO3_2"]
    #[inline(always)]
    pub fn cappio3_2(&self) -> CAPPIO3_2_R {
        CAPPIO3_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw reset status input PIO3_3"]
    #[inline(always)]
    pub fn cappio3_3(&self) -> CAPPIO3_3_R {
        CAPPIO3_3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw reset status input PIO3_4"]
    #[inline(always)]
    pub fn cappio3_4(&self) -> CAPPIO3_4_R {
        CAPPIO3_4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw reset status input PIO3_5"]
    #[inline(always)]
    pub fn cappio3_5(&self) -> CAPPIO3_5_R {
        CAPPIO3_5_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "POR captured PIO status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap1](index.html) module"]
pub struct PIOPORCAP1_SPEC;
impl crate::RegisterSpec for PIOPORCAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pioporcap1::R](R) reader structure"]
impl crate::Readable for PIOPORCAP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIOPORCAP1 to value 0"]
impl crate::Resettable for PIOPORCAP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
