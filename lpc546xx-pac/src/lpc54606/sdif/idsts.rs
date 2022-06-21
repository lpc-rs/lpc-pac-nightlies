#[doc = "Register `IDSTS` reader"]
pub struct R(crate::R<IDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDSTS` writer"]
pub struct W(crate::W<IDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDSTS_SPEC>;
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
impl From<crate::W<IDSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt."]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt."]
pub type TI_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 0>;
#[doc = "Field `RI` reader - Receive Interrupt."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt."]
pub type RI_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 1>;
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt."]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt."]
pub type FBE_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 2>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt."]
pub type DU_R = crate::BitReader<bool>;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt."]
pub type DU_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 4>;
#[doc = "Field `CES` reader - Card Error Summary."]
pub type CES_R = crate::BitReader<bool>;
#[doc = "Field `CES` writer - Card Error Summary."]
pub type CES_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 5>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary."]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary."]
pub type NIS_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 8>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary."]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary."]
pub type AIS_W<'a> = crate::BitWriter<'a, u32, IDSTS_SPEC, bool, 9>;
#[doc = "Field `EB` reader - Error Bits."]
pub type EB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EB` writer - Error Bits."]
pub type EB_W<'a> = crate::FieldWriter<'a, u32, IDSTS_SPEC, u8, u8, 3, 10>;
#[doc = "Field `FSM` reader - DMAC state machine present state."]
pub type FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSM` writer - DMAC state machine present state."]
pub type FSM_W<'a> = crate::FieldWriter<'a, u32, IDSTS_SPEC, u8, u8, 4, 13>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W::new(self)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W::new(self)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W::new(self)
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W::new(self)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W::new(self)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W::new(self)
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&mut self) -> EB_W {
        EB_W::new(self)
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&mut self) -> FSM_W {
        FSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal DMAC Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idsts](index.html) module"]
pub struct IDSTS_SPEC;
impl crate::RegisterSpec for IDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idsts::R](R) reader structure"]
impl crate::Readable for IDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idsts::W](W) writer structure"]
impl crate::Writable for IDSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDSTS to value 0"]
impl crate::Resettable for IDSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
