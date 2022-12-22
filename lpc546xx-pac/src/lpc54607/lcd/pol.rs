#[doc = "Register `POL` reader"]
pub struct R(crate::R<POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POL` writer"]
pub struct W(crate::W<POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POL_SPEC>;
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
impl From<crate::W<POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCD_LO` reader - Lower five bits of panel clock divisor."]
pub type PCD_LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCD_LO` writer - Lower five bits of panel clock divisor."]
pub type PCD_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POL_SPEC, u8, u8, 5, O>;
#[doc = "Field `ACB` reader - AC bias pin frequency."]
pub type ACB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACB` writer - AC bias pin frequency."]
pub type ACB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POL_SPEC, u8, u8, 5, O>;
#[doc = "Field `IVS` reader - Invert vertical synchronization."]
pub type IVS_R = crate::BitReader<bool>;
#[doc = "Field `IVS` writer - Invert vertical synchronization."]
pub type IVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, bool, O>;
#[doc = "Field `IHS` reader - Invert horizontal synchronization."]
pub type IHS_R = crate::BitReader<bool>;
#[doc = "Field `IHS` writer - Invert horizontal synchronization."]
pub type IHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, bool, O>;
#[doc = "Field `IPC` reader - Invert panel clock."]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - Invert panel clock."]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, bool, O>;
#[doc = "Field `IOE` reader - Invert output enable."]
pub type IOE_R = crate::BitReader<bool>;
#[doc = "Field `IOE` writer - Invert output enable."]
pub type IOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, bool, O>;
#[doc = "Field `CPL` reader - Clocks per line."]
pub type CPL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CPL` writer - Clocks per line."]
pub type CPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POL_SPEC, u16, u16, 10, O>;
#[doc = "Field `BCD` reader - Bypass panel clock divider."]
pub type BCD_R = crate::BitReader<bool>;
#[doc = "Field `BCD` writer - Bypass panel clock divider."]
pub type BCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, bool, O>;
#[doc = "Field `PCD_HI` reader - Upper five bits of panel clock divisor."]
pub type PCD_HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCD_HI` writer - Upper five bits of panel clock divisor."]
pub type PCD_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&self) -> PCD_LO_R {
        PCD_LO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&self) -> ACB_R {
        ACB_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&self) -> IVS_R {
        IVS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&self) -> IHS_R {
        IHS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&self) -> IOE_R {
        IOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&self) -> PCD_HI_R {
        PCD_HI_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&mut self) -> PCD_LO_W<0> {
        PCD_LO_W::new(self)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&mut self) -> ACB_W<6> {
        ACB_W::new(self)
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&mut self) -> IVS_W<11> {
        IVS_W::new(self)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&mut self) -> IHS_W<12> {
        IHS_W::new(self)
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<13> {
        IPC_W::new(self)
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&mut self) -> IOE_W<14> {
        IOE_W::new(self)
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W<16> {
        CPL_W::new(self)
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&mut self) -> BCD_W<26> {
        BCD_W::new(self)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&mut self) -> PCD_HI_W<27> {
        PCD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Signal Polarity Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](index.html) module"]
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pol::R](R) reader structure"]
impl crate::Readable for POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pol::W](W) writer structure"]
impl crate::Writable for POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
