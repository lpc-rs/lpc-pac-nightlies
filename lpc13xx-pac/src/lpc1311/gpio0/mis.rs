#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASK0` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK0_R = crate::BitReader<bool>;
#[doc = "Field `MASK1` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK1_R = crate::BitReader<bool>;
#[doc = "Field `MASK2` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK2_R = crate::BitReader<bool>;
#[doc = "Field `MASK3` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK3_R = crate::BitReader<bool>;
#[doc = "Field `MASK4` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK4_R = crate::BitReader<bool>;
#[doc = "Field `MASK5` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK5_R = crate::BitReader<bool>;
#[doc = "Field `MASK6` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK6_R = crate::BitReader<bool>;
#[doc = "Field `MASK7` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK7_R = crate::BitReader<bool>;
#[doc = "Field `MASK8` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK8_R = crate::BitReader<bool>;
#[doc = "Field `MASK9` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK9_R = crate::BitReader<bool>;
#[doc = "Field `MASK10` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK10_R = crate::BitReader<bool>;
#[doc = "Field `MASK11` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
pub type MASK11_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask8(&self) -> MASK8_R {
        MASK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask9(&self) -> MASK9_R {
        MASK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask10(&self) -> MASK10_R {
        MASK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask11(&self) -> MASK11_R {
        MASK11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Masked interrupt status register for port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
