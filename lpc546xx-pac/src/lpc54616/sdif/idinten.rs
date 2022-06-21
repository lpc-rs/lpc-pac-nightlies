#[doc = "Register `IDINTEN` reader"]
pub struct R(crate::R<IDINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDINTEN` writer"]
pub struct W(crate::W<IDINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDINTEN_SPEC>;
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
impl From<crate::W<IDINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt Enable."]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt Enable."]
pub type TI_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 0>;
#[doc = "Field `RI` reader - Receive Interrupt Enable."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt Enable."]
pub type RI_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 1>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable."]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable."]
pub type FBE_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 2>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt."]
pub type DU_R = crate::BitReader<bool>;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt."]
pub type DU_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 4>;
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable."]
pub type CES_R = crate::BitReader<bool>;
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable."]
pub type CES_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 5>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Enable."]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Enable."]
pub type NIS_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 8>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Enable."]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Enable."]
pub type AIS_W<'a> = crate::BitWriter<'a, u32, IDINTEN_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W::new(self)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W::new(self)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W::new(self)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W::new(self)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W::new(self)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal DMAC Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idinten](index.html) module"]
pub struct IDINTEN_SPEC;
impl crate::RegisterSpec for IDINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idinten::R](R) reader structure"]
impl crate::Readable for IDINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idinten::W](W) writer structure"]
impl crate::Writable for IDINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDINTEN to value 0"]
impl crate::Resettable for IDINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
