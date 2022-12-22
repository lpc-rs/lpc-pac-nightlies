#[doc = "Register `MAC_RXQ_CTRL2` reader"]
pub struct R(crate::R<MAC_RXQ_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RXQ_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RXQ_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_RXQ_CTRL2` writer"]
pub struct W(crate::W<MAC_RXQ_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL2_SPEC>;
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
impl From<crate::W<MAC_RXQ_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RXQ_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSRQ0` reader - Priorities Selected in the Receive Queue 0."]
pub type PSRQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ0` writer - Priorities Selected in the Receive Queue 0."]
pub type PSRQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSRQ1` reader - Priorities Selected in the Receive Queue 1."]
pub type PSRQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ1` writer - Priorities Selected in the Receive Queue 1."]
pub type PSRQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSRQ2` reader - Priorities Selected in the Receive Queue 2."]
pub type PSRQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ2` writer - Priorities Selected in the Receive Queue 2."]
pub type PSRQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSRQ3` reader - Priorities Selected in the Receive Queue 3."]
pub type PSRQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ3` writer - Priorities Selected in the Receive Queue 3."]
pub type PSRQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_RXQ_CTRL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&self) -> PSRQ2_R {
        PSRQ2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&self) -> PSRQ3_R {
        PSRQ3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W<0> {
        PSRQ0_W::new(self)
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W<8> {
        PSRQ1_W::new(self)
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&mut self) -> PSRQ2_W<16> {
        PSRQ2_W::new(self)
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&mut self) -> PSRQ3_W<24> {
        PSRQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl2](index.html) module"]
pub struct MAC_RXQ_CTRL2_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rxq_ctrl2::R](R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl2::W](W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_RXQ_CTRL2 to value 0"]
impl crate::Resettable for MAC_RXQ_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
