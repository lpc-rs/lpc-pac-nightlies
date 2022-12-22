#[doc = "Register `AUTHSTATUS` reader"]
pub struct R(crate::R<AUTHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NSID` reader - Reads as b00, Non-secure invasive debug not supported by the ETM."]
pub type NSID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSNID` reader - Permission for Non-secure non-invasive debug."]
pub type NSNID_R = crate::FieldReader<u8, NSNID_A>;
#[doc = "Permission for Non-secure non-invasive debug.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSNID_A {
    #[doc = "2: Non-secure non-invasive debug disabled"]
    NSNID_2 = 2,
    #[doc = "3: Non-secure non-invasive debug enabled"]
    NSNID_3 = 3,
}
impl From<NSNID_A> for u8 {
    #[inline(always)]
    fn from(variant: NSNID_A) -> Self {
        variant as _
    }
}
impl NSNID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSNID_A> {
        match self.bits {
            2 => Some(NSNID_A::NSNID_2),
            3 => Some(NSNID_A::NSNID_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NSNID_2`"]
    #[inline(always)]
    pub fn is_nsnid_2(&self) -> bool {
        *self == NSNID_A::NSNID_2
    }
    #[doc = "Checks if the value of the field is `NSNID_3`"]
    #[inline(always)]
    pub fn is_nsnid_3(&self) -> bool {
        *self == NSNID_A::NSNID_3
    }
}
#[doc = "Field `SID` reader - Reads as b00, Secure invasive debug not supported by the ETM."]
pub type SID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNID` reader - Permission for Secure non-invasive debug."]
pub type SNID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Reads as b00, Non-secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn nsid(&self) -> NSID_R {
        NSID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Permission for Non-secure non-invasive debug."]
    #[inline(always)]
    pub fn nsnid(&self) -> NSNID_R {
        NSNID_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Reads as b00, Secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Permission for Secure non-invasive debug."]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstatus](index.html) module"]
pub struct AUTHSTATUS_SPEC;
impl crate::RegisterSpec for AUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [authstatus::R](R) reader structure"]
impl crate::Readable for AUTHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AUTHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
