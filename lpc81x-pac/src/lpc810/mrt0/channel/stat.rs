#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTFLAG` reader - Monitors the interrupt flag."]
pub type INTFLAG_R = crate::BitReader<INTFLAG_A>;
#[doc = "Monitors the interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAG_A {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 1,
}
impl From<INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl INTFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTFLAG_A {
        match self.bits {
            false => INTFLAG_A::NO_PENDING_INTERRUPT,
            true => INTFLAG_A::PENDING_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::PENDING_INTERRUPT
    }
}
#[doc = "Field `INTFLAG` writer - Monitors the interrupt flag."]
pub type INTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, INTFLAG_A, O>;
impl<'a, const O: u8> INTFLAG_W<'a, O> {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::PENDING_INTERRUPT)
    }
}
#[doc = "Field `RUN` reader - Indicates the state of TIMERn. This bit is read-only."]
pub type RUN_R = crate::BitReader<RUN_A>;
#[doc = "Indicates the state of TIMERn. This bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: Idle state. TIMERn is stopped."]
    IDLE_STATE = 0,
    #[doc = "1: Running. TIMERn is running."]
    RUNNING = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::IDLE_STATE,
            true => RUN_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_STATE`"]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == RUN_A::IDLE_STATE
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUN_A::RUNNING
    }
}
#[doc = "Field `RUN` writer - Indicates the state of TIMERn. This bit is read-only."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, RUN_A, O>;
impl<'a, const O: u8> RUN_W<'a, O> {
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline(always)]
    pub fn idle_state(self) -> &'a mut W {
        self.variant(RUN_A::IDLE_STATE)
    }
    #[doc = "Running. TIMERn is running."]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(RUN_A::RUNNING)
    }
}
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&mut self) -> INTFLAG_W<0> {
        INTFLAG_W::new(self)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<1> {
        RUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MRT Status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
