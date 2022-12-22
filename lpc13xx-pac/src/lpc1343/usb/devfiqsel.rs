#[doc = "Register `DEVFIQSEL` writer"]
pub struct W(crate::W<DEVFIQSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVFIQSEL_SPEC>;
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
impl From<crate::W<DEVFIQSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVFIQSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "This interrupt comes from a 1 KHz free running clock resynchronized on the incoming SoF tokens. This is to be used for isochronous packet transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_AW {
    #[doc = "0: FRAME interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: FRAME interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - This interrupt comes from a 1 KHz free running clock resynchronized on the incoming SoF tokens. This is to be used for isochronous packet transfer."]
pub type FRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVFIQSEL_SPEC, FRAME_AW, O>;
impl<'a, const O: u8> FRAME_W<'a, O> {
    #[doc = "FRAME interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(FRAME_AW::LOWPRIORITY)
    }
    #[doc = "FRAME interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(FRAME_AW::HIGHPRIORITY)
    }
}
#[doc = "Interrupt routing for bulk out endpoints For logical endpoint 3 (physical endpoints 6 and 7) only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BULKOUT_AW {
    #[doc = "0: BULKOUT interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: BULKOUT interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<BULKOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: BULKOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BULKOUT` writer - Interrupt routing for bulk out endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
pub type BULKOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVFIQSEL_SPEC, BULKOUT_AW, O>;
impl<'a, const O: u8> BULKOUT_W<'a, O> {
    #[doc = "BULKOUT interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(BULKOUT_AW::LOWPRIORITY)
    }
    #[doc = "BULKOUT interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(BULKOUT_AW::HIGHPRIORITY)
    }
}
#[doc = "Interrupt routing for bulk in endpoints For logical endpoint 3 (physical endpoints 6 and 7) only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BULKIN_AW {
    #[doc = "0: BULKIN interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: BULKIN interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<BULKIN_AW> for bool {
    #[inline(always)]
    fn from(variant: BULKIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BULKIN` writer - Interrupt routing for bulk in endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
pub type BULKIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVFIQSEL_SPEC, BULKIN_AW, O>;
impl<'a, const O: u8> BULKIN_W<'a, O> {
    #[doc = "BULKIN interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(BULKIN_AW::LOWPRIORITY)
    }
    #[doc = "BULKIN interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(BULKIN_AW::HIGHPRIORITY)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt comes from a 1 KHz free running clock resynchronized on the incoming SoF tokens. This is to be used for isochronous packet transfer."]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<0> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt routing for bulk out endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
    #[inline(always)]
    pub fn bulkout(&mut self) -> BULKOUT_W<1> {
        BULKOUT_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt routing for bulk in endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
    #[inline(always)]
    pub fn bulkin(&mut self) -> BULKIN_W<2> {
        BULKIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device FIQ select\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devfiqsel](index.html) module"]
pub struct DEVFIQSEL_SPEC;
impl crate::RegisterSpec for DEVFIQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [devfiqsel::W](W) writer structure"]
impl crate::Writable for DEVFIQSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVFIQSEL to value 0"]
impl crate::Resettable for DEVFIQSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
