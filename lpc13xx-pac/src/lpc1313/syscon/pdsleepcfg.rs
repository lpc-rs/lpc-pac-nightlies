#[doc = "Register `PDSLEEPCFG` reader"]
pub struct R(crate::R<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSLEEPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSLEEPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSLEEPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSLEEPCFG` writer"]
pub struct W(crate::W<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSLEEPCFG_SPEC>;
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
impl From<crate::W<PDSLEEPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSLEEPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIXEDVAL0` reader - Reserved. Always write these bits as 111."]
pub type FIXEDVAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXEDVAL0` writer - Reserved. Always write these bits as 111."]
pub type FIXEDVAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDSLEEPCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `BOD_PD` reader - BOD power-down control in Deep-sleep mode, see Table 49."]
pub type BOD_PD_R = crate::BitReader<BOD_PD_A>;
#[doc = "BOD power-down control in Deep-sleep mode, see Table 49.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl BOD_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `BOD_PD` writer - BOD power-down control in Deep-sleep mode, see Table 49."]
pub type BOD_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG_SPEC, BOD_PD_A, O>;
impl<'a, const O: u8> BOD_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `FIXEDVAL1` reader - Reserved. Always write these bits as 11."]
pub type FIXEDVAL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXEDVAL1` writer - Reserved. Always write these bits as 11."]
pub type FIXEDVAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDSLEEPCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator power control in Deep-sleep mode, see Table 49."]
pub type WDTOSC_PD_R = crate::BitReader<WDTOSC_PD_A>;
#[doc = "Watchdog oscillator power control in Deep-sleep mode, see Table 49.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::POWERED,
            true => WDTOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator power control in Deep-sleep mode, see Table 49."]
pub type WDTOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG_SPEC, WDTOSC_PD_A, O>;
impl<'a, const O: u8> WDTOSC_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `FIXEDVAL2` reader - Reserved. Always write these bits as 11111."]
pub type FIXEDVAL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXEDVAL2` writer - Reserved. Always write these bits as 11111."]
pub type FIXEDVAL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDSLEEPCFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn fixedval0(&self) -> FIXEDVAL0_R {
        FIXEDVAL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 49."]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn fixedval1(&self) -> FIXEDVAL1_R {
        FIXEDVAL1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 49."]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Reserved. Always write these bits as 11111."]
    #[inline(always)]
    pub fn fixedval2(&self) -> FIXEDVAL2_R {
        FIXEDVAL2_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn fixedval0(&mut self) -> FIXEDVAL0_W<0> {
        FIXEDVAL0_W::new(self)
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 49."]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W<3> {
        BOD_PD_W::new(self)
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn fixedval1(&mut self) -> FIXEDVAL1_W<4> {
        FIXEDVAL1_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 49."]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W<6> {
        WDTOSC_PD_W::new(self)
    }
    #[doc = "Bits 7:11 - Reserved. Always write these bits as 11111."]
    #[inline(always)]
    pub fn fixedval2(&mut self) -> FIXEDVAL2_W<7> {
        FIXEDVAL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-down states in Deep-sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg](index.html) module"]
pub struct PDSLEEPCFG_SPEC;
impl crate::RegisterSpec for PDSLEEPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsleepcfg::R](R) reader structure"]
impl crate::Readable for PDSLEEPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg::W](W) writer structure"]
impl crate::Writable for PDSLEEPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSLEEPCFG to value 0"]
impl crate::Resettable for PDSLEEPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
