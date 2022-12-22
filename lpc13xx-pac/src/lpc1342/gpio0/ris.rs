#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAWST0` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST0_R = crate::BitReader<bool>;
#[doc = "Field `RAWST1` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST1_R = crate::BitReader<bool>;
#[doc = "Field `RAWST2` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST2_R = crate::BitReader<bool>;
#[doc = "Field `RAWST3` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST3_R = crate::BitReader<bool>;
#[doc = "Field `RAWST4` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST4_R = crate::BitReader<bool>;
#[doc = "Field `RAWST5` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST5_R = crate::BitReader<bool>;
#[doc = "Field `RAWST6` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST6_R = crate::BitReader<bool>;
#[doc = "Field `RAWST7` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST7_R = crate::BitReader<bool>;
#[doc = "Field `RAWST8` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST8_R = crate::BitReader<bool>;
#[doc = "Field `RAWST9` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST9_R = crate::BitReader<bool>;
#[doc = "Field `RAWST10` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST10_R = crate::BitReader<bool>;
#[doc = "Field `RAWST11` reader - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
pub type RAWST11_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst0(&self) -> RAWST0_R {
        RAWST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst1(&self) -> RAWST1_R {
        RAWST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst2(&self) -> RAWST2_R {
        RAWST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst3(&self) -> RAWST3_R {
        RAWST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst4(&self) -> RAWST4_R {
        RAWST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst5(&self) -> RAWST5_R {
        RAWST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst6(&self) -> RAWST6_R {
        RAWST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst7(&self) -> RAWST7_R {
        RAWST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst8(&self) -> RAWST8_R {
        RAWST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst9(&self) -> RAWST9_R {
        RAWST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst10(&self) -> RAWST10_R {
        RAWST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst11(&self) -> RAWST11_R {
        RAWST11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status register for port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
