#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE0` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE0_R = crate::BitReader<bool>;
#[doc = "Field `DONE1` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE1_R = crate::BitReader<bool>;
#[doc = "Field `DONE2` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE2_R = crate::BitReader<bool>;
#[doc = "Field `DONE3` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE3_R = crate::BitReader<bool>;
#[doc = "Field `DONE4` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE4_R = crate::BitReader<bool>;
#[doc = "Field `DONE5` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE5_R = crate::BitReader<bool>;
#[doc = "Field `DONE6` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE6_R = crate::BitReader<bool>;
#[doc = "Field `DONE7` reader - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
pub type DONE7_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN0` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN0_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN1` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN1_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN2` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN2_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN3` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN3_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN4` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN4_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN5` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN5_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN6` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN6_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN7` reader - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
pub type OVERRUN7_R = crate::BitReader<bool>;
#[doc = "Field `ADINT` reader - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
pub type ADINT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done2(&self) -> DONE2_R {
        DONE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done3(&self) -> DONE3_R {
        DONE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done4(&self) -> DONE4_R {
        DONE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done5(&self) -> DONE5_R {
        DONE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done6(&self) -> DONE6_R {
        DONE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done7(&self) -> DONE7_R {
        DONE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> ADINT_R {
        ADINT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
