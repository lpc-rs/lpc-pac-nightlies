#[doc = "Register `DCD_LOC` reader"]
pub struct R(crate::R<DCD_LOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCD_LOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCD_LOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCD_LOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCD_LOC` writer"]
pub struct W(crate::W<DCD_LOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCD_LOC_SPEC>;
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
impl From<crate::W<DCD_LOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCD_LOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDLOC` reader - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
pub type DCDLOC_R = crate::FieldReader<u8, DCDLOC_A>;
#[doc = "Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCDLOC_A {
    #[doc = "0: Selects DCD function in pin location PIO2_2/DCD/MISO1."]
    SELECTS_DCD_FUNCTION = 0,
    #[doc = "1: Selects DCD function in pin location PIO3_2/DCD."]
    SELECTS_DCD_FUNCTIO = 1,
    #[doc = "2: Reserved."]
    RESERVED_2 = 2,
    #[doc = "3: Reserved."]
    RESERVED_3 = 3,
}
impl From<DCDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDLOC_A) -> Self {
        variant as _
    }
}
impl DCDLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDLOC_A {
        match self.bits {
            0 => DCDLOC_A::SELECTS_DCD_FUNCTION,
            1 => DCDLOC_A::SELECTS_DCD_FUNCTIO,
            2 => DCDLOC_A::RESERVED_2,
            3 => DCDLOC_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_DCD_FUNCTION`"]
    #[inline(always)]
    pub fn is_selects_dcd_function(&self) -> bool {
        *self == DCDLOC_A::SELECTS_DCD_FUNCTION
    }
    #[doc = "Checks if the value of the field is `SELECTS_DCD_FUNCTIO`"]
    #[inline(always)]
    pub fn is_selects_dcd_functio(&self) -> bool {
        *self == DCDLOC_A::SELECTS_DCD_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == DCDLOC_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == DCDLOC_A::RESERVED_3
    }
}
#[doc = "Field `DCDLOC` writer - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
pub type DCDLOC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCD_LOC_SPEC, u8, DCDLOC_A, 2, O>;
impl<'a, const O: u8> DCDLOC_W<'a, O> {
    #[doc = "Selects DCD function in pin location PIO2_2/DCD/MISO1."]
    #[inline(always)]
    pub fn selects_dcd_function(self) -> &'a mut W {
        self.variant(DCDLOC_A::SELECTS_DCD_FUNCTION)
    }
    #[doc = "Selects DCD function in pin location PIO3_2/DCD."]
    #[inline(always)]
    pub fn selects_dcd_functio(self) -> &'a mut W {
        self.variant(DCDLOC_A::SELECTS_DCD_FUNCTIO)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut W {
        self.variant(DCDLOC_A::RESERVED_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(DCDLOC_A::RESERVED_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dcdloc(&self) -> DCDLOC_R {
        DCDLOC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dcdloc(&mut self) -> DCDLOC_W<0> {
        DCDLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCD pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcd_loc](index.html) module"]
pub struct DCD_LOC_SPEC;
impl crate::RegisterSpec for DCD_LOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcd_loc::R](R) reader structure"]
impl crate::Readable for DCD_LOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcd_loc::W](W) writer structure"]
impl crate::Writable for DCD_LOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCD_LOC to value 0"]
impl crate::Resettable for DCD_LOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
