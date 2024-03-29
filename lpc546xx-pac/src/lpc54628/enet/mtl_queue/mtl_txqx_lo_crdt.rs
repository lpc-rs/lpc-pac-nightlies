#[doc = "Register `MTL_TXQx_LO_CRDT` reader"]
pub struct R(crate::R<MTL_TXQX_LO_CRDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_LO_CRDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_LO_CRDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_LO_CRDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_LO_CRDT` writer"]
pub struct W(crate::W<MTL_TXQX_LO_CRDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_LO_CRDT_SPEC>;
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
impl From<crate::W<MTL_TXQX_LO_CRDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_LO_CRDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LC` reader - loCredit."]
pub type LC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LC` writer - loCredit."]
pub type LC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MTL_TXQX_LO_CRDT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - loCredit."]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - loCredit."]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W<0> {
        LC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL TxQx loCredit register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_lo_crdt](index.html) module"]
pub struct MTL_TXQX_LO_CRDT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_LO_CRDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_lo_crdt::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_LO_CRDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_lo_crdt::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_LO_CRDT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_LO_CRDT to value 0"]
impl crate::Resettable for MTL_TXQX_LO_CRDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
