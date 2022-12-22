#[doc = "Register `LAD` reader"]
pub struct R(crate::R<LAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAD` writer"]
pub struct W(crate::W<LAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAD_SPEC>;
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
impl From<crate::W<LAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LADEN` reader - Voltage ladder enable"]
pub type LADEN_R = crate::BitReader<bool>;
#[doc = "Field `LADEN` writer - Voltage ladder enable"]
pub type LADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAD_SPEC, bool, O>;
#[doc = "Field `LADSEL` reader - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
pub type LADSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LADSEL` writer - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
pub type LADSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAD_SPEC, u8, u8, 5, O>;
#[doc = "Field `LADREF` reader - Selects the reference voltage Vref for the voltage ladder."]
pub type LADREF_R = crate::BitReader<LADREF_A>;
#[doc = "Selects the reference voltage Vref for the voltage ladder.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADREF_A {
    #[doc = "0: Supply pin VDD"]
    LADREF_0 = 0,
    #[doc = "1: VDDCMP pin"]
    LADREF_1 = 1,
}
impl From<LADREF_A> for bool {
    #[inline(always)]
    fn from(variant: LADREF_A) -> Self {
        variant as u8 != 0
    }
}
impl LADREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LADREF_A {
        match self.bits {
            false => LADREF_A::LADREF_0,
            true => LADREF_A::LADREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LADREF_0`"]
    #[inline(always)]
    pub fn is_ladref_0(&self) -> bool {
        *self == LADREF_A::LADREF_0
    }
    #[doc = "Checks if the value of the field is `LADREF_1`"]
    #[inline(always)]
    pub fn is_ladref_1(&self) -> bool {
        *self == LADREF_A::LADREF_1
    }
}
#[doc = "Field `LADREF` writer - Selects the reference voltage Vref for the voltage ladder."]
pub type LADREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAD_SPEC, LADREF_A, O>;
impl<'a, const O: u8> LADREF_W<'a, O> {
    #[doc = "Supply pin VDD"]
    #[inline(always)]
    pub fn ladref_0(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_0)
    }
    #[doc = "VDDCMP pin"]
    #[inline(always)]
    pub fn ladref_1(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&self) -> LADEN_R {
        LADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&self) -> LADSEL_R {
        LADSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&self) -> LADREF_R {
        LADREF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&mut self) -> LADEN_W<0> {
        LADEN_W::new(self)
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&mut self) -> LADSEL_W<1> {
        LADSEL_W::new(self)
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&mut self) -> LADREF_W<6> {
        LADREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage ladder register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lad](index.html) module"]
pub struct LAD_SPEC;
impl crate::RegisterSpec for LAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lad::R](R) reader structure"]
impl crate::Readable for LAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lad::W](W) writer structure"]
impl crate::Writable for LAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAD to value 0"]
impl crate::Resettable for LAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
