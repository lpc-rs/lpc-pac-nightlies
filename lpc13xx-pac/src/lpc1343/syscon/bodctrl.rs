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
#[doc = "Field `BODRSTLEV` reader - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
pub type BODRSTLEV_R = crate::FieldReader<u8, BODRSTLEV_A>;
#[doc = "BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: The reset assertion threshold voltage is 1.49 V/1.46 V; the reset de-assertion threshold voltage is 1.64 V/1.63 V."]
    LEVEL0 = 0,
    #[doc = "1: The reset assertion threshold voltage is -/2.06 V; the reset de-assertion threshold voltage is -/2.15 V."]
    LEVEL1 = 1,
    #[doc = "2: The reset assertion threshold voltage is -/2.35 V; the reset de-assertion threshold voltage is -/2.43 V."]
    LEVEL2 = 2,
    #[doc = "3: The reset assertion threshold voltage is -/2.63 V; the reset de-assertion threshold voltage is -/2.71 V."]
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
#[doc = "Field `BODRSTLEV` writer - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
pub type BODRSTLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BODCTRL_SPEC, u8, BODRSTLEV_A, 2, O>;
impl<'a, const O: u8> BODRSTLEV_W<'a, O> {
    #[doc = "The reset assertion threshold voltage is 1.49 V/1.46 V; the reset de-assertion threshold voltage is 1.64 V/1.63 V."]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL0)
    }
    #[doc = "The reset assertion threshold voltage is -/2.06 V; the reset de-assertion threshold voltage is -/2.15 V."]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL1)
    }
    #[doc = "The reset assertion threshold voltage is -/2.35 V; the reset de-assertion threshold voltage is -/2.43 V."]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL2)
    }
    #[doc = "The reset assertion threshold voltage is -/2.63 V; the reset de-assertion threshold voltage is -/2.71 V."]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL3)
    }
}
#[doc = "Field `BODINTVAL` reader - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
pub type BODINTVAL_R = crate::FieldReader<u8, BODINTVAL_A>;
#[doc = "BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTVAL_A {
    #[doc = "0: The interrupt assertion threshold voltage is 1.69 V/1.65 V; the interrupt de-assertion threshold voltage is 1.84 V/1.8 V."]
    LEVEL0 = 0,
    #[doc = "1: The interrupt assertion threshold voltage is 2.29 V/2.22 V; the interrupt de-assertion threshold voltage is 2.44 V/2.35 V."]
    LEVEL1 = 1,
    #[doc = "2: The interrupt assertion threshold voltage is 2.59 V/ 2.52 V; the interrupt de-assertion threshold voltage is 2.74 V/2.66 V."]
    LEVEL2 = 2,
    #[doc = "3: The interrupt assertion threshold voltage is 2.87 V/2.80 V; the interrupt de-assertion threshold voltage is 2.98 V/2.90 V."]
    LEVEL3 = 3,
}
impl From<BODINTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTVAL_A) -> Self {
        variant as _
    }
}
impl BODINTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTVAL_A {
        match self.bits {
            0 => BODINTVAL_A::LEVEL0,
            1 => BODINTVAL_A::LEVEL1,
            2 => BODINTVAL_A::LEVEL2,
            3 => BODINTVAL_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODINTVAL_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODINTVAL_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODINTVAL_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODINTVAL_A::LEVEL3
    }
}
#[doc = "Field `BODINTVAL` writer - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
pub type BODINTVAL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BODCTRL_SPEC, u8, BODINTVAL_A, 2, O>;
impl<'a, const O: u8> BODINTVAL_W<'a, O> {
    #[doc = "The interrupt assertion threshold voltage is 1.69 V/1.65 V; the interrupt de-assertion threshold voltage is 1.84 V/1.8 V."]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL0)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.29 V/2.22 V; the interrupt de-assertion threshold voltage is 2.44 V/2.35 V."]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL1)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.59 V/ 2.52 V; the interrupt de-assertion threshold voltage is 2.74 V/2.66 V."]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL2)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.87 V/2.80 V; the interrupt de-assertion threshold voltage is 2.98 V/2.90 V."]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL3)
    }
}
#[doc = "Field `BODRSTENA` reader - BOD reset enable"]
pub type BODRSTENA_R = crate::BitReader<BODRSTENA_A>;
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE_RESET_FUNCTI = 0,
    #[doc = "1: Enable reset function."]
    ENABLE_RESET_FUNCTIO = 1,
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
            false => BODRSTENA_A::DISABLE_RESET_FUNCTI,
            true => BODRSTENA_A::ENABLE_RESET_FUNCTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_RESET_FUNCTI`"]
    #[inline(always)]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == BODRSTENA_A::DISABLE_RESET_FUNCTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_FUNCTIO`"]
    #[inline(always)]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == BODRSTENA_A::ENABLE_RESET_FUNCTIO
    }
}
#[doc = "Field `BODRSTENA` writer - BOD reset enable"]
pub type BODRSTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCTRL_SPEC, BODRSTENA_A, O>;
impl<'a, const O: u8> BODRSTENA_W<'a, O> {
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable_reset_functi(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE_RESET_FUNCTI)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable_reset_functio(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE_RESET_FUNCTIO)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodintval(&self) -> BODINTVAL_R {
        BODINTVAL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W<0> {
        BODRSTLEV_W::new(self)
    }
    #[doc = "Bits 2:3 - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodintval(&mut self) -> BODINTVAL_W<2> {
        BODINTVAL_W::new(self)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W<4> {
        BODRSTENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](index.html) module"]
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
