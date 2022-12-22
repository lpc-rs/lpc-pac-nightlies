#[doc = "Register `MAC_VERSION` reader"]
pub struct R(crate::R<MAC_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_VERSION` writer"]
pub struct W(crate::W<MAC_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_VERSION_SPEC>;
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
impl From<crate::W<MAC_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNPVER` reader - NXP defined version."]
pub type SNPVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNPVER` writer - NXP defined version."]
pub type SNPVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_VERSION_SPEC, u8, u8, 8, O>;
#[doc = "Field `USERVER` reader - User defined version."]
pub type USERVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USERVER` writer - User defined version."]
pub type USERVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_VERSION_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NXP defined version."]
    #[inline(always)]
    pub fn snpver(&self) -> SNPVER_R {
        SNPVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User defined version."]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NXP defined version."]
    #[inline(always)]
    pub fn snpver(&mut self) -> SNPVER_W<0> {
        SNPVER_W::new(self)
    }
    #[doc = "Bits 8:15 - User defined version."]
    #[inline(always)]
    pub fn userver(&mut self) -> USERVER_W<8> {
        USERVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC version register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_version](index.html) module"]
pub struct MAC_VERSION_SPEC;
impl crate::RegisterSpec for MAC_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_version::R](R) reader structure"]
impl crate::Readable for MAC_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_version::W](W) writer structure"]
impl crate::Writable for MAC_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_VERSION to value 0"]
impl crate::Resettable for MAC_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
