#[doc = "Register `SLVCTL` reader"]
pub struct R(crate::R<SLVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVCTL` writer"]
pub struct W(crate::W<SLVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVCTL_SPEC>;
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
impl From<crate::W<SLVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLVCONTINUE` reader - Slave Continue."]
pub type SLVCONTINUE_R = crate::BitReader<SLVCONTINUE_A>;
#[doc = "Slave Continue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUE_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Continue. Informs the Slave function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE = 1,
}
impl From<SLVCONTINUE_A> for bool {
    #[inline(always)]
    fn from(variant: SLVCONTINUE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVCONTINUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVCONTINUE_A {
        match self.bits {
            false => SLVCONTINUE_A::NO_EFFECT,
            true => SLVCONTINUE_A::CONTINUE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVCONTINUE_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == SLVCONTINUE_A::CONTINUE
    }
}
#[doc = "Field `SLVCONTINUE` writer - Slave Continue."]
pub type SLVCONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVCTL_SPEC, SLVCONTINUE_A, O>;
impl<'a, const O: u8> SLVCONTINUE_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVCONTINUE_A::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Slave function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(SLVCONTINUE_A::CONTINUE)
    }
}
#[doc = "Field `SLVNACK` reader - Slave NACK."]
pub type SLVNACK_R = crate::BitReader<SLVNACK_A>;
#[doc = "Slave NACK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACK_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK = 1,
}
impl From<SLVNACK_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNACK_A {
        match self.bits {
            false => SLVNACK_A::NO_EFFECT,
            true => SLVNACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVNACK_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == SLVNACK_A::NACK
    }
}
#[doc = "Field `SLVNACK` writer - Slave NACK."]
pub type SLVNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVCTL_SPEC, SLVNACK_A, O>;
impl<'a, const O: u8> SLVNACK_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVNACK_A::NO_EFFECT)
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(SLVNACK_A::NACK)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&self) -> SLVCONTINUE_R {
        SLVCONTINUE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&self) -> SLVNACK_R {
        SLVNACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&mut self) -> SLVCONTINUE_W<0> {
        SLVCONTINUE_W::new(self)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&mut self) -> SLVNACK_W<1> {
        SLVNACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvctl](index.html) module"]
pub struct SLVCTL_SPEC;
impl crate::RegisterSpec for SLVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvctl::R](R) reader structure"]
impl crate::Readable for SLVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvctl::W](W) writer structure"]
impl crate::Writable for SLVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLVCTL to value 0"]
impl crate::Resettable for SLVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
