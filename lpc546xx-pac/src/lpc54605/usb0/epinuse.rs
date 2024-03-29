#[doc = "Register `EPINUSE` reader"]
pub struct R(crate::R<EPINUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINUSE` writer"]
pub struct W(crate::W<EPINUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINUSE_SPEC>;
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
impl From<crate::W<EPINUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF` reader - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
pub type BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUF` writer - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
pub type BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPINUSE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W<2> {
        BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Buffer in use\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinuse](index.html) module"]
pub struct EPINUSE_SPEC;
impl crate::RegisterSpec for EPINUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epinuse::R](R) reader structure"]
impl crate::Readable for EPINUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epinuse::W](W) writer structure"]
impl crate::Writable for EPINUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINUSE to value 0"]
impl crate::Resettable for EPINUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
