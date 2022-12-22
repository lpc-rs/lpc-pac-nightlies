#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDYEN` reader - Determines whether an interrupt occurs when receiver data is available."]
pub type RXRDYEN_R = crate::BitReader<RXRDYEN_A>;
#[doc = "Determines whether an interrupt occurs when receiver data is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYEN_A {
    #[doc = "0: No interrupt will be generated when receiver data is available."]
    RXRDYEN_0 = 0,
    #[doc = "1: An interrupt will be generated when receiver data is available in the RXDAT register."]
    RXRDYEN_1 = 1,
}
impl From<RXRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRDYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDYEN_A {
        match self.bits {
            false => RXRDYEN_A::RXRDYEN_0,
            true => RXRDYEN_A::RXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_0`"]
    #[inline(always)]
    pub fn is_rxrdyen_0(&self) -> bool {
        *self == RXRDYEN_A::RXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_1`"]
    #[inline(always)]
    pub fn is_rxrdyen_1(&self) -> bool {
        *self == RXRDYEN_A::RXRDYEN_1
    }
}
#[doc = "Field `RXRDYEN` writer - Determines whether an interrupt occurs when receiver data is available."]
pub type RXRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXRDYEN_A, O>;
impl<'a, const O: u8> RXRDYEN_W<'a, O> {
    #[doc = "No interrupt will be generated when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen_0(self) -> &'a mut W {
        self.variant(RXRDYEN_A::RXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen_1(self) -> &'a mut W {
        self.variant(RXRDYEN_A::RXRDYEN_1)
    }
}
#[doc = "Field `TXRDYEN` reader - Determines whether an interrupt occurs when the transmitter holding register is available."]
pub type TXRDYEN_R = crate::BitReader<TXRDYEN_A>;
#[doc = "Determines whether an interrupt occurs when the transmitter holding register is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYEN_A {
    #[doc = "0: No interrupt will be generated when the transmitter holding register is available."]
    TXRDYEN_0 = 0,
    #[doc = "1: An interrupt will be generated when data may be written to TXDAT."]
    TXRDYEN_1 = 1,
}
impl From<TXRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRDYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDYEN_A {
        match self.bits {
            false => TXRDYEN_A::TXRDYEN_0,
            true => TXRDYEN_A::TXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_0`"]
    #[inline(always)]
    pub fn is_txrdyen_0(&self) -> bool {
        *self == TXRDYEN_A::TXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_1`"]
    #[inline(always)]
    pub fn is_txrdyen_1(&self) -> bool {
        *self == TXRDYEN_A::TXRDYEN_1
    }
}
#[doc = "Field `TXRDYEN` writer - Determines whether an interrupt occurs when the transmitter holding register is available."]
pub type TXRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXRDYEN_A, O>;
impl<'a, const O: u8> TXRDYEN_W<'a, O> {
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen_0(self) -> &'a mut W {
        self.variant(TXRDYEN_A::TXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    #[inline(always)]
    pub fn txrdyen_1(self) -> &'a mut W {
        self.variant(TXRDYEN_A::TXRDYEN_1)
    }
}
#[doc = "Field `RXOVEN` reader - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
pub type RXOVEN_R = crate::BitReader<RXOVEN_A>;
#[doc = "Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVEN_A {
    #[doc = "0: No interrupt will be generated when a receiver overrun occurs."]
    RXOVEN_0 = 0,
    #[doc = "1: An interrupt will be generated if a receiver overrun occurs."]
    RXOVEN_1 = 1,
}
impl From<RXOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVEN_A {
        match self.bits {
            false => RXOVEN_A::RXOVEN_0,
            true => RXOVEN_A::RXOVEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXOVEN_0`"]
    #[inline(always)]
    pub fn is_rxoven_0(&self) -> bool {
        *self == RXOVEN_A::RXOVEN_0
    }
    #[doc = "Checks if the value of the field is `RXOVEN_1`"]
    #[inline(always)]
    pub fn is_rxoven_1(&self) -> bool {
        *self == RXOVEN_A::RXOVEN_1
    }
}
#[doc = "Field `RXOVEN` writer - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
pub type RXOVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXOVEN_A, O>;
impl<'a, const O: u8> RXOVEN_W<'a, O> {
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    #[inline(always)]
    pub fn rxoven_0(self) -> &'a mut W {
        self.variant(RXOVEN_A::RXOVEN_0)
    }
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    #[inline(always)]
    pub fn rxoven_1(self) -> &'a mut W {
        self.variant(RXOVEN_A::RXOVEN_1)
    }
}
#[doc = "Field `TXUREN` reader - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
pub type TXUREN_R = crate::BitReader<TXUREN_A>;
#[doc = "Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUREN_A {
    #[doc = "0: No interrupt will be generated when the transmitter underruns."]
    TXUREN_0 = 0,
    #[doc = "1: An interrupt will be generated if the transmitter underruns."]
    TXUREN_1 = 1,
}
impl From<TXUREN_A> for bool {
    #[inline(always)]
    fn from(variant: TXUREN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUREN_A {
        match self.bits {
            false => TXUREN_A::TXUREN_0,
            true => TXUREN_A::TXUREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXUREN_0`"]
    #[inline(always)]
    pub fn is_txuren_0(&self) -> bool {
        *self == TXUREN_A::TXUREN_0
    }
    #[doc = "Checks if the value of the field is `TXUREN_1`"]
    #[inline(always)]
    pub fn is_txuren_1(&self) -> bool {
        *self == TXUREN_A::TXUREN_1
    }
}
#[doc = "Field `TXUREN` writer - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
pub type TXUREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXUREN_A, O>;
impl<'a, const O: u8> TXUREN_W<'a, O> {
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    #[inline(always)]
    pub fn txuren_0(self) -> &'a mut W {
        self.variant(TXUREN_A::TXUREN_0)
    }
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    #[inline(always)]
    pub fn txuren_1(self) -> &'a mut W {
        self.variant(TXUREN_A::TXUREN_1)
    }
}
#[doc = "Field `SSAEN` reader - Determines whether an interrupt occurs when the Slave Select is asserted."]
pub type SSAEN_R = crate::BitReader<SSAEN_A>;
#[doc = "Determines whether an interrupt occurs when the Slave Select is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAEN_A {
    #[doc = "0: No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_0 = 0,
    #[doc = "1: An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_1 = 1,
}
impl From<SSAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SSAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSAEN_A {
        match self.bits {
            false => SSAEN_A::SSAEN_0,
            true => SSAEN_A::SSAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSAEN_0`"]
    #[inline(always)]
    pub fn is_ssaen_0(&self) -> bool {
        *self == SSAEN_A::SSAEN_0
    }
    #[doc = "Checks if the value of the field is `SSAEN_1`"]
    #[inline(always)]
    pub fn is_ssaen_1(&self) -> bool {
        *self == SSAEN_A::SSAEN_1
    }
}
#[doc = "Field `SSAEN` writer - Determines whether an interrupt occurs when the Slave Select is asserted."]
pub type SSAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, SSAEN_A, O>;
impl<'a, const O: u8> SSAEN_W<'a, O> {
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn ssaen_0(self) -> &'a mut W {
        self.variant(SSAEN_A::SSAEN_0)
    }
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn ssaen_1(self) -> &'a mut W {
        self.variant(SSAEN_A::SSAEN_1)
    }
}
#[doc = "Field `SSDEN` reader - Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub type SSDEN_R = crate::BitReader<SSDEN_A>;
#[doc = "Determines whether an interrupt occurs when the Slave Select is deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDEN_A {
    #[doc = "0: No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_0 = 0,
    #[doc = "1: An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_1 = 1,
}
impl From<SSDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SSDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSDEN_A {
        match self.bits {
            false => SSDEN_A::SSDEN_0,
            true => SSDEN_A::SSDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSDEN_0`"]
    #[inline(always)]
    pub fn is_ssden_0(&self) -> bool {
        *self == SSDEN_A::SSDEN_0
    }
    #[doc = "Checks if the value of the field is `SSDEN_1`"]
    #[inline(always)]
    pub fn is_ssden_1(&self) -> bool {
        *self == SSDEN_A::SSDEN_1
    }
}
#[doc = "Field `SSDEN` writer - Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub type SSDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, SSDEN_A, O>;
impl<'a, const O: u8> SSDEN_W<'a, O> {
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn ssden_0(self) -> &'a mut W {
        self.variant(SSDEN_A::SSDEN_0)
    }
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn ssden_1(self) -> &'a mut W {
        self.variant(SSDEN_A::SSDEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen(&self) -> RXRDYEN_R {
        RXRDYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen(&self) -> TXRDYEN_R {
        TXRDYEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline(always)]
    pub fn rxoven(&self) -> RXOVEN_R {
        RXOVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline(always)]
    pub fn txuren(&self) -> TXUREN_R {
        TXUREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&self) -> SSAEN_R {
        SSAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&self) -> SSDEN_R {
        SSDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen(&mut self) -> RXRDYEN_W<0> {
        RXRDYEN_W::new(self)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen(&mut self) -> TXRDYEN_W<1> {
        TXRDYEN_W::new(self)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline(always)]
    pub fn rxoven(&mut self) -> RXOVEN_W<2> {
        RXOVEN_W::new(self)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline(always)]
    pub fn txuren(&mut self) -> TXUREN_W<3> {
        TXUREN_W::new(self)
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&mut self) -> SSAEN_W<4> {
        SSAEN_W::new(self)
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&mut self) -> SSDEN_W<5> {
        SSDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
