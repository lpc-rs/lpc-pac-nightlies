#[doc = "Register `CTYPE` reader"]
pub struct R(crate::R<CTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTYPE` writer"]
pub struct W(crate::W<CTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTYPE_SPEC>;
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
impl From<crate::W<CTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_WIDTH0` reader - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type CARD_WIDTH0_R = crate::BitReader<bool>;
#[doc = "Field `CARD_WIDTH0` writer - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type CARD_WIDTH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTYPE_SPEC, bool, O>;
#[doc = "Field `CARD_WIDTH1` reader - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type CARD_WIDTH1_R = crate::BitReader<bool>;
#[doc = "Field `CARD_WIDTH1` writer - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type CARD_WIDTH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTYPE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card_width0(&self) -> CARD_WIDTH0_R {
        CARD_WIDTH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card_width1(&self) -> CARD_WIDTH1_R {
        CARD_WIDTH1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card_width0(&mut self) -> CARD_WIDTH0_W<0> {
        CARD_WIDTH0_W::new(self)
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card_width1(&mut self) -> CARD_WIDTH1_W<16> {
        CARD_WIDTH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card Type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctype](index.html) module"]
pub struct CTYPE_SPEC;
impl crate::RegisterSpec for CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctype::R](R) reader structure"]
impl crate::Readable for CTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctype::W](W) writer structure"]
impl crate::Writable for CTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
