#[doc = "Register `RI_LOC` reader"]
pub struct R(crate::R<RI_LOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RI_LOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RI_LOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RI_LOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RI_LOC` writer"]
pub struct W(crate::W<RI_LOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RI_LOC_SPEC>;
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
impl From<crate::W<RI_LOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RI_LOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RILOC` reader - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
pub type RILOC_R = crate::FieldReader<u8, RILOC_A>;
#[doc = "Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RILOC_A {
    #[doc = "0: Selects RI function in pin location PIO2_3/RI/MOSI1."]
    SELECTS_RI_FUNCTION_0 = 0,
    #[doc = "1: Selects RI function in pin location PIO3_3/RI."]
    SELECTS_RI_FUNCTION_1 = 1,
    #[doc = "2: Reserved."]
    RESERVED_2 = 2,
    #[doc = "3: Reserved."]
    RESERVED_3 = 3,
}
impl From<RILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RILOC_A) -> Self {
        variant as _
    }
}
impl RILOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RILOC_A {
        match self.bits {
            0 => RILOC_A::SELECTS_RI_FUNCTION_0,
            1 => RILOC_A::SELECTS_RI_FUNCTION_1,
            2 => RILOC_A::RESERVED_2,
            3 => RILOC_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_RI_FUNCTION_0`"]
    #[inline(always)]
    pub fn is_selects_ri_function_0(&self) -> bool {
        *self == RILOC_A::SELECTS_RI_FUNCTION_0
    }
    #[doc = "Checks if the value of the field is `SELECTS_RI_FUNCTION_1`"]
    #[inline(always)]
    pub fn is_selects_ri_function_1(&self) -> bool {
        *self == RILOC_A::SELECTS_RI_FUNCTION_1
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == RILOC_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == RILOC_A::RESERVED_3
    }
}
#[doc = "Field `RILOC` writer - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
pub type RILOC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RI_LOC_SPEC, u8, RILOC_A, 2, O>;
impl<'a, const O: u8> RILOC_W<'a, O> {
    #[doc = "Selects RI function in pin location PIO2_3/RI/MOSI1."]
    #[inline(always)]
    pub fn selects_ri_function_0(self) -> &'a mut W {
        self.variant(RILOC_A::SELECTS_RI_FUNCTION_0)
    }
    #[doc = "Selects RI function in pin location PIO3_3/RI."]
    #[inline(always)]
    pub fn selects_ri_function_1(self) -> &'a mut W {
        self.variant(RILOC_A::SELECTS_RI_FUNCTION_1)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut W {
        self.variant(RILOC_A::RESERVED_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(RILOC_A::RESERVED_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
    #[inline(always)]
    pub fn riloc(&self) -> RILOC_R {
        RILOC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
    #[inline(always)]
    pub fn riloc(&mut self) -> RILOC_W<0> {
        RILOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI pin location register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ri_loc](index.html) module"]
pub struct RI_LOC_SPEC;
impl crate::RegisterSpec for RI_LOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ri_loc::R](R) reader structure"]
impl crate::Readable for RI_LOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ri_loc::W](W) writer structure"]
impl crate::Writable for RI_LOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RI_LOC to value 0"]
impl crate::Resettable for RI_LOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
