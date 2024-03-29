#[doc = "Register `PSELID` reader"]
pub struct R(crate::R<PSELID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELID` writer"]
pub struct W(crate::W<PSELID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELID_SPEC>;
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
impl From<crate::W<PSELID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSEL` reader - Peripheral Select. This field is writable by software."]
pub type PERSEL_R = crate::FieldReader<u8, PERSEL_A>;
#[doc = "Peripheral Select. This field is writable by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERSEL_A {
    #[doc = "0: No peripheral selected."]
    NO_PERIPH_SELECTED = 0,
    #[doc = "1: USART function selected."]
    USART = 1,
    #[doc = "2: SPI function selected."]
    SPI = 2,
    #[doc = "3: I2C function selected."]
    I2C = 3,
    #[doc = "4: I2S transmit function selected."]
    I2S_TRANSMIT = 4,
    #[doc = "5: I2S receive function selected."]
    I2S_RECEIVE = 5,
}
impl From<PERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSEL_A) -> Self {
        variant as _
    }
}
impl PERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERSEL_A> {
        match self.bits {
            0 => Some(PERSEL_A::NO_PERIPH_SELECTED),
            1 => Some(PERSEL_A::USART),
            2 => Some(PERSEL_A::SPI),
            3 => Some(PERSEL_A::I2C),
            4 => Some(PERSEL_A::I2S_TRANSMIT),
            5 => Some(PERSEL_A::I2S_RECEIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PERIPH_SELECTED`"]
    #[inline(always)]
    pub fn is_no_periph_selected(&self) -> bool {
        *self == PERSEL_A::NO_PERIPH_SELECTED
    }
    #[doc = "Checks if the value of the field is `USART`"]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == PERSEL_A::USART
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == PERSEL_A::SPI
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == PERSEL_A::I2C
    }
    #[doc = "Checks if the value of the field is `I2S_TRANSMIT`"]
    #[inline(always)]
    pub fn is_i2s_transmit(&self) -> bool {
        *self == PERSEL_A::I2S_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S_RECEIVE`"]
    #[inline(always)]
    pub fn is_i2s_receive(&self) -> bool {
        *self == PERSEL_A::I2S_RECEIVE
    }
}
#[doc = "Field `PERSEL` writer - Peripheral Select. This field is writable by software."]
pub type PERSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSELID_SPEC, u8, PERSEL_A, 3, O>;
impl<'a, const O: u8> PERSEL_W<'a, O> {
    #[doc = "No peripheral selected."]
    #[inline(always)]
    pub fn no_periph_selected(self) -> &'a mut W {
        self.variant(PERSEL_A::NO_PERIPH_SELECTED)
    }
    #[doc = "USART function selected."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut W {
        self.variant(PERSEL_A::USART)
    }
    #[doc = "SPI function selected."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(PERSEL_A::SPI)
    }
    #[doc = "I2C function selected."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(PERSEL_A::I2C)
    }
    #[doc = "I2S transmit function selected."]
    #[inline(always)]
    pub fn i2s_transmit(self) -> &'a mut W {
        self.variant(PERSEL_A::I2S_TRANSMIT)
    }
    #[doc = "I2S receive function selected."]
    #[inline(always)]
    pub fn i2s_receive(self) -> &'a mut W {
        self.variant(PERSEL_A::I2S_RECEIVE)
    }
}
#[doc = "Field `LOCK` reader - Lock the peripheral select. This field is writable by software."]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock the peripheral select. This field is writable by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Peripheral select can be changed by software."]
    UNLOCKED = 0,
    #[doc = "1: Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCK` writer - Lock the peripheral select. This field is writable by software."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSELID_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "Peripheral select can be changed by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
}
#[doc = "Field `USARTPRESENT` reader - USART present indicator. This field is Read-only."]
pub type USARTPRESENT_R = crate::BitReader<USARTPRESENT_A>;
#[doc = "USART present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USARTPRESENT_A {
    #[doc = "0: This Flexcomm does not include the USART function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the USART function."]
    PRESENT = 1,
}
impl From<USARTPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: USARTPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl USARTPRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USARTPRESENT_A {
        match self.bits {
            false => USARTPRESENT_A::NOT_PRESENT,
            true => USARTPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USARTPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == USARTPRESENT_A::PRESENT
    }
}
#[doc = "Field `SPIPRESENT` reader - SPI present indicator. This field is Read-only."]
pub type SPIPRESENT_R = crate::BitReader<SPIPRESENT_A>;
#[doc = "SPI present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIPRESENT_A {
    #[doc = "0: This Flexcomm does not include the SPI function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the SPI function."]
    PRESENT = 1,
}
impl From<SPIPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: SPIPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIPRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIPRESENT_A {
        match self.bits {
            false => SPIPRESENT_A::NOT_PRESENT,
            true => SPIPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == SPIPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SPIPRESENT_A::PRESENT
    }
}
#[doc = "Field `I2CPRESENT` reader - I2C present indicator. This field is Read-only."]
pub type I2CPRESENT_R = crate::BitReader<I2CPRESENT_A>;
#[doc = "I2C present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CPRESENT_A {
    #[doc = "0: This Flexcomm does not include the I2C function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the I2C function."]
    PRESENT = 1,
}
impl From<I2CPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: I2CPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CPRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CPRESENT_A {
        match self.bits {
            false => I2CPRESENT_A::NOT_PRESENT,
            true => I2CPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2CPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2CPRESENT_A::PRESENT
    }
}
#[doc = "Field `I2SPRESENT` reader - I 2S present indicator. This field is Read-only."]
pub type I2SPRESENT_R = crate::BitReader<I2SPRESENT_A>;
#[doc = "I 2S present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SPRESENT_A {
    #[doc = "0: This Flexcomm does not include the I2S function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the I2S function."]
    PRESENT = 1,
}
impl From<I2SPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: I2SPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SPRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SPRESENT_A {
        match self.bits {
            false => I2SPRESENT_A::NOT_PRESENT,
            true => I2SPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2SPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2SPRESENT_A::PRESENT
    }
}
#[doc = "Field `ID` reader - Flexcomm ID."]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn usartpresent(&self) -> USARTPRESENT_R {
        USARTPRESENT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn spipresent(&self) -> SPIPRESENT_R {
        SPIPRESENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2cpresent(&self) -> I2CPRESENT_R {
        I2CPRESENT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2spresent(&self) -> I2SPRESENT_R {
        I2SPRESENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Flexcomm ID."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&mut self) -> PERSEL_W<0> {
        PERSEL_W::new(self)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<3> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Select and Flexcomm ID register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselid](index.html) module"]
pub struct PSELID_SPEC;
impl crate::RegisterSpec for PSELID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselid::R](R) reader structure"]
impl crate::Readable for PSELID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselid::W](W) writer structure"]
impl crate::Writable for PSELID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELID to value 0x0010_1000"]
impl crate::Resettable for PSELID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_1000
    }
}
