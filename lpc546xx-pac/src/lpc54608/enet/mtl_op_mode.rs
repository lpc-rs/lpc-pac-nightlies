#[doc = "Register `MTL_OP_MODE` reader"]
pub struct R(crate::R<MTL_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_OP_MODE` writer"]
pub struct W(crate::W<MTL_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_OP_MODE_SPEC>;
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
impl From<crate::W<MTL_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
pub type DTXSTS_R = crate::BitReader<bool>;
#[doc = "Field `DTXSTS` writer - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
pub type DTXSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RAA` reader - Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
pub type RAA_R = crate::BitReader<bool>;
#[doc = "Field `SCHALG` reader - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
pub type SCHALG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCHALG` writer - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
pub type SCHALG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTL_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNTPRST` reader - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
pub type CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRST` writer - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_OP_MODE_SPEC, bool, O>;
#[doc = "Field `CNTCLR` reader - Counters Reset When this bit is set, all counters are reset."]
pub type CNTCLR_R = crate::BitReader<bool>;
#[doc = "Field `CNTCLR` writer - Counters Reset When this bit is set, all counters are reset."]
pub type CNTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_OP_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W<1> {
        DTXSTS_W::new(self)
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&mut self) -> SCHALG_W<5> {
        SCHALG_W::new(self)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W<8> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W<9> {
        CNTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_op_mode](index.html) module"]
pub struct MTL_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_OP_MODE to value 0"]
impl crate::Resettable for MTL_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
