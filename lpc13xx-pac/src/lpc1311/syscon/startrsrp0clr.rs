#[doc = "Register `STARTRSRP0CLR` writer"]
pub struct W(crate::W<STARTRSRP0CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTRSRP0CLR_SPEC>;
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
impl From<crate::W<STARTRSRP0CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTRSRP0CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSRPIO0_0` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_1` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_2` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_3` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_4` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_5` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_6` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_7` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_8` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_9` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_10` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO0_11` writer - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO0_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_0` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_1` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_2` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_3` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_4` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_5` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_6` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_7` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_8` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_9` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_10` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO1_11` writer - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO1_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_0` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_1` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_2` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_3` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_4` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_5` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_6` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_7` writer - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
pub type RSRPIO2_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP0CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_0(&mut self) -> RSRPIO0_0_W<0> {
        RSRPIO0_0_W::new(self)
    }
    #[doc = "Bit 1 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_1(&mut self) -> RSRPIO0_1_W<1> {
        RSRPIO0_1_W::new(self)
    }
    #[doc = "Bit 2 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_2(&mut self) -> RSRPIO0_2_W<2> {
        RSRPIO0_2_W::new(self)
    }
    #[doc = "Bit 3 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_3(&mut self) -> RSRPIO0_3_W<3> {
        RSRPIO0_3_W::new(self)
    }
    #[doc = "Bit 4 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_4(&mut self) -> RSRPIO0_4_W<4> {
        RSRPIO0_4_W::new(self)
    }
    #[doc = "Bit 5 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_5(&mut self) -> RSRPIO0_5_W<5> {
        RSRPIO0_5_W::new(self)
    }
    #[doc = "Bit 6 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_6(&mut self) -> RSRPIO0_6_W<6> {
        RSRPIO0_6_W::new(self)
    }
    #[doc = "Bit 7 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_7(&mut self) -> RSRPIO0_7_W<7> {
        RSRPIO0_7_W::new(self)
    }
    #[doc = "Bit 8 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_8(&mut self) -> RSRPIO0_8_W<8> {
        RSRPIO0_8_W::new(self)
    }
    #[doc = "Bit 9 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_9(&mut self) -> RSRPIO0_9_W<9> {
        RSRPIO0_9_W::new(self)
    }
    #[doc = "Bit 10 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_10(&mut self) -> RSRPIO0_10_W<10> {
        RSRPIO0_10_W::new(self)
    }
    #[doc = "Bit 11 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_11(&mut self) -> RSRPIO0_11_W<11> {
        RSRPIO0_11_W::new(self)
    }
    #[doc = "Bit 12 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_0(&mut self) -> RSRPIO1_0_W<12> {
        RSRPIO1_0_W::new(self)
    }
    #[doc = "Bit 13 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_1(&mut self) -> RSRPIO1_1_W<13> {
        RSRPIO1_1_W::new(self)
    }
    #[doc = "Bit 14 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_2(&mut self) -> RSRPIO1_2_W<14> {
        RSRPIO1_2_W::new(self)
    }
    #[doc = "Bit 15 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_3(&mut self) -> RSRPIO1_3_W<15> {
        RSRPIO1_3_W::new(self)
    }
    #[doc = "Bit 16 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_4(&mut self) -> RSRPIO1_4_W<16> {
        RSRPIO1_4_W::new(self)
    }
    #[doc = "Bit 17 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_5(&mut self) -> RSRPIO1_5_W<17> {
        RSRPIO1_5_W::new(self)
    }
    #[doc = "Bit 18 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_6(&mut self) -> RSRPIO1_6_W<18> {
        RSRPIO1_6_W::new(self)
    }
    #[doc = "Bit 19 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_7(&mut self) -> RSRPIO1_7_W<19> {
        RSRPIO1_7_W::new(self)
    }
    #[doc = "Bit 20 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_8(&mut self) -> RSRPIO1_8_W<20> {
        RSRPIO1_8_W::new(self)
    }
    #[doc = "Bit 21 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_9(&mut self) -> RSRPIO1_9_W<21> {
        RSRPIO1_9_W::new(self)
    }
    #[doc = "Bit 22 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_10(&mut self) -> RSRPIO1_10_W<22> {
        RSRPIO1_10_W::new(self)
    }
    #[doc = "Bit 23 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_11(&mut self) -> RSRPIO1_11_W<23> {
        RSRPIO1_11_W::new(self)
    }
    #[doc = "Bit 24 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_0(&mut self) -> RSRPIO2_0_W<24> {
        RSRPIO2_0_W::new(self)
    }
    #[doc = "Bit 25 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_1(&mut self) -> RSRPIO2_1_W<25> {
        RSRPIO2_1_W::new(self)
    }
    #[doc = "Bit 26 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_2(&mut self) -> RSRPIO2_2_W<26> {
        RSRPIO2_2_W::new(self)
    }
    #[doc = "Bit 27 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_3(&mut self) -> RSRPIO2_3_W<27> {
        RSRPIO2_3_W::new(self)
    }
    #[doc = "Bit 28 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_4(&mut self) -> RSRPIO2_4_W<28> {
        RSRPIO2_4_W::new(self)
    }
    #[doc = "Bit 29 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_5(&mut self) -> RSRPIO2_5_W<29> {
        RSRPIO2_5_W::new(self)
    }
    #[doc = "Bit 30 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_6(&mut self) -> RSRPIO2_6_W<30> {
        RSRPIO2_6_W::new(self)
    }
    #[doc = "Bit 31 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_7(&mut self) -> RSRPIO2_7_W<31> {
        RSRPIO2_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic reset register 0; bottom 32 interrupts\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startrsrp0clr](index.html) module"]
pub struct STARTRSRP0CLR_SPEC;
impl crate::RegisterSpec for STARTRSRP0CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [startrsrp0clr::W](W) writer structure"]
impl crate::Writable for STARTRSRP0CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTRSRP0CLR to value 0"]
impl crate::Resettable for STARTRSRP0CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
