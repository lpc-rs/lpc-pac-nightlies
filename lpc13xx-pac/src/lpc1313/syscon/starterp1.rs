#[doc = "Register `STARTERP1` reader"]
pub struct R(crate::R<STARTERP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP1` writer"]
pub struct W(crate::W<STARTERP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP1_SPEC>;
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
impl From<crate::W<STARTERP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERPIO2_8` reader - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_8_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_8` writer - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO2_9` reader - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_9_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_9` writer - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO2_10` reader - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_10_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_10` writer - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO2_11` reader - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_11_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO2_11` writer - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
pub type ERPIO2_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO3_0` reader - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_0_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO3_0` writer - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO3_1` reader - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_1_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO3_1` writer - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO3_2` reader - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_2_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO3_2` writer - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
#[doc = "Field `ERPIO3_3` reader - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_3_R = crate::BitReader<bool>;
#[doc = "Field `ERPIO3_3` writer - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
pub type ERPIO3_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_8(&self) -> ERPIO2_8_R {
        ERPIO2_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_9(&self) -> ERPIO2_9_R {
        ERPIO2_9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_10(&self) -> ERPIO2_10_R {
        ERPIO2_10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_11(&self) -> ERPIO2_11_R {
        ERPIO2_11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_0(&self) -> ERPIO3_0_R {
        ERPIO3_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_1(&self) -> ERPIO3_1_R {
        ERPIO3_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_2(&self) -> ERPIO3_2_R {
        ERPIO3_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_3(&self) -> ERPIO3_3_R {
        ERPIO3_3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_8(&mut self) -> ERPIO2_8_W<0> {
        ERPIO2_8_W::new(self)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_9(&mut self) -> ERPIO2_9_W<1> {
        ERPIO2_9_W::new(self)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_10(&mut self) -> ERPIO2_10_W<2> {
        ERPIO2_10_W::new(self)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_11(&mut self) -> ERPIO2_11_W<3> {
        ERPIO2_11_W::new(self)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_0(&mut self) -> ERPIO3_0_W<4> {
        ERPIO3_0_W::new(self)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_1(&mut self) -> ERPIO3_1_W<5> {
        ERPIO3_1_W::new(self)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_2(&mut self) -> ERPIO3_2_W<6> {
        ERPIO3_2_W::new(self)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_3(&mut self) -> ERPIO3_3_W<7> {
        ERPIO3_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic signal enable register 1; top 8 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](index.html) module"]
pub struct STARTERP1_SPEC;
impl crate::RegisterSpec for STARTERP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp1::R](R) reader structure"]
impl crate::Readable for STARTERP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp1::W](W) writer structure"]
impl crate::Writable for STARTERP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERP1 to value 0"]
impl crate::Resettable for STARTERP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
