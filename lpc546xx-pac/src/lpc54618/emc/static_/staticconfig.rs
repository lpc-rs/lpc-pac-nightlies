#[doc = "Register `STATICCONFIG` reader"]
pub struct R(crate::R<STATICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONFIG` writer"]
pub struct W(crate::W<STATICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONFIG_SPEC>;
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
impl From<crate::W<STATICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MW` reader - Memory width."]
pub type MW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MW` writer - Memory width."]
pub type MW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATICCONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PM` reader - Page mode."]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Page mode."]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
#[doc = "Field `PC` reader - Chip select polarity."]
pub type PC_R = crate::BitReader<bool>;
#[doc = "Field `PC` writer - Chip select polarity."]
pub type PC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
#[doc = "Field `PB` reader - Byte lane state."]
pub type PB_R = crate::BitReader<bool>;
#[doc = "Field `PB` writer - Byte lane state."]
pub type PB_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
#[doc = "Field `EW` reader - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
#[doc = "Field `B` reader - Buffer enable \\[2\\]."]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Buffer enable \\[2\\]."]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
#[doc = "Field `P` reader - Write protect."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - Write protect."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Page mode."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Chip select polarity."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Byte lane state."]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W<0> {
        MW_W::new(self)
    }
    #[doc = "Bit 3 - Page mode."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<3> {
        PM_W::new(self)
    }
    #[doc = "Bit 6 - Chip select polarity."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<6> {
        PC_W::new(self)
    }
    #[doc = "Bit 7 - Byte lane state."]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W<7> {
        PB_W::new(self)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<8> {
        EW_W::new(self)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<19> {
        B_W::new(self)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W<20> {
        P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for EMC_CSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticconfig](index.html) module"]
pub struct STATICCONFIG_SPEC;
impl crate::RegisterSpec for STATICCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticconfig::R](R) reader structure"]
impl crate::Readable for STATICCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticconfig::W](W) writer structure"]
impl crate::Writable for STATICCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICCONFIG to value 0"]
impl crate::Resettable for STATICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
