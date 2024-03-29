#[doc = "Register `DAT` reader"]
pub struct R(crate::R<DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAT` writer"]
pub struct W(crate::W<DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAT_SPEC>;
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
impl From<crate::W<DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Data` reader - This register holds data values that have been received or are to be transmitted."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data` writer - This register holds data values that have been received or are to be transmitted."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dat](index.html) module"]
pub struct DAT_SPEC;
impl crate::RegisterSpec for DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dat::R](R) reader structure"]
impl crate::Readable for DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dat::W](W) writer structure"]
impl crate::Writable for DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAT to value 0"]
impl crate::Resettable for DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
