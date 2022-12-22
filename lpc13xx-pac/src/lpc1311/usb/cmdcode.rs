#[doc = "Register `CMDCODE` writer"]
pub struct W(crate::W<CMDCODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDCODE_SPEC>;
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
impl From<crate::W<CMDCODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDCODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command phase action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_PHASE_AW {
    #[doc = "1: Write"]
    WRITE = 1,
    #[doc = "2: Read"]
    READ = 2,
    #[doc = "5: Command"]
    COMMAND = 5,
}
impl From<CMD_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_PHASE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD_PHASE` writer - Command phase action"]
pub type CMD_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDCODE_SPEC, u8, CMD_PHASE_AW, 8, O>;
impl<'a, const O: u8> CMD_PHASE_W<'a, O> {
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::WRITE)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::READ)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::COMMAND)
    }
}
#[doc = "Field `CODE_WDATA` writer - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
pub type CODE_WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDCODE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 8:15 - Command phase action"]
    #[inline(always)]
    pub fn cmd_phase(&mut self) -> CMD_PHASE_W<8> {
        CMD_PHASE_W::new(self)
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    pub fn code_wdata(&mut self) -> CODE_WDATA_W<16> {
        CODE_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command Code\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdcode](index.html) module"]
pub struct CMDCODE_SPEC;
impl crate::RegisterSpec for CMDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmdcode::W](W) writer structure"]
impl crate::Writable for CMDCODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDCODE to value 0"]
impl crate::Resettable for CMDCODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
