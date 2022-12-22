#[doc = "Register `PINASSIGN6` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN6` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIN_1_I` reader - CTIN_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_1_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTIN_1_I` writer - CTIN_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_1_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTIN_2_I` reader - CTIN_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_2_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTIN_2_I` writer - CTIN_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_2_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTIN_3_I` reader - CTIN_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_3_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTIN_3_I` writer - CTIN_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_3_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTOUT_0_O` reader - CTOUT_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_0_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTOUT_0_O` writer - CTOUT_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_0_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTIN_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_1_i(&self) -> CTIN_1_I_R {
        CTIN_1_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CTIN_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_2_i(&self) -> CTIN_2_I_R {
        CTIN_2_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CTIN_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_3_i(&self) -> CTIN_3_I_R {
        CTIN_3_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CTOUT_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_0_o(&self) -> CTOUT_0_O_R {
        CTOUT_0_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTIN_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_1_i(&mut self) -> CTIN_1_I_W<0> {
        CTIN_1_I_W::new(self)
    }
    #[doc = "Bits 8:15 - CTIN_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_2_i(&mut self) -> CTIN_2_I_W<8> {
        CTIN_2_I_W::new(self)
    }
    #[doc = "Bits 16:23 - CTIN_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_3_i(&mut self) -> CTIN_3_I_W<16> {
        CTIN_3_I_W::new(self)
    }
    #[doc = "Bits 24:31 - CTOUT_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_0_o(&mut self) -> CTOUT_0_O_W<24> {
        CTOUT_0_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3,CTOUT_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign6](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign6::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign6::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN6 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
