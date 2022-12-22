#[doc = "Register `CIDR1` reader"]
pub struct R(crate::R<CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Preamble` reader - Preamble"]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ComponentClass` reader - Component class"]
pub type COMPONENT_CLASS_R = crate::FieldReader<u8, COMPONENT_CLASS_A>;
#[doc = "Component class\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPONENT_CLASS_A {
    #[doc = "1: ROM table."]
    COMPONENT_CLASS_1 = 1,
    #[doc = "9: CoreSight component."]
    COMPONENT_CLASS_9 = 9,
    #[doc = "15: PrimeCell of system component with no standardized register layout, for backward compatibility."]
    COMPONENT_CLASS_15 = 15,
}
impl From<COMPONENT_CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPONENT_CLASS_A) -> Self {
        variant as _
    }
}
impl COMPONENT_CLASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPONENT_CLASS_A> {
        match self.bits {
            1 => Some(COMPONENT_CLASS_A::COMPONENT_CLASS_1),
            9 => Some(COMPONENT_CLASS_A::COMPONENT_CLASS_9),
            15 => Some(COMPONENT_CLASS_A::COMPONENT_CLASS_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPONENT_CLASS_1`"]
    #[inline(always)]
    pub fn is_component_class_1(&self) -> bool {
        *self == COMPONENT_CLASS_A::COMPONENT_CLASS_1
    }
    #[doc = "Checks if the value of the field is `COMPONENT_CLASS_9`"]
    #[inline(always)]
    pub fn is_component_class_9(&self) -> bool {
        *self == COMPONENT_CLASS_A::COMPONENT_CLASS_9
    }
    #[doc = "Checks if the value of the field is `COMPONENT_CLASS_15`"]
    #[inline(always)]
    pub fn is_component_class_15(&self) -> bool {
        *self == COMPONENT_CLASS_A::COMPONENT_CLASS_15
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn component_class(&self) -> COMPONENT_CLASS_R {
        COMPONENT_CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr1](index.html) module"]
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr1::R](R) reader structure"]
impl crate::Readable for CIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for CIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
