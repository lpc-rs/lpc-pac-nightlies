#[doc = "Register `DSR_LOC` reader"]
pub struct R(crate::R<DSR_LOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSR_LOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSR_LOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSR_LOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSR_LOC` writer"]
pub struct W(crate::W<DSR_LOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSR_LOC_SPEC>;
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
impl From<crate::W<DSR_LOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSR_LOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSRLOC` reader - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
pub type DSRLOC_R = crate::FieldReader<u8, DSRLOC_A>;
#[doc = "Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSRLOC_A {
    #[doc = "0: Selects DSR function in pin location PIO2_1/DSR/SCK1."]
    SELECTS_DSR_FUNCTION_0 = 0,
    #[doc = "1: Selects DSR function in pin location PIO3_1/DSR."]
    SELECTS_DSR_FUNCTION_1 = 1,
    #[doc = "2: Reserved."]
    RESERVED_2 = 2,
    #[doc = "3: Reserved."]
    RESERVED_3 = 3,
}
impl From<DSRLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSRLOC_A) -> Self {
        variant as _
    }
}
impl DSRLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRLOC_A {
        match self.bits {
            0 => DSRLOC_A::SELECTS_DSR_FUNCTION_0,
            1 => DSRLOC_A::SELECTS_DSR_FUNCTION_1,
            2 => DSRLOC_A::RESERVED_2,
            3 => DSRLOC_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_DSR_FUNCTION_0`"]
    #[inline(always)]
    pub fn is_selects_dsr_function_0(&self) -> bool {
        *self == DSRLOC_A::SELECTS_DSR_FUNCTION_0
    }
    #[doc = "Checks if the value of the field is `SELECTS_DSR_FUNCTION_1`"]
    #[inline(always)]
    pub fn is_selects_dsr_function_1(&self) -> bool {
        *self == DSRLOC_A::SELECTS_DSR_FUNCTION_1
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == DSRLOC_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == DSRLOC_A::RESERVED_3
    }
}
#[doc = "Field `DSRLOC` writer - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
pub type DSRLOC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DSR_LOC_SPEC, u8, DSRLOC_A, 2, O>;
impl<'a, const O: u8> DSRLOC_W<'a, O> {
    #[doc = "Selects DSR function in pin location PIO2_1/DSR/SCK1."]
    #[inline(always)]
    pub fn selects_dsr_function_0(self) -> &'a mut W {
        self.variant(DSRLOC_A::SELECTS_DSR_FUNCTION_0)
    }
    #[doc = "Selects DSR function in pin location PIO3_1/DSR."]
    #[inline(always)]
    pub fn selects_dsr_function_1(self) -> &'a mut W {
        self.variant(DSRLOC_A::SELECTS_DSR_FUNCTION_1)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut W {
        self.variant(DSRLOC_A::RESERVED_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(DSRLOC_A::RESERVED_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dsrloc(&self) -> DSRLOC_R {
        DSRLOC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dsrloc(&mut self) -> DSRLOC_W<0> {
        DSRLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSR pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_loc](index.html) module"]
pub struct DSR_LOC_SPEC;
impl crate::RegisterSpec for DSR_LOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsr_loc::R](R) reader structure"]
impl crate::Readable for DSR_LOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsr_loc::W](W) writer structure"]
impl crate::Writable for DSR_LOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSR_LOC to value 0"]
impl crate::Resettable for DSR_LOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
