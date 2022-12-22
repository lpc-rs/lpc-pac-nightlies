#[doc = "Register `SCK0_LOC` reader"]
pub struct R(crate::R<SCK0_LOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCK0_LOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCK0_LOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCK0_LOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCK0_LOC` writer"]
pub struct W(crate::W<SCK0_LOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCK0_LOC_SPEC>;
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
impl From<crate::W<SCK0_LOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCK0_LOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKLOC` reader - Selects pin location for SCK0 pin."]
pub type SCKLOC_R = crate::FieldReader<u8, SCKLOC_A>;
#[doc = "Selects pin location for SCK0 pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCKLOC_A {
    #[doc = "0: Selects SCK0 function for pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 (see Table 121)."]
    SELECTS_SCK0_FUNCTION0 = 0,
    #[doc = "1: Selects SCK0 function for pin PIO2_11/SCK0 (see Table 123"]
    SELECTS_SCK0_FUNCTION1 = 1,
    #[doc = "2: Selects SCK0 function for pin PIO0_6/USB_CONNECT/SCK0 (see Table 114)."]
    SELECTS_SCK0_FUNCTION2 = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<SCKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKLOC_A) -> Self {
        variant as _
    }
}
impl SCKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKLOC_A {
        match self.bits {
            0 => SCKLOC_A::SELECTS_SCK0_FUNCTION0,
            1 => SCKLOC_A::SELECTS_SCK0_FUNCTION1,
            2 => SCKLOC_A::SELECTS_SCK0_FUNCTION2,
            3 => SCKLOC_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTION0`"]
    #[inline(always)]
    pub fn is_selects_sck0_function0(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTION0
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTION1`"]
    #[inline(always)]
    pub fn is_selects_sck0_function1(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTION1
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTION2`"]
    #[inline(always)]
    pub fn is_selects_sck0_function2(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTION2
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == SCKLOC_A::RESERVED_
    }
}
#[doc = "Field `SCKLOC` writer - Selects pin location for SCK0 pin."]
pub type SCKLOC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCK0_LOC_SPEC, u8, SCKLOC_A, 2, O>;
impl<'a, const O: u8> SCKLOC_W<'a, O> {
    #[doc = "Selects SCK0 function for pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 (see Table 121)."]
    #[inline(always)]
    pub fn selects_sck0_function0(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTION0)
    }
    #[doc = "Selects SCK0 function for pin PIO2_11/SCK0 (see Table 123"]
    #[inline(always)]
    pub fn selects_sck0_function1(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTION1)
    }
    #[doc = "Selects SCK0 function for pin PIO0_6/USB_CONNECT/SCK0 (see Table 114)."]
    #[inline(always)]
    pub fn selects_sck0_function2(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTION2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(SCKLOC_A::RESERVED_)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 pin."]
    #[inline(always)]
    pub fn sckloc(&self) -> SCKLOC_R {
        SCKLOC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 pin."]
    #[inline(always)]
    pub fn sckloc(&mut self) -> SCKLOC_W<0> {
        SCKLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCK0 pin location register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck0_loc](index.html) module"]
pub struct SCK0_LOC_SPEC;
impl crate::RegisterSpec for SCK0_LOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sck0_loc::R](R) reader structure"]
impl crate::Readable for SCK0_LOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sck0_loc::W](W) writer structure"]
impl crate::Writable for SCK0_LOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCK0_LOC to value 0"]
impl crate::Resettable for SCK0_LOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
