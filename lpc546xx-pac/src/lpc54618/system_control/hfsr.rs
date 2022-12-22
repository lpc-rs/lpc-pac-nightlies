#[doc = "Register `HFSR` reader"]
pub struct R(crate::R<HFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFSR` writer"]
pub struct W(crate::W<HFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFSR_SPEC>;
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
impl From<crate::W<HFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTTBL` reader - no description available"]
pub type VECTTBL_R = crate::BitReader<VECTTBL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBL_A {
    #[doc = "0: no BusFault on vector table read"]
    VECTTBL_0 = 0,
    #[doc = "1: BusFault on vector table read"]
    VECTTBL_1 = 1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        variant as u8 != 0
    }
}
impl VECTTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VECTTBL_0,
            true => VECTTBL_A::VECTTBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VECTTBL_0`"]
    #[inline(always)]
    pub fn is_vecttbl_0(&self) -> bool {
        *self == VECTTBL_A::VECTTBL_0
    }
    #[doc = "Checks if the value of the field is `VECTTBL_1`"]
    #[inline(always)]
    pub fn is_vecttbl_1(&self) -> bool {
        *self == VECTTBL_A::VECTTBL_1
    }
}
#[doc = "Field `VECTTBL` writer - no description available"]
pub type VECTTBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, VECTTBL_A, O>;
impl<'a, const O: u8> VECTTBL_W<'a, O> {
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_0(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_0)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_1(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_1)
    }
}
#[doc = "Field `FORCED` reader - no description available"]
pub type FORCED_R = crate::BitReader<FORCED_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCED_A {
    #[doc = "0: no forced HardFault"]
    FORCED_0 = 0,
    #[doc = "1: forced HardFault"]
    FORCED_1 = 1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::FORCED_0,
            true => FORCED_A::FORCED_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCED_0`"]
    #[inline(always)]
    pub fn is_forced_0(&self) -> bool {
        *self == FORCED_A::FORCED_0
    }
    #[doc = "Checks if the value of the field is `FORCED_1`"]
    #[inline(always)]
    pub fn is_forced_1(&self) -> bool {
        *self == FORCED_A::FORCED_1
    }
}
#[doc = "Field `FORCED` writer - no description available"]
pub type FORCED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, FORCED_A, O>;
impl<'a, const O: u8> FORCED_W<'a, O> {
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_0)
    }
    #[doc = "forced HardFault"]
    #[inline(always)]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_1)
    }
}
#[doc = "Field `DEBUGEVT` reader - no description available"]
pub type DEBUGEVT_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGEVT` writer - no description available"]
pub type DEBUGEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VECTTBL_W<1> {
        VECTTBL_W::new(self)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W<30> {
        FORCED_W::new(self)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn debugevt(&mut self) -> DEBUGEVT_W<31> {
        DEBUGEVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HardFault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](index.html) module"]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfsr::R](R) reader structure"]
impl crate::Readable for HFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfsr::W](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
