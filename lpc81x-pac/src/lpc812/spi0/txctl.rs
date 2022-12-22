#[doc = "Register `TXCTL` reader"]
pub struct R(crate::R<TXCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTL` writer"]
pub struct W(crate::W<TXCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTL_SPEC>;
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
impl From<crate::W<TXCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSSEL0_N` reader - Transmit Slave Select 0."]
pub type TXSSEL0_N_R = crate::BitReader<bool>;
#[doc = "Field `TXSSEL0_N` writer - Transmit Slave Select 0."]
pub type TXSSEL0_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTL_SPEC, bool, O>;
#[doc = "Field `EOT` reader - End of Transfer."]
pub type EOT_R = crate::BitReader<bool>;
#[doc = "Field `EOT` writer - End of Transfer."]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTL_SPEC, bool, O>;
#[doc = "Field `EOF` reader - End of Frame."]
pub type EOF_R = crate::BitReader<bool>;
#[doc = "Field `EOF` writer - End of Frame."]
pub type EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTL_SPEC, bool, O>;
#[doc = "Field `RXIGNORE` reader - Receive Ignore."]
pub type RXIGNORE_R = crate::BitReader<bool>;
#[doc = "Field `RXIGNORE` writer - Receive Ignore."]
pub type RXIGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTL_SPEC, bool, O>;
#[doc = "Field `LEN` reader - Data transfer Length."]
pub type LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN` writer - Data transfer Length."]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&self) -> TXSSEL0_N_R {
        TXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&self) -> RXIGNORE_R {
        RXIGNORE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W<16> {
        TXSSEL0_N_W::new(self)
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W<20> {
        EOT_W::new(self)
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W<21> {
        EOF_W::new(self)
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W<22> {
        RXIGNORE_W::new(self)
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<24> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctl](index.html) module"]
pub struct TXCTL_SPEC;
impl crate::RegisterSpec for TXCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctl::R](R) reader structure"]
impl crate::Readable for TXCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctl::W](W) writer structure"]
impl crate::Writable for TXCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTL to value 0"]
impl crate::Resettable for TXCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
