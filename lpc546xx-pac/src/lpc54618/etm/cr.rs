#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETMPD` reader - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
pub type ETMPD_R = crate::BitReader<bool>;
#[doc = "Field `ETMPD` writer - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
pub type ETMPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PS` reader - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
pub type PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS` writer - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SP` reader - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
pub type SP_R = crate::BitReader<bool>;
#[doc = "Field `SP` writer - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
pub type SP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BO` reader - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DRC` reader - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
pub type DRC_R = crate::BitReader<bool>;
#[doc = "Field `DRC` writer - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
pub type DRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ETMP` reader - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
pub type ETMP_R = crate::BitReader<bool>;
#[doc = "Field `ETMP` writer - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
pub type ETMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ETMPS` reader - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
pub type ETMPS_R = crate::BitReader<ETMPS_A>;
#[doc = "ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMPS_A {
    #[doc = "0: ETMEN is LOW."]
    ETMPS_0 = 0,
    #[doc = "1: ETMEN is HIGH."]
    ETMPS_1 = 1,
}
impl From<ETMPS_A> for bool {
    #[inline(always)]
    fn from(variant: ETMPS_A) -> Self {
        variant as u8 != 0
    }
}
impl ETMPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMPS_A {
        match self.bits {
            false => ETMPS_A::ETMPS_0,
            true => ETMPS_A::ETMPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ETMPS_0`"]
    #[inline(always)]
    pub fn is_etmps_0(&self) -> bool {
        *self == ETMPS_A::ETMPS_0
    }
    #[doc = "Checks if the value of the field is `ETMPS_1`"]
    #[inline(always)]
    pub fn is_etmps_1(&self) -> bool {
        *self == ETMPS_A::ETMPS_1
    }
}
#[doc = "Field `ETMPS` writer - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
pub type ETMPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ETMPS_A, O>;
impl<'a, const O: u8> ETMPS_W<'a, O> {
    #[doc = "ETMEN is LOW."]
    #[inline(always)]
    pub fn etmps_0(self) -> &'a mut W {
        self.variant(ETMPS_A::ETMPS_0)
    }
    #[doc = "ETMEN is HIGH."]
    #[inline(always)]
    pub fn etmps_1(self) -> &'a mut W {
        self.variant(ETMPS_A::ETMPS_1)
    }
}
#[doc = "Field `PM2` reader - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub type PM2_R = crate::BitReader<bool>;
#[doc = "Field `PM2` writer - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub type PM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PM` reader - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
pub type PM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PM` writer - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
pub type PM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PS3` reader - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub type PS3_R = crate::BitReader<bool>;
#[doc = "Field `PS3` writer - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub type PS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TE` reader - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&self) -> DRC_R {
        DRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&self) -> ETMP_R {
        ETMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&self) -> ETMPS_R {
        ETMPS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&self) -> PM2_R {
        PM2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&self) -> PS3_R {
        PS3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&mut self) -> ETMPD_W<0> {
        ETMPD_W::new(self)
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<4> {
        PS_W::new(self)
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<7> {
        SP_W::new(self)
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<8> {
        BO_W::new(self)
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&mut self) -> DRC_W<9> {
        DRC_W::new(self)
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&mut self) -> ETMP_W<10> {
        ETMP_W::new(self)
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&mut self) -> ETMPS_W<11> {
        ETMPS_W::new(self)
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&mut self) -> PM2_W<13> {
        PM2_W::new(self)
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<16> {
        PM_W::new(self)
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&mut self) -> PS3_W<21> {
        PS3_W::new(self)
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<28> {
        TE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0411"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0411
    }
}
