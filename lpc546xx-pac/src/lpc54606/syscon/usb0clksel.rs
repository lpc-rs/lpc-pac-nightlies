#[doc = "Register `USB0CLKSEL` reader"]
pub struct R(crate::R<USB0CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0CLKSEL` writer"]
pub struct W(crate::W<USB0CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0CLKSEL_SPEC>;
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
impl From<crate::W<USB0CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - USB0 device clock source selection."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "USB0 device clock source selection.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 0,
    #[doc = "1: System PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT = 1,
    #[doc = "2: USB PLL clock (usb_pll_clk)"]
    USB_PLL_CLOCK = 2,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::FRO_HF),
            1 => Some(SEL_A::SYSTEM_PLL_OUTPUT),
            2 => Some(SEL_A::USB_PLL_CLOCK),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `USB_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_usb_pll_clock(&self) -> bool {
        *self == SEL_A::USB_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - USB0 device clock source selection."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB0CLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "System PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "USB PLL clock (usb_pll_clk)"]
    #[inline(always)]
    pub fn usb_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::USB_PLL_CLOCK)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB0 device clock source selection."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB0 device clock source selection."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB0 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clksel](index.html) module"]
pub struct USB0CLKSEL_SPEC;
impl crate::RegisterSpec for USB0CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0clksel::R](R) reader structure"]
impl crate::Readable for USB0CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0clksel::W](W) writer structure"]
impl crate::Writable for USB0CLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB0CLKSEL to value 0x07"]
impl crate::Resettable for USB0CLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
