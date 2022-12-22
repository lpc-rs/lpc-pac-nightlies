#[doc = "Register `MPIN[%s]` reader"]
pub struct R(crate::R<MPIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPIN[%s]` writer"]
pub struct W(crate::W<MPIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPIN_SPEC>;
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
impl From<crate::W<MPIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPORTP` reader - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MPORTP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MPORTP` writer - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MPORTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPIN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&self) -> MPORTP_R {
        MPORTP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&mut self) -> MPORTP_W<0> {
        MPORTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Masked port register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpin](index.html) module"]
pub struct MPIN_SPEC;
impl crate::RegisterSpec for MPIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpin::R](R) reader structure"]
impl crate::Readable for MPIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpin::W](W) writer structure"]
impl crate::Writable for MPIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPIN[%s]
to value 0"]
impl crate::Resettable for MPIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
