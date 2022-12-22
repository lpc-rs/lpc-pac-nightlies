#[doc = "Register `STARTSRP1` reader"]
pub struct R(crate::R<STARTSRP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTSRP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTSRP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTSRP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRPIO2_8` reader - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO2_8_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO2_9` reader - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO2_9_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO2_10` reader - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO2_10_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO2_11` reader - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO2_11_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO3_0` reader - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO3_0_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO3_1` reader - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO3_1_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO3_2` reader - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO3_2_R = crate::BitReader<bool>;
#[doc = "Field `SRPIO3_3` reader - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
pub type SRPIO3_3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_8(&self) -> SRPIO2_8_R {
        SRPIO2_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_9(&self) -> SRPIO2_9_R {
        SRPIO2_9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_10(&self) -> SRPIO2_10_R {
        SRPIO2_10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_11(&self) -> SRPIO2_11_R {
        SRPIO2_11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_0(&self) -> SRPIO3_0_R {
        SRPIO3_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_1(&self) -> SRPIO3_1_R {
        SRPIO3_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_2(&self) -> SRPIO3_2_R {
        SRPIO3_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_3(&self) -> SRPIO3_3_R {
        SRPIO3_3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Start logic status register 1; top 8 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startsrp1](index.html) module"]
pub struct STARTSRP1_SPEC;
impl crate::RegisterSpec for STARTSRP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startsrp1::R](R) reader structure"]
impl crate::Readable for STARTSRP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STARTSRP1 to value 0"]
impl crate::Resettable for STARTSRP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
