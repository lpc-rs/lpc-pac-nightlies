#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBRIE` reader - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
pub type RBRIE_R = crate::BitReader<RBRIE_A>;
#[doc = "Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRIE_A {
    #[doc = "0: Disable the RDA interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the RDA interrupt."]
    ENABLE = 1,
}
impl From<RBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RBRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRIE_A {
        match self.bits {
            false => RBRIE_A::DISABLE,
            true => RBRIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RBRIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RBRIE_A::ENABLE
    }
}
#[doc = "Field `RBRIE` writer - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
pub type RBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RBRIE_A, O>;
impl<'a, const O: u8> RBRIE_W<'a, O> {
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBRIE_A::DISABLE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBRIE_A::ENABLE)
    }
}
#[doc = "Field `THREIE` reader - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREIE_R = crate::BitReader<THREIE_A>;
#[doc = "Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIE_A {
    #[doc = "0: Disable the THRE interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the THRE interrupt."]
    ENABLE = 1,
}
impl From<THREIE_A> for bool {
    #[inline(always)]
    fn from(variant: THREIE_A) -> Self {
        variant as u8 != 0
    }
}
impl THREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREIE_A {
        match self.bits {
            false => THREIE_A::DISABLE,
            true => THREIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THREIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THREIE_A::ENABLE
    }
}
#[doc = "Field `THREIE` writer - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, THREIE_A, O>;
impl<'a, const O: u8> THREIE_W<'a, O> {
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(THREIE_A::DISABLE)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(THREIE_A::ENABLE)
    }
}
#[doc = "Field `RXLIE` reader - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RXLIE_R = crate::BitReader<RXLIE_A>;
#[doc = "Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLIE_A {
    #[doc = "0: Disable the RX line status interrupts."]
    DISABLE = 0,
    #[doc = "1: Enable the RX line status interrupts."]
    ENABLE = 1,
}
impl From<RXLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLIE_A {
        match self.bits {
            false => RXLIE_A::DISABLE,
            true => RXLIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXLIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXLIE_A::ENABLE
    }
}
#[doc = "Field `RXLIE` writer - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RXLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RXLIE_A, O>;
impl<'a, const O: u8> RXLIE_W<'a, O> {
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXLIE_A::DISABLE)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXLIE_A::ENABLE)
    }
}
#[doc = "Field `ABEOINTEN` reader - Enables the end of auto-baud interrupt."]
pub type ABEOINTEN_R = crate::BitReader<ABEOINTEN_A>;
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTEN_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLE = 1,
}
impl From<ABEOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABEOINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTEN_A {
        match self.bits {
            false => ABEOINTEN_A::DISABLE,
            true => ABEOINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ABEOINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABEOINTEN_A::ENABLE
    }
}
#[doc = "Field `ABEOINTEN` writer - Enables the end of auto-baud interrupt."]
pub type ABEOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ABEOINTEN_A, O>;
impl<'a, const O: u8> ABEOINTEN_W<'a, O> {
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::DISABLE)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::ENABLE)
    }
}
#[doc = "Field `ABTOINTEN` reader - Enables the auto-baud time-out interrupt."]
pub type ABTOINTEN_R = crate::BitReader<ABTOINTEN_A>;
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTEN_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLE = 1,
}
impl From<ABTOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABTOINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTEN_A {
        match self.bits {
            false => ABTOINTEN_A::DISABLE,
            true => ABTOINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ABTOINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABTOINTEN_A::ENABLE
    }
}
#[doc = "Field `ABTOINTEN` writer - Enables the auto-baud time-out interrupt."]
pub type ABTOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ABTOINTEN_A, O>;
impl<'a, const O: u8> ABTOINTEN_W<'a, O> {
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::DISABLE)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxlie(&self) -> RXLIE_R {
        RXLIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W<0> {
        RBRIE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&mut self) -> THREIE_W<1> {
        THREIE_W::new(self)
    }
    #[doc = "Bit 2 - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxlie(&mut self) -> RXLIE_W<2> {
        RXLIE_W::new(self)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> ABEOINTEN_W<8> {
        ABEOINTEN_W::new(self)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> ABTOINTEN_W<9> {
        ABTOINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts. When DLAB=0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
