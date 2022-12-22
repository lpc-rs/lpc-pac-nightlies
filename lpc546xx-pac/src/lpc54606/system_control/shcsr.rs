#[doc = "Register `SHCSR` reader"]
pub struct R(crate::R<SHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHCSR` writer"]
pub struct W(crate::W<SHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHCSR_SPEC>;
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
impl From<crate::W<SHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMFAULTACT` reader - no description available"]
pub type MEMFAULTACT_R = crate::BitReader<MEMFAULTACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACT_A {
    #[doc = "0: exception is not active"]
    MEMFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    MEMFAULTACT_1 = 1,
}
impl From<MEMFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTACT_A {
        match self.bits {
            false => MEMFAULTACT_A::MEMFAULTACT_0,
            true => MEMFAULTACT_A::MEMFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_0`"]
    #[inline(always)]
    pub fn is_memfaultact_0(&self) -> bool {
        *self == MEMFAULTACT_A::MEMFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_1`"]
    #[inline(always)]
    pub fn is_memfaultact_1(&self) -> bool {
        *self == MEMFAULTACT_A::MEMFAULTACT_1
    }
}
#[doc = "Field `MEMFAULTACT` writer - no description available"]
pub type MEMFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTACT_A, O>;
impl<'a, const O: u8> MEMFAULTACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn memfaultact_0(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::MEMFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn memfaultact_1(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::MEMFAULTACT_1)
    }
}
#[doc = "Field `BUSFAULTACT` reader - no description available"]
pub type BUSFAULTACT_R = crate::BitReader<BUSFAULTACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACT_A {
    #[doc = "0: exception is not active"]
    BUSFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    BUSFAULTACT_1 = 1,
}
impl From<BUSFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTACT_A {
        match self.bits {
            false => BUSFAULTACT_A::BUSFAULTACT_0,
            true => BUSFAULTACT_A::BUSFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_0`"]
    #[inline(always)]
    pub fn is_busfaultact_0(&self) -> bool {
        *self == BUSFAULTACT_A::BUSFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_1`"]
    #[inline(always)]
    pub fn is_busfaultact_1(&self) -> bool {
        *self == BUSFAULTACT_A::BUSFAULTACT_1
    }
}
#[doc = "Field `BUSFAULTACT` writer - no description available"]
pub type BUSFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTACT_A, O>;
impl<'a, const O: u8> BUSFAULTACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn busfaultact_0(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::BUSFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn busfaultact_1(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::BUSFAULTACT_1)
    }
}
#[doc = "Field `USGFAULTACT` reader - no description available"]
pub type USGFAULTACT_R = crate::BitReader<USGFAULTACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACT_A {
    #[doc = "0: exception is not active"]
    USGFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    USGFAULTACT_1 = 1,
}
impl From<USGFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTACT_A {
        match self.bits {
            false => USGFAULTACT_A::USGFAULTACT_0,
            true => USGFAULTACT_A::USGFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_0`"]
    #[inline(always)]
    pub fn is_usgfaultact_0(&self) -> bool {
        *self == USGFAULTACT_A::USGFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_1`"]
    #[inline(always)]
    pub fn is_usgfaultact_1(&self) -> bool {
        *self == USGFAULTACT_A::USGFAULTACT_1
    }
}
#[doc = "Field `USGFAULTACT` writer - no description available"]
pub type USGFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTACT_A, O>;
impl<'a, const O: u8> USGFAULTACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn usgfaultact_0(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::USGFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn usgfaultact_1(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::USGFAULTACT_1)
    }
}
#[doc = "Field `SVCALLACT` reader - no description available"]
pub type SVCALLACT_R = crate::BitReader<SVCALLACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACT_A {
    #[doc = "0: exception is not active"]
    SVCALLACT_0 = 0,
    #[doc = "1: exception is active"]
    SVCALLACT_1 = 1,
}
impl From<SVCALLACT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLACT_A) -> Self {
        variant as u8 != 0
    }
}
impl SVCALLACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLACT_A {
        match self.bits {
            false => SVCALLACT_A::SVCALLACT_0,
            true => SVCALLACT_A::SVCALLACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_0`"]
    #[inline(always)]
    pub fn is_svcallact_0(&self) -> bool {
        *self == SVCALLACT_A::SVCALLACT_0
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_1`"]
    #[inline(always)]
    pub fn is_svcallact_1(&self) -> bool {
        *self == SVCALLACT_A::SVCALLACT_1
    }
}
#[doc = "Field `SVCALLACT` writer - no description available"]
pub type SVCALLACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SVCALLACT_A, O>;
impl<'a, const O: u8> SVCALLACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn svcallact_0(self) -> &'a mut W {
        self.variant(SVCALLACT_A::SVCALLACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn svcallact_1(self) -> &'a mut W {
        self.variant(SVCALLACT_A::SVCALLACT_1)
    }
}
#[doc = "Field `MONITORACT` reader - no description available"]
pub type MONITORACT_R = crate::BitReader<MONITORACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACT_A {
    #[doc = "0: exception is not active"]
    MONITORACT_0 = 0,
    #[doc = "1: exception is active"]
    MONITORACT_1 = 1,
}
impl From<MONITORACT_A> for bool {
    #[inline(always)]
    fn from(variant: MONITORACT_A) -> Self {
        variant as u8 != 0
    }
}
impl MONITORACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONITORACT_A {
        match self.bits {
            false => MONITORACT_A::MONITORACT_0,
            true => MONITORACT_A::MONITORACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MONITORACT_0`"]
    #[inline(always)]
    pub fn is_monitoract_0(&self) -> bool {
        *self == MONITORACT_A::MONITORACT_0
    }
    #[doc = "Checks if the value of the field is `MONITORACT_1`"]
    #[inline(always)]
    pub fn is_monitoract_1(&self) -> bool {
        *self == MONITORACT_A::MONITORACT_1
    }
}
#[doc = "Field `MONITORACT` writer - no description available"]
pub type MONITORACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MONITORACT_A, O>;
impl<'a, const O: u8> MONITORACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn monitoract_0(self) -> &'a mut W {
        self.variant(MONITORACT_A::MONITORACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn monitoract_1(self) -> &'a mut W {
        self.variant(MONITORACT_A::MONITORACT_1)
    }
}
#[doc = "Field `PENDSVACT` reader - no description available"]
pub type PENDSVACT_R = crate::BitReader<PENDSVACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVACT_A {
    #[doc = "0: exception is not active"]
    PENDSVACT_0 = 0,
    #[doc = "1: exception is active"]
    PENDSVACT_1 = 1,
}
impl From<PENDSVACT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVACT_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVACT_A {
        match self.bits {
            false => PENDSVACT_A::PENDSVACT_0,
            true => PENDSVACT_A::PENDSVACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_0`"]
    #[inline(always)]
    pub fn is_pendsvact_0(&self) -> bool {
        *self == PENDSVACT_A::PENDSVACT_0
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_1`"]
    #[inline(always)]
    pub fn is_pendsvact_1(&self) -> bool {
        *self == PENDSVACT_A::PENDSVACT_1
    }
}
#[doc = "Field `PENDSVACT` writer - no description available"]
pub type PENDSVACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, PENDSVACT_A, O>;
impl<'a, const O: u8> PENDSVACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn pendsvact_0(self) -> &'a mut W {
        self.variant(PENDSVACT_A::PENDSVACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn pendsvact_1(self) -> &'a mut W {
        self.variant(PENDSVACT_A::PENDSVACT_1)
    }
}
#[doc = "Field `SYSTICKACT` reader - no description available"]
pub type SYSTICKACT_R = crate::BitReader<SYSTICKACT_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACT_A {
    #[doc = "0: exception is not active"]
    SYSTICKACT_0 = 0,
    #[doc = "1: exception is active"]
    SYSTICKACT_1 = 1,
}
impl From<SYSTICKACT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTICKACT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSTICKACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKACT_A {
        match self.bits {
            false => SYSTICKACT_A::SYSTICKACT_0,
            true => SYSTICKACT_A::SYSTICKACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_0`"]
    #[inline(always)]
    pub fn is_systickact_0(&self) -> bool {
        *self == SYSTICKACT_A::SYSTICKACT_0
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_1`"]
    #[inline(always)]
    pub fn is_systickact_1(&self) -> bool {
        *self == SYSTICKACT_A::SYSTICKACT_1
    }
}
#[doc = "Field `SYSTICKACT` writer - no description available"]
pub type SYSTICKACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SYSTICKACT_A, O>;
impl<'a, const O: u8> SYSTICKACT_W<'a, O> {
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn systickact_0(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::SYSTICKACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn systickact_1(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::SYSTICKACT_1)
    }
}
#[doc = "Field `USGFAULTPENDED` reader - no description available"]
pub type USGFAULTPENDED_R = crate::BitReader<USGFAULTPENDED_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    USGFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    USGFAULTPENDED_1 = 1,
}
impl From<USGFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTPENDED_A {
        match self.bits {
            false => USGFAULTPENDED_A::USGFAULTPENDED_0,
            true => USGFAULTPENDED_A::USGFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_usgfaultpended_0(&self) -> bool {
        *self == USGFAULTPENDED_A::USGFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_usgfaultpended_1(&self) -> bool {
        *self == USGFAULTPENDED_A::USGFAULTPENDED_1
    }
}
#[doc = "Field `USGFAULTPENDED` writer - no description available"]
pub type USGFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTPENDED_A, O>;
impl<'a, const O: u8> USGFAULTPENDED_W<'a, O> {
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn usgfaultpended_0(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::USGFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn usgfaultpended_1(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::USGFAULTPENDED_1)
    }
}
#[doc = "Field `MEMFAULTPENDED` reader - no description available"]
pub type MEMFAULTPENDED_R = crate::BitReader<MEMFAULTPENDED_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    MEMFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    MEMFAULTPENDED_1 = 1,
}
impl From<MEMFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTPENDED_A {
        match self.bits {
            false => MEMFAULTPENDED_A::MEMFAULTPENDED_0,
            true => MEMFAULTPENDED_A::MEMFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_memfaultpended_0(&self) -> bool {
        *self == MEMFAULTPENDED_A::MEMFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_memfaultpended_1(&self) -> bool {
        *self == MEMFAULTPENDED_A::MEMFAULTPENDED_1
    }
}
#[doc = "Field `MEMFAULTPENDED` writer - no description available"]
pub type MEMFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTPENDED_A, O>;
impl<'a, const O: u8> MEMFAULTPENDED_W<'a, O> {
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn memfaultpended_0(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::MEMFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn memfaultpended_1(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::MEMFAULTPENDED_1)
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - no description available"]
pub type BUSFAULTPENDED_R = crate::BitReader<BUSFAULTPENDED_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    BUSFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    BUSFAULTPENDED_1 = 1,
}
impl From<BUSFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTPENDED_A {
        match self.bits {
            false => BUSFAULTPENDED_A::BUSFAULTPENDED_0,
            true => BUSFAULTPENDED_A::BUSFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_busfaultpended_0(&self) -> bool {
        *self == BUSFAULTPENDED_A::BUSFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_busfaultpended_1(&self) -> bool {
        *self == BUSFAULTPENDED_A::BUSFAULTPENDED_1
    }
}
#[doc = "Field `BUSFAULTPENDED` writer - no description available"]
pub type BUSFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTPENDED_A, O>;
impl<'a, const O: u8> BUSFAULTPENDED_W<'a, O> {
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn busfaultpended_0(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::BUSFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn busfaultpended_1(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::BUSFAULTPENDED_1)
    }
}
#[doc = "Field `SVCALLPENDED` reader - no description available"]
pub type SVCALLPENDED_R = crate::BitReader<SVCALLPENDED_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDED_A {
    #[doc = "0: exception is not pending"]
    SVCALLPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    SVCALLPENDED_1 = 1,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl SVCALLPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            false => SVCALLPENDED_A::SVCALLPENDED_0,
            true => SVCALLPENDED_A::SVCALLPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_0`"]
    #[inline(always)]
    pub fn is_svcallpended_0(&self) -> bool {
        *self == SVCALLPENDED_A::SVCALLPENDED_0
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_1`"]
    #[inline(always)]
    pub fn is_svcallpended_1(&self) -> bool {
        *self == SVCALLPENDED_A::SVCALLPENDED_1
    }
}
#[doc = "Field `SVCALLPENDED` writer - no description available"]
pub type SVCALLPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SVCALLPENDED_A, O>;
impl<'a, const O: u8> SVCALLPENDED_W<'a, O> {
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn svcallpended_0(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::SVCALLPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn svcallpended_1(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::SVCALLPENDED_1)
    }
}
#[doc = "Field `MEMFAULTENA` reader - no description available"]
pub type MEMFAULTENA_R = crate::BitReader<MEMFAULTENA_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENA_A {
    #[doc = "0: disable the exception"]
    MEMFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    MEMFAULTENA_1 = 1,
}
impl From<MEMFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTENA_A {
        match self.bits {
            false => MEMFAULTENA_A::MEMFAULTENA_0,
            true => MEMFAULTENA_A::MEMFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_0`"]
    #[inline(always)]
    pub fn is_memfaultena_0(&self) -> bool {
        *self == MEMFAULTENA_A::MEMFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_1`"]
    #[inline(always)]
    pub fn is_memfaultena_1(&self) -> bool {
        *self == MEMFAULTENA_A::MEMFAULTENA_1
    }
}
#[doc = "Field `MEMFAULTENA` writer - no description available"]
pub type MEMFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTENA_A, O>;
impl<'a, const O: u8> MEMFAULTENA_W<'a, O> {
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn memfaultena_0(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::MEMFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn memfaultena_1(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::MEMFAULTENA_1)
    }
}
#[doc = "Field `BUSFAULTENA` reader - no description available"]
pub type BUSFAULTENA_R = crate::BitReader<BUSFAULTENA_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENA_A {
    #[doc = "0: disable the exception"]
    BUSFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    BUSFAULTENA_1 = 1,
}
impl From<BUSFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTENA_A {
        match self.bits {
            false => BUSFAULTENA_A::BUSFAULTENA_0,
            true => BUSFAULTENA_A::BUSFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_0`"]
    #[inline(always)]
    pub fn is_busfaultena_0(&self) -> bool {
        *self == BUSFAULTENA_A::BUSFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_1`"]
    #[inline(always)]
    pub fn is_busfaultena_1(&self) -> bool {
        *self == BUSFAULTENA_A::BUSFAULTENA_1
    }
}
#[doc = "Field `BUSFAULTENA` writer - no description available"]
pub type BUSFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTENA_A, O>;
impl<'a, const O: u8> BUSFAULTENA_W<'a, O> {
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn busfaultena_0(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::BUSFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn busfaultena_1(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::BUSFAULTENA_1)
    }
}
#[doc = "Field `USGFAULTENA` reader - no description available"]
pub type USGFAULTENA_R = crate::BitReader<USGFAULTENA_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENA_A {
    #[doc = "0: disable the exception"]
    USGFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    USGFAULTENA_1 = 1,
}
impl From<USGFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTENA_A {
        match self.bits {
            false => USGFAULTENA_A::USGFAULTENA_0,
            true => USGFAULTENA_A::USGFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_0`"]
    #[inline(always)]
    pub fn is_usgfaultena_0(&self) -> bool {
        *self == USGFAULTENA_A::USGFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_1`"]
    #[inline(always)]
    pub fn is_usgfaultena_1(&self) -> bool {
        *self == USGFAULTENA_A::USGFAULTENA_1
    }
}
#[doc = "Field `USGFAULTENA` writer - no description available"]
pub type USGFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTENA_A, O>;
impl<'a, const O: u8> USGFAULTENA_W<'a, O> {
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn usgfaultena_0(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::USGFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn usgfaultena_1(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::USGFAULTENA_1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W<0> {
        MEMFAULTACT_W::new(self)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W<1> {
        BUSFAULTACT_W::new(self)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W<3> {
        USGFAULTACT_W::new(self)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W<7> {
        SVCALLACT_W::new(self)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W<8> {
        MONITORACT_W::new(self)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W<10> {
        PENDSVACT_W::new(self)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W<11> {
        SYSTICKACT_W::new(self)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W<12> {
        USGFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W<13> {
        MEMFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W<14> {
        BUSFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<15> {
        SVCALLPENDED_W::new(self)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W<16> {
        MEMFAULTENA_W::new(self)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W<17> {
        BUSFAULTENA_W::new(self)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W<18> {
        USGFAULTENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](index.html) module"]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shcsr::R](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shcsr::W](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
