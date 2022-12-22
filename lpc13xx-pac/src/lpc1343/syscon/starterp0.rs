#[doc = "Register `STARTERP0` reader"]
pub struct R(crate::R<STARTERP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP0` writer"]
pub struct W(crate::W<STARTERP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP0_SPEC>;
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
impl From<crate::W<STARTERP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERPIO0_0` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_0_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_0` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_1` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_1_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_1` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_2` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_2_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_2` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_3` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_3_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_3` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_4` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_4_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_4` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_5` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_5_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_5` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_6` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_6_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_6` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_7` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_7_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_7` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_8` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_8_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_8` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_9` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_9_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_9` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_10` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_10_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_10` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO0_11` reader - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_11_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO0_11` writer - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO0_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_0` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_0_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_0` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_1` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_1_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_1` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_2` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_2_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_2` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_3` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_3_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_3` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_4` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_4_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_4` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_5` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_5_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_5` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_6` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_6_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_6` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_7` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_7_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_7` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_8` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_8_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_8` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_9` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_9_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_9` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_10` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_10_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_10` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO1_11` reader - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_11_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO1_11` writer - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO1_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_0` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_0_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_0` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_1` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_1_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_1` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_2` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_2_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_2` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_3` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_3_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_3` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_4` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_4_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_4` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_5` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_5_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_5` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_6` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_6_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_6` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
#[doc = "Field `ERPIO2_7` reader - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_7_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_7` writer - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_0(&self) -> ERPIO0_0_R {
        ERPIO0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_1(&self) -> ERPIO0_1_R {
        ERPIO0_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_2(&self) -> ERPIO0_2_R {
        ERPIO0_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_3(&self) -> ERPIO0_3_R {
        ERPIO0_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_4(&self) -> ERPIO0_4_R {
        ERPIO0_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_5(&self) -> ERPIO0_5_R {
        ERPIO0_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_6(&self) -> ERPIO0_6_R {
        ERPIO0_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_7(&self) -> ERPIO0_7_R {
        ERPIO0_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_8(&self) -> ERPIO0_8_R {
        ERPIO0_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_9(&self) -> ERPIO0_9_R {
        ERPIO0_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_10(&self) -> ERPIO0_10_R {
        ERPIO0_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_11(&self) -> ERPIO0_11_R {
        ERPIO0_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_0(&self) -> ERPIO1_0_R {
        ERPIO1_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_1(&self) -> ERPIO1_1_R {
        ERPIO1_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_2(&self) -> ERPIO1_2_R {
        ERPIO1_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_3(&self) -> ERPIO1_3_R {
        ERPIO1_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_4(&self) -> ERPIO1_4_R {
        ERPIO1_4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_5(&self) -> ERPIO1_5_R {
        ERPIO1_5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_6(&self) -> ERPIO1_6_R {
        ERPIO1_6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_7(&self) -> ERPIO1_7_R {
        ERPIO1_7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_8(&self) -> ERPIO1_8_R {
        ERPIO1_8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_9(&self) -> ERPIO1_9_R {
        ERPIO1_9_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_10(&self) -> ERPIO1_10_R {
        ERPIO1_10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_11(&self) -> ERPIO1_11_R {
        ERPIO1_11_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_0(&self) -> ERPIO2_0_R {
        ERPIO2_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_1(&self) -> ERPIO2_1_R {
        ERPIO2_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_2(&self) -> ERPIO2_2_R {
        ERPIO2_2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_3(&self) -> ERPIO2_3_R {
        ERPIO2_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_4(&self) -> ERPIO2_4_R {
        ERPIO2_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_5(&self) -> ERPIO2_5_R {
        ERPIO2_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_6(&self) -> ERPIO2_6_R {
        ERPIO2_6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_7(&self) -> ERPIO2_7_R {
        ERPIO2_7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_0(&mut self) -> ERPIO0_0_W<0> {
        ERPIO0_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_1(&mut self) -> ERPIO0_1_W<1> {
        ERPIO0_1_W::new(self)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_2(&mut self) -> ERPIO0_2_W<2> {
        ERPIO0_2_W::new(self)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_3(&mut self) -> ERPIO0_3_W<3> {
        ERPIO0_3_W::new(self)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_4(&mut self) -> ERPIO0_4_W<4> {
        ERPIO0_4_W::new(self)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_5(&mut self) -> ERPIO0_5_W<5> {
        ERPIO0_5_W::new(self)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_6(&mut self) -> ERPIO0_6_W<6> {
        ERPIO0_6_W::new(self)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_7(&mut self) -> ERPIO0_7_W<7> {
        ERPIO0_7_W::new(self)
    }
    #[doc = "Bit 8 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_8(&mut self) -> ERPIO0_8_W<8> {
        ERPIO0_8_W::new(self)
    }
    #[doc = "Bit 9 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_9(&mut self) -> ERPIO0_9_W<9> {
        ERPIO0_9_W::new(self)
    }
    #[doc = "Bit 10 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_10(&mut self) -> ERPIO0_10_W<10> {
        ERPIO0_10_W::new(self)
    }
    #[doc = "Bit 11 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_11(&mut self) -> ERPIO0_11_W<11> {
        ERPIO0_11_W::new(self)
    }
    #[doc = "Bit 12 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_0(&mut self) -> ERPIO1_0_W<12> {
        ERPIO1_0_W::new(self)
    }
    #[doc = "Bit 13 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_1(&mut self) -> ERPIO1_1_W<13> {
        ERPIO1_1_W::new(self)
    }
    #[doc = "Bit 14 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_2(&mut self) -> ERPIO1_2_W<14> {
        ERPIO1_2_W::new(self)
    }
    #[doc = "Bit 15 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_3(&mut self) -> ERPIO1_3_W<15> {
        ERPIO1_3_W::new(self)
    }
    #[doc = "Bit 16 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_4(&mut self) -> ERPIO1_4_W<16> {
        ERPIO1_4_W::new(self)
    }
    #[doc = "Bit 17 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_5(&mut self) -> ERPIO1_5_W<17> {
        ERPIO1_5_W::new(self)
    }
    #[doc = "Bit 18 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_6(&mut self) -> ERPIO1_6_W<18> {
        ERPIO1_6_W::new(self)
    }
    #[doc = "Bit 19 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_7(&mut self) -> ERPIO1_7_W<19> {
        ERPIO1_7_W::new(self)
    }
    #[doc = "Bit 20 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_8(&mut self) -> ERPIO1_8_W<20> {
        ERPIO1_8_W::new(self)
    }
    #[doc = "Bit 21 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_9(&mut self) -> ERPIO1_9_W<21> {
        ERPIO1_9_W::new(self)
    }
    #[doc = "Bit 22 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_10(&mut self) -> ERPIO1_10_W<22> {
        ERPIO1_10_W::new(self)
    }
    #[doc = "Bit 23 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_11(&mut self) -> ERPIO1_11_W<23> {
        ERPIO1_11_W::new(self)
    }
    #[doc = "Bit 24 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_0(&mut self) -> ERPIO2_0_W<24> {
        ERPIO2_0_W::new(self)
    }
    #[doc = "Bit 25 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_1(&mut self) -> ERPIO2_1_W<25> {
        ERPIO2_1_W::new(self)
    }
    #[doc = "Bit 26 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_2(&mut self) -> ERPIO2_2_W<26> {
        ERPIO2_2_W::new(self)
    }
    #[doc = "Bit 27 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_3(&mut self) -> ERPIO2_3_W<27> {
        ERPIO2_3_W::new(self)
    }
    #[doc = "Bit 28 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_4(&mut self) -> ERPIO2_4_W<28> {
        ERPIO2_4_W::new(self)
    }
    #[doc = "Bit 29 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_5(&mut self) -> ERPIO2_5_W<29> {
        ERPIO2_5_W::new(self)
    }
    #[doc = "Bit 30 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_6(&mut self) -> ERPIO2_6_W<30> {
        ERPIO2_6_W::new(self)
    }
    #[doc = "Bit 31 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_7(&mut self) -> ERPIO2_7_W<31> {
        ERPIO2_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic signal enable register 0; bottom 32 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp0](index.html) module"]
pub struct STARTERP0_SPEC;
impl crate::RegisterSpec for STARTERP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp0::R](R) reader structure"]
impl crate::Readable for STARTERP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp0::W](W) writer structure"]
impl crate::Writable for STARTERP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERP0 to value 0"]
impl crate::Resettable for STARTERP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
