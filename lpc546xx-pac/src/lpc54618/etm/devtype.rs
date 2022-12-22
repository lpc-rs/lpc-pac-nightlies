#[doc = "Register `DEVTYPE` reader"]
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MajorType` reader - Major Type and Class"]
pub type MAJOR_TYPE_R = crate::FieldReader<u8, MAJOR_TYPE_A>;
#[doc = "Major Type and Class\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAJOR_TYPE_A {
    #[doc = "3: Trace source"]
    MAJOR_TYPE_3 = 3,
}
impl From<MAJOR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJOR_TYPE_A) -> Self {
        variant as _
    }
}
impl MAJOR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAJOR_TYPE_A> {
        match self.bits {
            3 => Some(MAJOR_TYPE_A::MAJOR_TYPE_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAJOR_TYPE_3`"]
    #[inline(always)]
    pub fn is_major_type_3(&self) -> bool {
        *self == MAJOR_TYPE_A::MAJOR_TYPE_3
    }
}
#[doc = "Field `SubType` reader - Sub Type"]
pub type SUB_TYPE_R = crate::FieldReader<u8, SUB_TYPE_A>;
#[doc = "Sub Type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUB_TYPE_A {
    #[doc = "1: Processor trace"]
    SUB_TYPE_1 = 1,
}
impl From<SUB_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_TYPE_A) -> Self {
        variant as _
    }
}
impl SUB_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUB_TYPE_A> {
        match self.bits {
            1 => Some(SUB_TYPE_A::SUB_TYPE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SUB_TYPE_1`"]
    #[inline(always)]
    pub fn is_sub_type_1(&self) -> bool {
        *self == SUB_TYPE_A::SUB_TYPE_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Major Type and Class"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJOR_TYPE_R {
        MAJOR_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub Type"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUB_TYPE_R {
        SUB_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](index.html) module"]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devtype::R](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVTYPE to value 0x13"]
impl crate::Resettable for DEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13
    }
}
