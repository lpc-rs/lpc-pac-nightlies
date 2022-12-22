#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UOF` reader - Untraced overflow flag. If set to 1, there is an overflow that has not yet been traced. This bit is cleared to 0 when either: - trace is restarted - the ETM Power Down bit, bit \\[0\\]
of the ETM Control Register, 0x00, is set to 1. Note: Setting or clearing the ETM programming bit does not cause this bit to be cleared to 0."]
pub type UOF_R = crate::BitReader<bool>;
#[doc = "Field `Progbit` reader - ETM programming bit value (Progbit). The current effective value of the ETM Programming bit (ETM Control Register bit \\[10\\]). Tou must wait for this bit to go to 1 before you start to program the ETM."]
pub type PROGBIT_R = crate::BitReader<bool>;
#[doc = "Field `Status` reader - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `Status` writer - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
pub type STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `Trigger` reader - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `Trigger` writer - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Untraced overflow flag. If set to 1, there is an overflow that has not yet been traced. This bit is cleared to 0 when either: - trace is restarted - the ETM Power Down bit, bit \\[0\\]
of the ETM Control Register, 0x00, is set to 1. Note: Setting or clearing the ETM programming bit does not cause this bit to be cleared to 0."]
    #[inline(always)]
    pub fn uof(&self) -> UOF_R {
        UOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM programming bit value (Progbit). The current effective value of the ETM Programming bit (ETM Control Register bit \\[10\\]). Tou must wait for this bit to go to 1 before you start to program the ETM."]
    #[inline(always)]
    pub fn progbit(&self) -> PROGBIT_R {
        PROGBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W<2> {
        STATUS_W::new(self)
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W<3> {
        TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
