#[doc = "Register `FIFO_CTRL` reader"]
pub struct R(crate::R<FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CTRL` writer"]
pub struct W(crate::W<FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CTRL_SPEC>;
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
impl From<crate::W<FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - FIFO enable."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "FIFO enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    DISABLED = 0,
    #[doc = "1: FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - FIFO enable."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Field `RESETN` reader - FIFO reset."]
pub type RESETN_R = crate::BitReader<RESETN_A>;
#[doc = "FIFO reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETN_A {
    #[doc = "0: Reset the FIFO."]
    RESET = 0,
    #[doc = "1: Normal operation"]
    NORMAL = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::RESET,
            true => RESETN_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETN_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RESETN_A::NORMAL
    }
}
#[doc = "Field `RESETN` writer - FIFO reset."]
pub type RESETN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, RESETN_A, O>;
impl<'a, const O: u8> RESETN_W<'a, O> {
    #[doc = "Reset the FIFO."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETN_A::RESET)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESETN_A::NORMAL)
    }
}
#[doc = "Field `INTEN` reader - Interrupt enable."]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "Interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: FIFO level interrupts are not enabled."]
    DISABLED = 0,
    #[doc = "1: FIFO level interrupts are enabled."]
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN_A::ENABLED
    }
}
#[doc = "Field `INTEN` writer - Interrupt enable."]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA requests are not enabled."]
    DISABLED = 0,
    #[doc = "1: DMA requests based on FIFO level are enabled."]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
    }
}
#[doc = "Field `TRIGLVL` reader - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
pub type TRIGLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGLVL` writer - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
pub type TRIGLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W<1> {
        RESETN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W<2> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TRIGLVL_W<16> {
        TRIGLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](index.html) module"]
pub struct FIFO_CTRL_SPEC;
impl crate::RegisterSpec for FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_ctrl::R](R) reader structure"]
impl crate::Readable for FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](W) writer structure"]
impl crate::Writable for FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CTRL to value 0"]
impl crate::Resettable for FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
