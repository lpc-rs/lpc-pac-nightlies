#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMP` reader - Lock mechanism is implemented. This bit always reads 1."]
pub type IMP_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` reader - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked."]
pub type STATUS_R = crate::BitReader<STATUS_A>;
#[doc = "Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Access permitted."]
    STATUS_0 = 0,
    #[doc = "1: Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    STATUS_1 = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::STATUS_0,
            true => STATUS_A::STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == STATUS_A::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == STATUS_A::STATUS_1
    }
}
#[doc = "Field `s8BIT` reader - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present."]
pub type S8BIT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Lock mechanism is implemented. This bit always reads 1."]
    #[inline(always)]
    pub fn imp(&self) -> IMP_R {
        IMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present."]
    #[inline(always)]
    pub fn s8bit(&self) -> S8BIT_R {
        S8BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0x01"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
