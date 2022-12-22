#[doc = "Register `RES` reader"]
pub struct R(crate::R<RES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES` writer"]
pub struct W(crate::W<RES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_SPEC>;
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
impl From<crate::W<RES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `O0RES` reader - Effect of simultaneous set and clear on output 0."]
pub type O0RES_R = crate::FieldReader<u8, O0RES_A>;
#[doc = "Effect of simultaneous set and clear on output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O0RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR0 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O0RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O0RES_A) -> Self {
        variant as _
    }
}
impl O0RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O0RES_A {
        match self.bits {
            0 => O0RES_A::NO_CHANGE,
            1 => O0RES_A::SET,
            2 => O0RES_A::CLEAR,
            3 => O0RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O0RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O0RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O0RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Field `O0RES` writer - Effect of simultaneous set and clear on output 0."]
pub type O0RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RES_SPEC, u8, O0RES_A, 2, O>;
impl<'a, const O: u8> O0RES_W<'a, O> {
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O0RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O0RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RES_A::TOGGLE_OUTPUT)
    }
}
#[doc = "Field `O1RES` reader - Effect of simultaneous set and clear on output 1."]
pub type O1RES_R = crate::FieldReader<u8, O1RES_A>;
#[doc = "Effect of simultaneous set and clear on output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O1RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR1 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O1RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O1RES_A) -> Self {
        variant as _
    }
}
impl O1RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O1RES_A {
        match self.bits {
            0 => O1RES_A::NO_CHANGE,
            1 => O1RES_A::SET,
            2 => O1RES_A::CLEAR,
            3 => O1RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O1RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O1RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O1RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Field `O1RES` writer - Effect of simultaneous set and clear on output 1."]
pub type O1RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RES_SPEC, u8, O1RES_A, 2, O>;
impl<'a, const O: u8> O1RES_W<'a, O> {
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O1RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O1RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RES_A::TOGGLE_OUTPUT)
    }
}
#[doc = "Field `O2RES` reader - Effect of simultaneous set and clear on output 2."]
pub type O2RES_R = crate::FieldReader<u8, O2RES_A>;
#[doc = "Effect of simultaneous set and clear on output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O2RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output n (or set based on the SETCLR2 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O2RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O2RES_A) -> Self {
        variant as _
    }
}
impl O2RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O2RES_A {
        match self.bits {
            0 => O2RES_A::NO_CHANGE,
            1 => O2RES_A::SET,
            2 => O2RES_A::CLEAR,
            3 => O2RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O2RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O2RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O2RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Field `O2RES` writer - Effect of simultaneous set and clear on output 2."]
pub type O2RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RES_SPEC, u8, O2RES_A, 2, O>;
impl<'a, const O: u8> O2RES_W<'a, O> {
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O2RES_A::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O2RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RES_A::TOGGLE_OUTPUT)
    }
}
#[doc = "Field `O3RES` reader - Effect of simultaneous set and clear on output 3."]
pub type O3RES_R = crate::FieldReader<u8, O3RES_A>;
#[doc = "Effect of simultaneous set and clear on output 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O3RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR3 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O3RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O3RES_A) -> Self {
        variant as _
    }
}
impl O3RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O3RES_A {
        match self.bits {
            0 => O3RES_A::NO_CHANGE,
            1 => O3RES_A::SET,
            2 => O3RES_A::CLEAR,
            3 => O3RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O3RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O3RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O3RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Field `O3RES` writer - Effect of simultaneous set and clear on output 3."]
pub type O3RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RES_SPEC, u8, O3RES_A, 2, O>;
impl<'a, const O: u8> O3RES_W<'a, O> {
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O3RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O3RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RES_A::TOGGLE_OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&self) -> O0RES_R {
        O0RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&self) -> O1RES_R {
        O1RES_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&self) -> O2RES_R {
        O2RES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&self) -> O3RES_R {
        O3RES_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&mut self) -> O0RES_W<0> {
        O0RES_W::new(self)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&mut self) -> O1RES_W<2> {
        O1RES_W::new(self)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&mut self) -> O2RES_W<4> {
        O2RES_W::new(self)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&mut self) -> O3RES_W<6> {
        O3RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict resolution register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](index.html) module"]
pub struct RES_SPEC;
impl crate::RegisterSpec for RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res::R](R) reader structure"]
impl crate::Readable for RES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res::W](W) writer structure"]
impl crate::Writable for RES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES to value 0"]
impl crate::Resettable for RES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
