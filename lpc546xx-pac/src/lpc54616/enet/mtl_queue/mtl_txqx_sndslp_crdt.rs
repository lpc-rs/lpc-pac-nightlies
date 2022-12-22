#[doc = "Register `MTL_TXQx_SNDSLP_CRDT` reader"]
pub struct R(crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_SNDSLP_CRDT` writer"]
pub struct W(crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>;
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
impl From<crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSC` reader - sendSlopeCredit."]
pub type SSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSC` writer - sendSlopeCredit."]
pub type SSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MTL_TXQX_SNDSLP_CRDT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - sendSlopeCredit."]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - sendSlopeCredit."]
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W<0> {
        SSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL TxQx SendSlopCredit register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_sndslp_crdt](index.html) module"]
pub struct MTL_TXQX_SNDSLP_CRDT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_sndslp_crdt::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_sndslp_crdt::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_SNDSLP_CRDT to value 0"]
impl crate::Resettable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
