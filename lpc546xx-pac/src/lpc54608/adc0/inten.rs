#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQA_INTEN` reader - Sequence A interrupt enable."]
pub type SEQA_INTEN_R = crate::BitReader<SEQA_INTEN_A>;
#[doc = "Sequence A interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTEN_A {
    #[doc = "0: Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED = 1,
}
impl From<SEQA_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQA_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQA_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQA_INTEN_A {
        match self.bits {
            false => SEQA_INTEN_A::DISABLED,
            true => SEQA_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQA_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQA_INTEN_A::ENABLED
    }
}
#[doc = "Field `SEQA_INTEN` writer - Sequence A interrupt enable."]
pub type SEQA_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SEQA_INTEN_A, O>;
impl<'a, const O: u8> SEQA_INTEN_W<'a, O> {
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::ENABLED)
    }
}
#[doc = "Field `SEQB_INTEN` reader - Sequence B interrupt enable."]
pub type SEQB_INTEN_R = crate::BitReader<SEQB_INTEN_A>;
#[doc = "Sequence B interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTEN_A {
    #[doc = "0: Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED = 1,
}
impl From<SEQB_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQB_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQB_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQB_INTEN_A {
        match self.bits {
            false => SEQB_INTEN_A::DISABLED,
            true => SEQB_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_INTEN_A::ENABLED
    }
}
#[doc = "Field `SEQB_INTEN` writer - Sequence B interrupt enable."]
pub type SEQB_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SEQB_INTEN_A, O>;
impl<'a, const O: u8> SEQB_INTEN_W<'a, O> {
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::ENABLED)
    }
}
#[doc = "Field `OVR_INTEN` reader - Overrun interrupt enable."]
pub type OVR_INTEN_R = crate::BitReader<OVR_INTEN_A>;
#[doc = "Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTEN_A {
    #[doc = "0: Disabled. The overrun interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    ENABLED = 1,
}
impl From<OVR_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_INTEN_A {
        match self.bits {
            false => OVR_INTEN_A::DISABLED,
            true => OVR_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_INTEN_A::ENABLED
    }
}
#[doc = "Field `OVR_INTEN` writer - Overrun interrupt enable."]
pub type OVR_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, OVR_INTEN_A, O>;
impl<'a, const O: u8> OVR_INTEN_W<'a, O> {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::ENABLED)
    }
}
#[doc = "Field `ADCMPINTEN0` reader - Threshold comparison interrupt enable for channel 0."]
pub type ADCMPINTEN0_R = crate::FieldReader<u8, ADCMPINTEN0_A>;
#[doc = "Threshold comparison interrupt enable for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCMPINTEN0_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Outside threshold."]
    OUTSIDE_THRESHOLD = 1,
    #[doc = "2: Crossing threshold."]
    CROSSING_THRESHOLD = 2,
}
impl From<ADCMPINTEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMPINTEN0_A) -> Self {
        variant as _
    }
}
impl ADCMPINTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCMPINTEN0_A> {
        match self.bits {
            0 => Some(ADCMPINTEN0_A::DISABLED),
            1 => Some(ADCMPINTEN0_A::OUTSIDE_THRESHOLD),
            2 => Some(ADCMPINTEN0_A::CROSSING_THRESHOLD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN0_A::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN0_A::CROSSING_THRESHOLD
    }
}
#[doc = "Field `ADCMPINTEN0` writer - Threshold comparison interrupt enable for channel 0."]
pub type ADCMPINTEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTEN_SPEC, u8, ADCMPINTEN0_A, 2, O>;
impl<'a, const O: u8> ADCMPINTEN0_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::CROSSING_THRESHOLD)
    }
}
#[doc = "Field `ADCMPINTEN1` reader - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN1` writer - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN2` reader - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN2` writer - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN3` reader - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN3` writer - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN4` reader - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN4` writer - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN5` reader - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN5` writer - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN6` reader - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN6` writer - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN7` reader - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN7` writer - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN8` reader - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN8` writer - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN9` reader - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN9` writer - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN10` reader - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN10` writer - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADCMPINTEN11` reader - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMPINTEN11` writer - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
pub type ADCMPINTEN11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&self) -> SEQA_INTEN_R {
        SEQA_INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&self) -> SEQB_INTEN_R {
        SEQB_INTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&self) -> OVR_INTEN_R {
        OVR_INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0_R {
        ADCMPINTEN0_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1_R {
        ADCMPINTEN1_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2_R {
        ADCMPINTEN2_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3_R {
        ADCMPINTEN3_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4_R {
        ADCMPINTEN4_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5_R {
        ADCMPINTEN5_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6_R {
        ADCMPINTEN6_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7_R {
        ADCMPINTEN7_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8_R {
        ADCMPINTEN8_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9_R {
        ADCMPINTEN9_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10_R {
        ADCMPINTEN10_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11_R {
        ADCMPINTEN11_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&mut self) -> SEQA_INTEN_W<0> {
        SEQA_INTEN_W::new(self)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&mut self) -> SEQB_INTEN_W<1> {
        SEQB_INTEN_W::new(self)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&mut self) -> OVR_INTEN_W<2> {
        OVR_INTEN_W::new(self)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&mut self) -> ADCMPINTEN0_W<3> {
        ADCMPINTEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&mut self) -> ADCMPINTEN1_W<5> {
        ADCMPINTEN1_W::new(self)
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&mut self) -> ADCMPINTEN2_W<7> {
        ADCMPINTEN2_W::new(self)
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&mut self) -> ADCMPINTEN3_W<9> {
        ADCMPINTEN3_W::new(self)
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&mut self) -> ADCMPINTEN4_W<11> {
        ADCMPINTEN4_W::new(self)
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&mut self) -> ADCMPINTEN5_W<13> {
        ADCMPINTEN5_W::new(self)
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&mut self) -> ADCMPINTEN6_W<15> {
        ADCMPINTEN6_W::new(self)
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&mut self) -> ADCMPINTEN7_W<17> {
        ADCMPINTEN7_W::new(self)
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&mut self) -> ADCMPINTEN8_W<19> {
        ADCMPINTEN8_W::new(self)
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&mut self) -> ADCMPINTEN9_W<21> {
        ADCMPINTEN9_W::new(self)
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&mut self) -> ADCMPINTEN10_W<23> {
        ADCMPINTEN10_W::new(self)
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&mut self) -> ADCMPINTEN11_W<25> {
        ADCMPINTEN11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
