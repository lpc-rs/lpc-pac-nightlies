#[doc = "Register `MTL_TXQx_ETS_CTRL` reader"]
pub struct R(crate::R<MTL_TXQX_ETS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_ETS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_ETS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_ETS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_ETS_CTRL` writer"]
pub struct W(crate::W<MTL_TXQX_ETS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_ETS_CTRL_SPEC>;
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
impl From<crate::W<MTL_TXQX_ETS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_ETS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVALG` reader - AV Algorithm."]
pub type AVALG_R = crate::BitReader<bool>;
#[doc = "Field `AVALG` writer - AV Algorithm."]
pub type AVALG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_ETS_CTRL_SPEC, bool, O>;
#[doc = "Field `CC` reader - Credit Control."]
pub type CC_R = crate::BitReader<bool>;
#[doc = "Field `CC` writer - Credit Control."]
pub type CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_ETS_CTRL_SPEC, bool, O>;
#[doc = "Field `SLC` reader - Credit Control."]
pub type SLC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 2 - AV Algorithm."]
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Credit Control."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Credit Control."]
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AV Algorithm."]
    #[inline(always)]
    pub fn avalg(&mut self) -> AVALG_W<2> {
        AVALG_W::new(self)
    }
    #[doc = "Bit 3 - Credit Control."]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W<3> {
        CC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL TxQx ETS control register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_ets_ctrl](index.html) module"]
pub struct MTL_TXQX_ETS_CTRL_SPEC;
impl crate::RegisterSpec for MTL_TXQX_ETS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_ets_ctrl::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_ETS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_ets_ctrl::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_ETS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_ETS_CTRL to value 0"]
impl crate::Resettable for MTL_TXQX_ETS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
