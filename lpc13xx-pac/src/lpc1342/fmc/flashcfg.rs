#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl From<crate::W<FLASHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub type FLASHTIM_R = crate::FieldReader<u8, FLASHTIM_A>;
#[doc = "Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    _1_SYSTEM_CLOCK_FLASH = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    _2_SYSTEM_CLOCKS_FLAS = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock frequencies of up to 72 MHz)."]
    _3_SYSTEM_CLOCKS_FLAS = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
impl FLASHTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHTIM_A {
        match self.bits {
            0 => FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH,
            1 => FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS,
            2 => FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS,
            3 => FLASHTIM_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SYSTEM_CLOCK_FLASH`"]
    #[inline(always)]
    pub fn is_1_system_clock_flash(&self) -> bool {
        *self == FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH
    }
    #[doc = "Checks if the value of the field is `_2_SYSTEM_CLOCKS_FLAS`"]
    #[inline(always)]
    pub fn is_2_system_clocks_flas(&self) -> bool {
        *self == FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS
    }
    #[doc = "Checks if the value of the field is `_3_SYSTEM_CLOCKS_FLAS`"]
    #[inline(always)]
    pub fn is_3_system_clocks_flas(&self) -> bool {
        *self == FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == FLASHTIM_A::RESERVED_
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub type FLASHTIM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLASHCFG_SPEC, u8, FLASHTIM_A, 2, O>;
impl<'a, const O: u8> FLASHTIM_W<'a, O> {
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    #[inline(always)]
    pub fn _1_system_clock_flash(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH)
    }
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    #[inline(always)]
    pub fn _2_system_clocks_flas(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS)
    }
    #[doc = "3 system clocks flash access time (for system clock frequencies of up to 72 MHz)."]
    #[inline(always)]
    pub fn _3_system_clocks_flas(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(FLASHTIM_A::RESERVED_)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W<0> {
        FLASHTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
