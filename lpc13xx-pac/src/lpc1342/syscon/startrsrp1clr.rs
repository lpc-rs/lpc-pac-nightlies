#[doc = "Register `STARTRSRP1CLR` writer"]
pub struct W(crate::W<STARTRSRP1CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTRSRP1CLR_SPEC>;
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
impl From<crate::W<STARTRSRP1CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTRSRP1CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSRPIO2_8` writer - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO2_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_9` writer - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO2_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_10` writer - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO2_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO2_11` writer - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO2_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO3_0` writer - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO3_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO3_1` writer - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO3_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO3_2` writer - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO3_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
#[doc = "Field `RSRPIO3_3` writer - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
pub type RSRPIO3_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTRSRP1CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_8(&mut self) -> RSRPIO2_8_W<0> {
        RSRPIO2_8_W::new(self)
    }
    #[doc = "Bit 1 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_9(&mut self) -> RSRPIO2_9_W<1> {
        RSRPIO2_9_W::new(self)
    }
    #[doc = "Bit 2 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_10(&mut self) -> RSRPIO2_10_W<2> {
        RSRPIO2_10_W::new(self)
    }
    #[doc = "Bit 3 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_11(&mut self) -> RSRPIO2_11_W<3> {
        RSRPIO2_11_W::new(self)
    }
    #[doc = "Bit 4 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_0(&mut self) -> RSRPIO3_0_W<4> {
        RSRPIO3_0_W::new(self)
    }
    #[doc = "Bit 5 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_1(&mut self) -> RSRPIO3_1_W<5> {
        RSRPIO3_1_W::new(self)
    }
    #[doc = "Bit 6 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_2(&mut self) -> RSRPIO3_2_W<6> {
        RSRPIO3_2_W::new(self)
    }
    #[doc = "Bit 7 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_3(&mut self) -> RSRPIO3_3_W<7> {
        RSRPIO3_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic reset register 1; top 8 interrupts\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startrsrp1clr](index.html) module"]
pub struct STARTRSRP1CLR_SPEC;
impl crate::RegisterSpec for STARTRSRP1CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [startrsrp1clr::W](W) writer structure"]
impl crate::Writable for STARTRSRP1CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTRSRP1CLR to value 0"]
impl crate::Resettable for STARTRSRP1CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
