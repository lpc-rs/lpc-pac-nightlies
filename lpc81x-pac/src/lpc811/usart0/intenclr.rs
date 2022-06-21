#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDYCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type RXRDYCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 0>;
#[doc = "Field `TXRDYCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type TXRDYCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 2>;
#[doc = "Field `DELTACTSCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type DELTACTSCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 5>;
#[doc = "Field `TXDISINTCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type TXDISINTCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 6>;
#[doc = "Field `OVERRUNCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type OVERRUNCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 8>;
#[doc = "Field `DELTARXBRKCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type DELTARXBRKCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 11>;
#[doc = "Field `STARTCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type STARTCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 12>;
#[doc = "Field `FRAMERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type FRAMERRCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 13>;
#[doc = "Field `PARITYERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type PARITYERRCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 14>;
#[doc = "Field `RXNOISECLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type RXNOISECLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 15>;
impl W {
    #[doc = "Bit 0 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxrdyclr(&mut self) -> RXRDYCLR_W {
        RXRDYCLR_W::new(self)
    }
    #[doc = "Bit 2 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txrdyclr(&mut self) -> TXRDYCLR_W {
        TXRDYCLR_W::new(self)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltactsclr(&mut self) -> DELTACTSCLR_W {
        DELTACTSCLR_W::new(self)
    }
    #[doc = "Bit 6 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txdisintclr(&mut self) -> TXDISINTCLR_W {
        TXDISINTCLR_W::new(self)
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn overrunclr(&mut self) -> OVERRUNCLR_W {
        OVERRUNCLR_W::new(self)
    }
    #[doc = "Bit 11 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltarxbrkclr(&mut self) -> DELTARXBRKCLR_W {
        DELTARXBRKCLR_W::new(self)
    }
    #[doc = "Bit 12 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn startclr(&mut self) -> STARTCLR_W {
        STARTCLR_W::new(self)
    }
    #[doc = "Bit 13 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn framerrclr(&mut self) -> FRAMERRCLR_W {
        FRAMERRCLR_W::new(self)
    }
    #[doc = "Bit 14 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn parityerrclr(&mut self) -> PARITYERRCLR_W {
        PARITYERRCLR_W::new(self)
    }
    #[doc = "Bit 15 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxnoiseclr(&mut self) -> RXNOISECLR_W {
        RXNOISECLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
