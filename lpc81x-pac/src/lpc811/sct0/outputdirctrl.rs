#[doc = "Register `OUTPUTDIRCTRL` reader"]
pub struct R(crate::R<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUTDIRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUTDIRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUTDIRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUTDIRCTRL` writer"]
pub struct W(crate::W<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUTDIRCTRL_SPEC>;
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
impl From<crate::W<OUTPUTDIRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUTDIRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETCLR0` reader - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR0_R = crate::FieldReader<u8, SETCLR0_A>;
#[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR0_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR0_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR0_A) -> Self {
        variant as _
    }
}
impl SETCLR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR0_A> {
        match self.bits {
            0 => Some(SETCLR0_A::INDEPENDENT),
            1 => Some(SETCLR0_A::L_REVERSED),
            2 => Some(SETCLR0_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR0_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR0_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR0` writer - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR0_A, 2, O>;
impl<'a, const O: u8> SETCLR0_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR1` reader - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR1_R = crate::FieldReader<u8, SETCLR1_A>;
#[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR1_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR1_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR1_A) -> Self {
        variant as _
    }
}
impl SETCLR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR1_A> {
        match self.bits {
            0 => Some(SETCLR1_A::INDEPENDENT),
            1 => Some(SETCLR1_A::L_REVERSED),
            2 => Some(SETCLR1_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR1_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR1_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR1` writer - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR1_A, 2, O>;
impl<'a, const O: u8> SETCLR1_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR2` reader - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR2_R = crate::FieldReader<u8, SETCLR2_A>;
#[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR2_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR2_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR2_A) -> Self {
        variant as _
    }
}
impl SETCLR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR2_A> {
        match self.bits {
            0 => Some(SETCLR2_A::INDEPENDENT),
            1 => Some(SETCLR2_A::L_REVERSED),
            2 => Some(SETCLR2_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR2_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR2_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR2` writer - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR2_A, 2, O>;
impl<'a, const O: u8> SETCLR2_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR3` reader - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR3_R = crate::FieldReader<u8, SETCLR3_A>;
#[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR3_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR3_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR3_A) -> Self {
        variant as _
    }
}
impl SETCLR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR3_A> {
        match self.bits {
            0 => Some(SETCLR3_A::INDEPENDENT),
            1 => Some(SETCLR3_A::L_REVERSED),
            2 => Some(SETCLR3_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR3_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR3_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR3` writer - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR3_A, 2, O>;
impl<'a, const O: u8> SETCLR3_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::H_REVERSED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> SETCLR0_R {
        SETCLR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> SETCLR1_R {
        SETCLR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> SETCLR2_R {
        SETCLR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> SETCLR3_R {
        SETCLR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&mut self) -> SETCLR0_W<0> {
        SETCLR0_W::new(self)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&mut self) -> SETCLR1_W<2> {
        SETCLR1_W::new(self)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&mut self) -> SETCLR2_W<4> {
        SETCLR2_W::new(self)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&mut self) -> SETCLR3_W<6> {
        SETCLR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output counter direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdirctrl](index.html) module"]
pub struct OUTPUTDIRCTRL_SPEC;
impl crate::RegisterSpec for OUTPUTDIRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outputdirctrl::R](R) reader structure"]
impl crate::Readable for OUTPUTDIRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outputdirctrl::W](W) writer structure"]
impl crate::Writable for OUTPUTDIRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPUTDIRCTRL to value 0"]
impl crate::Resettable for OUTPUTDIRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
