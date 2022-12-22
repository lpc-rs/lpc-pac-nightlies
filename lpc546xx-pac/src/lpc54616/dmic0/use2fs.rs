#[doc = "Register `USE2FS` reader"]
pub struct R(crate::R<USE2FS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USE2FS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USE2FS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USE2FS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USE2FS` writer"]
pub struct W(crate::W<USE2FS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USE2FS_SPEC>;
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
impl From<crate::W<USE2FS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USE2FS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USE2FS` reader - Use 2FS register"]
pub type USE2FS_R = crate::BitReader<USE2FS_A>;
#[doc = "Use 2FS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE2FS_A {
    #[doc = "0: Use 1FS output for PCM data."]
    USE_1FS = 0,
    #[doc = "1: Use 2FS output for PCM data."]
    USE_2FS = 1,
}
impl From<USE2FS_A> for bool {
    #[inline(always)]
    fn from(variant: USE2FS_A) -> Self {
        variant as u8 != 0
    }
}
impl USE2FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USE2FS_A {
        match self.bits {
            false => USE2FS_A::USE_1FS,
            true => USE2FS_A::USE_2FS,
        }
    }
    #[doc = "Checks if the value of the field is `USE_1FS`"]
    #[inline(always)]
    pub fn is_use_1fs(&self) -> bool {
        *self == USE2FS_A::USE_1FS
    }
    #[doc = "Checks if the value of the field is `USE_2FS`"]
    #[inline(always)]
    pub fn is_use_2fs(&self) -> bool {
        *self == USE2FS_A::USE_2FS
    }
}
#[doc = "Field `USE2FS` writer - Use 2FS register"]
pub type USE2FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USE2FS_SPEC, USE2FS_A, O>;
impl<'a, const O: u8> USE2FS_W<'a, O> {
    #[doc = "Use 1FS output for PCM data."]
    #[inline(always)]
    pub fn use_1fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_1FS)
    }
    #[doc = "Use 2FS output for PCM data."]
    #[inline(always)]
    pub fn use_2fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_2FS)
    }
}
impl R {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&self) -> USE2FS_R {
        USE2FS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&mut self) -> USE2FS_W<0> {
        USE2FS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use 2FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [use2fs](index.html) module"]
pub struct USE2FS_SPEC;
impl crate::RegisterSpec for USE2FS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [use2fs::R](R) reader structure"]
impl crate::Readable for USE2FS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [use2fs::W](W) writer structure"]
impl crate::Writable for USE2FS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USE2FS to value 0"]
impl crate::Resettable for USE2FS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
