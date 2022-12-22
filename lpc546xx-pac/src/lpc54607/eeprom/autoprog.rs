#[doc = "Register `AUTOPROG` reader"]
pub struct R(crate::R<AUTOPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOPROG` writer"]
pub struct W(crate::W<AUTOPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOPROG_SPEC>;
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
impl From<crate::W<AUTOPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOPROG` reader - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
pub type AUTOPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTOPROG` writer - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
pub type AUTOPROG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOPROG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
    #[inline(always)]
    pub fn autoprog(&self) -> AUTOPROG_R {
        AUTOPROG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
    #[inline(always)]
    pub fn autoprog(&mut self) -> AUTOPROG_W<0> {
        AUTOPROG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM auto programming register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoprog](index.html) module"]
pub struct AUTOPROG_SPEC;
impl crate::RegisterSpec for AUTOPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autoprog::R](R) reader structure"]
impl crate::Readable for AUTOPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autoprog::W](W) writer structure"]
impl crate::Writable for AUTOPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOPROG to value 0"]
impl crate::Resettable for AUTOPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
