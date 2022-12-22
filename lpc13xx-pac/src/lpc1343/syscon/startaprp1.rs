#[doc = "Register `STARTAPRP1` reader"]
pub struct R(crate::R<STARTAPRP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTAPRP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTAPRP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTAPRP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTAPRP1` writer"]
pub struct W(crate::W<STARTAPRP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTAPRP1_SPEC>;
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
impl From<crate::W<STARTAPRP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTAPRP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APRPIO2_8` reader - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_8_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_8` writer - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO2_9` reader - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_9_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_9` writer - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO2_10` reader - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_10_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_10` writer - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO2_11` reader - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_11_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO2_11` writer - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO2_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO3_0` reader - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_0_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO3_0` writer - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO3_1` reader - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_1_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO3_1` writer - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO3_2` reader - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_2_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO3_2` writer - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
#[doc = "Field `APRPIO3_3` reader - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_3_R = crate::BitReader<bool>;
#[doc = "Field `APRPIO3_3` writer - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
pub type APRPIO3_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTAPRP1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_8(&self) -> APRPIO2_8_R {
        APRPIO2_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_9(&self) -> APRPIO2_9_R {
        APRPIO2_9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_10(&self) -> APRPIO2_10_R {
        APRPIO2_10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_11(&self) -> APRPIO2_11_R {
        APRPIO2_11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_0(&self) -> APRPIO3_0_R {
        APRPIO3_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_1(&self) -> APRPIO3_1_R {
        APRPIO3_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_2(&self) -> APRPIO3_2_R {
        APRPIO3_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_3(&self) -> APRPIO3_3_R {
        APRPIO3_3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_8(&mut self) -> APRPIO2_8_W<0> {
        APRPIO2_8_W::new(self)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_9(&mut self) -> APRPIO2_9_W<1> {
        APRPIO2_9_W::new(self)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_10(&mut self) -> APRPIO2_10_W<2> {
        APRPIO2_10_W::new(self)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_11(&mut self) -> APRPIO2_11_W<3> {
        APRPIO2_11_W::new(self)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_0(&mut self) -> APRPIO3_0_W<4> {
        APRPIO3_0_W::new(self)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_1(&mut self) -> APRPIO3_1_W<5> {
        APRPIO3_1_W::new(self)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_2(&mut self) -> APRPIO3_2_W<6> {
        APRPIO3_2_W::new(self)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_3(&mut self) -> APRPIO3_3_W<7> {
        APRPIO3_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic edge control register 1; top 8 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startaprp1](index.html) module"]
pub struct STARTAPRP1_SPEC;
impl crate::RegisterSpec for STARTAPRP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startaprp1::R](R) reader structure"]
impl crate::Readable for STARTAPRP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startaprp1::W](W) writer structure"]
impl crate::Writable for STARTAPRP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTAPRP1 to value 0"]
impl crate::Resettable for STARTAPRP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
