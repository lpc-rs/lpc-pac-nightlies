#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
pub type INT_R = crate::BitReader<INT_A>;
#[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_A {
    #[doc = "0: No request. No interrupt request is pending."]
    NO_REQUEST = 0,
    #[doc = "1: Request active. Interrupt request is active."]
    REQUEST_ACTIVE = 1,
}
impl From<INT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_A {
        match self.bits {
            false => INT_A::NO_REQUEST,
            true => INT_A::REQUEST_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == INT_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST_ACTIVE`"]
    #[inline(always)]
    pub fn is_request_active(&self) -> bool {
        *self == INT_A::REQUEST_ACTIVE
    }
}
#[doc = "Field `INT` writer - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
pub type INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, INT_A, O>;
impl<'a, const O: u8> INT_W<'a, O> {
    #[doc = "No request. No interrupt request is pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(INT_A::NO_REQUEST)
    }
    #[doc = "Request active. Interrupt request is active."]
    #[inline(always)]
    pub fn request_active(self) -> &'a mut W {
        self.variant(INT_A::REQUEST_ACTIVE)
    }
}
#[doc = "Field `COMB` reader - Combine enabled inputs for group interrupt"]
pub type COMB_R = crate::BitReader<COMB_A>;
#[doc = "Combine enabled inputs for group interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMB_A {
    #[doc = "0: Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR = 0,
    #[doc = "1: And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND = 1,
}
impl From<COMB_A> for bool {
    #[inline(always)]
    fn from(variant: COMB_A) -> Self {
        variant as u8 != 0
    }
}
impl COMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMB_A {
        match self.bits {
            false => COMB_A::OR,
            true => COMB_A::AND,
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == COMB_A::OR
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == COMB_A::AND
    }
}
#[doc = "Field `COMB` writer - Combine enabled inputs for group interrupt"]
pub type COMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, COMB_A, O>;
impl<'a, const O: u8> COMB_W<'a, O> {
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(COMB_A::OR)
    }
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(COMB_A::AND)
    }
}
#[doc = "Field `TRIG` reader - Group interrupt trigger"]
pub type TRIG_R = crate::BitReader<TRIG_A>;
#[doc = "Group interrupt trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG_A {
    #[doc = "0: Edge-triggered."]
    EDGE_TRIGGERED = 0,
    #[doc = "1: Level-triggered."]
    LEVEL_TRIGGERED = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::EDGE_TRIGGERED,
            true => TRIG_A::LEVEL_TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_TRIGGERED`"]
    #[inline(always)]
    pub fn is_edge_triggered(&self) -> bool {
        *self == TRIG_A::EDGE_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `LEVEL_TRIGGERED`"]
    #[inline(always)]
    pub fn is_level_triggered(&self) -> bool {
        *self == TRIG_A::LEVEL_TRIGGERED
    }
}
#[doc = "Field `TRIG` writer - Group interrupt trigger"]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TRIG_A, O>;
impl<'a, const O: u8> TRIG_W<'a, O> {
    #[doc = "Edge-triggered."]
    #[inline(always)]
    pub fn edge_triggered(self) -> &'a mut W {
        self.variant(TRIG_A::EDGE_TRIGGERED)
    }
    #[doc = "Level-triggered."]
    #[inline(always)]
    pub fn level_triggered(self) -> &'a mut W {
        self.variant(TRIG_A::LEVEL_TRIGGERED)
    }
}
impl R {
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&self) -> COMB_R {
        COMB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&mut self) -> COMB_W<1> {
        COMB_W::new(self)
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W<2> {
        TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
