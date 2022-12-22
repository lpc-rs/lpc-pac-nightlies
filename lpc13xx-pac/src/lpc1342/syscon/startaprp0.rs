#[doc = "Register `STARTAPRP0` reader"]
pub struct R(crate::R<STARTAPRP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTAPRP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTAPRP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTAPRP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTAPRP0` writer"]
pub struct W(crate::W<STARTAPRP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTAPRP0_SPEC>;
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
impl From<crate::W<STARTAPRP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTAPRP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APRPIO0_0` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_0_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_0` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_1` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_1_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_1` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_2` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_2_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_2` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_3` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_3_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_3` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_4` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_4_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_4` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_5` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_5_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_5` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_6` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_6_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_6` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_7` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_7_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_7` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_8` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_8_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_8` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_9` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_9_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_9` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_10` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_10_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_10` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO0_11` reader - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_11_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO0_11` writer - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO0_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_0` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_0_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_0` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_1` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_1_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_1` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_2` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_2_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_2` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_3` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_3_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_3` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_4` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_4_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_4` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_5` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_5_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_5` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_6` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_6_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_6` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_7` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_7_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_7` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_8` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_8_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_8` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_9` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_9_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_9` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_10` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_10_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_10` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO1_11` reader - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_11_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO1_11` writer - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO1_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_0` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_0_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_0` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_1` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_1_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_1` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_2` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_2_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_2` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_3` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_3_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_3` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_4` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_4_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_4` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_5` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_5_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_5` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_6` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_6_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_6` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
#[doc = "Field `APRPIO2_7` reader - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_7_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_7` writer - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&self) -> APRPIO0_0_R {
        APRPIO0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&self) -> APRPIO0_1_R {
        APRPIO0_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&self) -> APRPIO0_2_R {
        APRPIO0_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&self) -> APRPIO0_3_R {
        APRPIO0_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&self) -> APRPIO0_4_R {
        APRPIO0_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&self) -> APRPIO0_5_R {
        APRPIO0_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&self) -> APRPIO0_6_R {
        APRPIO0_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&self) -> APRPIO0_7_R {
        APRPIO0_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&self) -> APRPIO0_8_R {
        APRPIO0_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&self) -> APRPIO0_9_R {
        APRPIO0_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&self) -> APRPIO0_10_R {
        APRPIO0_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&self) -> APRPIO0_11_R {
        APRPIO0_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&self) -> APRPIO1_0_R {
        APRPIO1_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_1(&self) -> APRPIO1_1_R {
        APRPIO1_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_2(&self) -> APRPIO1_2_R {
        APRPIO1_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_3(&self) -> APRPIO1_3_R {
        APRPIO1_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_4(&self) -> APRPIO1_4_R {
        APRPIO1_4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_5(&self) -> APRPIO1_5_R {
        APRPIO1_5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_6(&self) -> APRPIO1_6_R {
        APRPIO1_6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_7(&self) -> APRPIO1_7_R {
        APRPIO1_7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_8(&self) -> APRPIO1_8_R {
        APRPIO1_8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_9(&self) -> APRPIO1_9_R {
        APRPIO1_9_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_10(&self) -> APRPIO1_10_R {
        APRPIO1_10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_11(&self) -> APRPIO1_11_R {
        APRPIO1_11_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_0(&self) -> APRPIO2_0_R {
        APRPIO2_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_1(&self) -> APRPIO2_1_R {
        APRPIO2_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_2(&self) -> APRPIO2_2_R {
        APRPIO2_2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_3(&self) -> APRPIO2_3_R {
        APRPIO2_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_4(&self) -> APRPIO2_4_R {
        APRPIO2_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_5(&self) -> APRPIO2_5_R {
        APRPIO2_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_6(&self) -> APRPIO2_6_R {
        APRPIO2_6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_7(&self) -> APRPIO2_7_R {
        APRPIO2_7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&mut self) -> APRPIO0_0_W<0> {
        APRPIO0_0_W::new(self)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&mut self) -> APRPIO0_1_W<1> {
        APRPIO0_1_W::new(self)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&mut self) -> APRPIO0_2_W<2> {
        APRPIO0_2_W::new(self)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&mut self) -> APRPIO0_3_W<3> {
        APRPIO0_3_W::new(self)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&mut self) -> APRPIO0_4_W<4> {
        APRPIO0_4_W::new(self)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&mut self) -> APRPIO0_5_W<5> {
        APRPIO0_5_W::new(self)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&mut self) -> APRPIO0_6_W<6> {
        APRPIO0_6_W::new(self)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&mut self) -> APRPIO0_7_W<7> {
        APRPIO0_7_W::new(self)
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&mut self) -> APRPIO0_8_W<8> {
        APRPIO0_8_W::new(self)
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&mut self) -> APRPIO0_9_W<9> {
        APRPIO0_9_W::new(self)
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&mut self) -> APRPIO0_10_W<10> {
        APRPIO0_10_W::new(self)
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&mut self) -> APRPIO0_11_W<11> {
        APRPIO0_11_W::new(self)
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&mut self) -> APRPIO1_0_W<12> {
        APRPIO1_0_W::new(self)
    }
    #[doc = "Bit 13 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_1(&mut self) -> APRPIO1_1_W<13> {
        APRPIO1_1_W::new(self)
    }
    #[doc = "Bit 14 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_2(&mut self) -> APRPIO1_2_W<14> {
        APRPIO1_2_W::new(self)
    }
    #[doc = "Bit 15 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_3(&mut self) -> APRPIO1_3_W<15> {
        APRPIO1_3_W::new(self)
    }
    #[doc = "Bit 16 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_4(&mut self) -> APRPIO1_4_W<16> {
        APRPIO1_4_W::new(self)
    }
    #[doc = "Bit 17 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_5(&mut self) -> APRPIO1_5_W<17> {
        APRPIO1_5_W::new(self)
    }
    #[doc = "Bit 18 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_6(&mut self) -> APRPIO1_6_W<18> {
        APRPIO1_6_W::new(self)
    }
    #[doc = "Bit 19 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_7(&mut self) -> APRPIO1_7_W<19> {
        APRPIO1_7_W::new(self)
    }
    #[doc = "Bit 20 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_8(&mut self) -> APRPIO1_8_W<20> {
        APRPIO1_8_W::new(self)
    }
    #[doc = "Bit 21 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_9(&mut self) -> APRPIO1_9_W<21> {
        APRPIO1_9_W::new(self)
    }
    #[doc = "Bit 22 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_10(&mut self) -> APRPIO1_10_W<22> {
        APRPIO1_10_W::new(self)
    }
    #[doc = "Bit 23 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_11(&mut self) -> APRPIO1_11_W<23> {
        APRPIO1_11_W::new(self)
    }
    #[doc = "Bit 24 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_0(&mut self) -> APRPIO2_0_W<24> {
        APRPIO2_0_W::new(self)
    }
    #[doc = "Bit 25 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_1(&mut self) -> APRPIO2_1_W<25> {
        APRPIO2_1_W::new(self)
    }
    #[doc = "Bit 26 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_2(&mut self) -> APRPIO2_2_W<26> {
        APRPIO2_2_W::new(self)
    }
    #[doc = "Bit 27 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_3(&mut self) -> APRPIO2_3_W<27> {
        APRPIO2_3_W::new(self)
    }
    #[doc = "Bit 28 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_4(&mut self) -> APRPIO2_4_W<28> {
        APRPIO2_4_W::new(self)
    }
    #[doc = "Bit 29 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_5(&mut self) -> APRPIO2_5_W<29> {
        APRPIO2_5_W::new(self)
    }
    #[doc = "Bit 30 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_6(&mut self) -> APRPIO2_6_W<30> {
        APRPIO2_6_W::new(self)
    }
    #[doc = "Bit 31 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_7(&mut self) -> APRPIO2_7_W<31> {
        APRPIO2_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic edge control register 0; bottom 32 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startaprp0](index.html) module"]
pub struct STARTAPRP0_SPEC;
impl crate::RegisterSpec for STARTAPRP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startaprp0::R](R) reader structure"]
impl crate::Readable for STARTAPRP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startaprp0::W](W) writer structure"]
impl crate::Writable for STARTAPRP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTAPRP0 to value 0"]
impl crate::Resettable for STARTAPRP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
