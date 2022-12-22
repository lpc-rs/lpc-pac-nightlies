#[doc = "Register `BODCTRL` reader"]
pub struct R(crate::R<BODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCTRL` writer"]
pub struct W(crate::W<BODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCTRL_SPEC>;
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
impl From<crate::W<BODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODRSTLEV` reader - BOD reset level"]
pub type BODRSTLEV_R = crate::FieldReader<u8, BODRSTLEV_A>;
#[doc = "BOD reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: Level 0: 1.5 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 1.85 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.0 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 2.3 V"]
    LEVEL3 = 3,
}
impl From<BODRSTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODRSTLEV_A) -> Self {
        variant as _
    }
}
impl BODRSTLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTLEV_A {
        match self.bits {
            0 => BODRSTLEV_A::LEVEL0,
            1 => BODRSTLEV_A::LEVEL1,
            2 => BODRSTLEV_A::LEVEL2,
            3 => BODRSTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL3
    }
}
#[doc = "Field `BODRSTLEV` writer - BOD reset level"]
pub type BODRSTLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BODCTRL_SPEC, u8, BODRSTLEV_A, 2, O>;
impl<'a, const O: u8> BODRSTLEV_W<'a, O> {
    #[doc = "Level 0: 1.5 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 1.85 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.0 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 2.3 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL3)
    }
}
#[doc = "Field `BODRSTENA` reader - BOD reset enable"]
pub type BODRSTENA_R = crate::BitReader<BODRSTENA_A>;
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE = 0,
    #[doc = "1: Enable reset function."]
    ENABLE = 1,
}
impl From<BODRSTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl BODRSTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENA_A {
        match self.bits {
            false => BODRSTENA_A::DISABLE,
            true => BODRSTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODRSTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENA_A::ENABLE
    }
}
#[doc = "Field `BODRSTENA` writer - BOD reset enable"]
pub type BODRSTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, BODRSTENA_A, O>;
impl<'a, const O: u8> BODRSTENA_W<'a, O> {
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE)
    }
}
#[doc = "Field `BODINTLEV` reader - BOD interrupt level"]
pub type BODINTLEV_R = crate::FieldReader<u8, BODINTLEV_A>;
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTLEV_A {
    #[doc = "0: Level 0: 2.05 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 2.45 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.75 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 3.05 V"]
    LEVEL3 = 3,
}
impl From<BODINTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTLEV_A) -> Self {
        variant as _
    }
}
impl BODINTLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTLEV_A {
        match self.bits {
            0 => BODINTLEV_A::LEVEL0,
            1 => BODINTLEV_A::LEVEL1,
            2 => BODINTLEV_A::LEVEL2,
            3 => BODINTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODINTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODINTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODINTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODINTLEV_A::LEVEL3
    }
}
#[doc = "Field `BODINTLEV` writer - BOD interrupt level"]
pub type BODINTLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BODCTRL_SPEC, u8, BODINTLEV_A, 2, O>;
impl<'a, const O: u8> BODINTLEV_W<'a, O> {
    #[doc = "Level 0: 2.05 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 2.45 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.75 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 3.05 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL3)
    }
}
#[doc = "Field `BODINTENA` reader - BOD interrupt enable"]
pub type BODINTENA_R = crate::BitReader<BODINTENA_A>;
#[doc = "BOD interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTENA_A {
    #[doc = "0: Disable interrupt function."]
    DISABLE = 0,
    #[doc = "1: Enable interrupt function."]
    ENABLE = 1,
}
impl From<BODINTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODINTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl BODINTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTENA_A {
        match self.bits {
            false => BODINTENA_A::DISABLE,
            true => BODINTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODINTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODINTENA_A::ENABLE
    }
}
#[doc = "Field `BODINTENA` writer - BOD interrupt enable"]
pub type BODINTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, BODINTENA_A, O>;
impl<'a, const O: u8> BODINTENA_W<'a, O> {
    #[doc = "Disable interrupt function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODINTENA_A::DISABLE)
    }
    #[doc = "Enable interrupt function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODINTENA_A::ENABLE)
    }
}
#[doc = "Field `BODRSTSTAT` reader - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
pub type BODRSTSTAT_R = crate::BitReader<bool>;
#[doc = "Field `BODRSTSTAT` writer - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
pub type BODRSTSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, bool, O>;
#[doc = "Field `BODINTSTAT` reader - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
pub type BODINTSTAT_R = crate::BitReader<bool>;
#[doc = "Field `BODINTSTAT` writer - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
pub type BODINTSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&self) -> BODINTLEV_R {
        BODINTLEV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&self) -> BODINTENA_R {
        BODINTENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&self) -> BODRSTSTAT_R {
        BODRSTSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&self) -> BODINTSTAT_R {
        BODINTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W<0> {
        BODRSTLEV_W::new(self)
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W<2> {
        BODRSTENA_W::new(self)
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&mut self) -> BODINTLEV_W<3> {
        BODINTLEV_W::new(self)
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&mut self) -> BODINTENA_W<5> {
        BODINTENA_W::new(self)
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&mut self) -> BODRSTSTAT_W<6> {
        BODRSTSTAT_W::new(self)
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&mut self) -> BODINTSTAT_W<7> {
        BODINTSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown-Out Detect control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](index.html) module"]
pub struct BODCTRL_SPEC;
impl crate::RegisterSpec for BODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodctrl::R](R) reader structure"]
impl crate::Readable for BODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](W) writer structure"]
impl crate::Writable for BODCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BODCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
