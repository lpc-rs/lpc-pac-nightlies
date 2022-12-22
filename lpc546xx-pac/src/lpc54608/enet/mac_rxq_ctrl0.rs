#[doc = "Register `MAC_RXQ_CTRL0` reader"]
pub struct R(crate::R<MAC_RXQ_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RXQ_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RXQ_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_RXQ_CTRL0` writer"]
pub struct W(crate::W<MAC_RXQ_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL0_SPEC>;
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
impl From<crate::W<MAC_RXQ_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RXQ_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ0EN` reader - Receive Queue 0 Enable."]
pub type RXQ0EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ0EN` writer - Receive Queue 0 Enable."]
pub type RXQ0EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXQ1EN` reader - Receive Queue 1 Enable."]
pub type RXQ1EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ1EN` writer - Receive Queue 1 Enable."]
pub type RXQ1EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Receive Queue 0 Enable."]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Receive Queue 1 Enable."]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue 0 Enable."]
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W<0> {
        RXQ0EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Receive Queue 1 Enable."]
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W<2> {
        RXQ1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl0](index.html) module"]
pub struct MAC_RXQ_CTRL0_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rxq_ctrl0::R](R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl0::W](W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_RXQ_CTRL0 to value 0"]
impl crate::Resettable for MAC_RXQ_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
