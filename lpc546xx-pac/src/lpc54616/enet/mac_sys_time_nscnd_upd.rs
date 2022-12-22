#[doc = "Register `MAC_SYS_TIME_NSCND_UPD` reader"]
pub struct R(crate::R<MAC_SYS_TIME_NSCND_UPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIME_NSCND_UPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIME_NSCND_UPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIME_NSCND_UPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_SYS_TIME_NSCND_UPD` writer"]
pub struct W(crate::W<MAC_SYS_TIME_NSCND_UPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SYS_TIME_NSCND_UPD_SPEC>;
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
impl From<crate::W<MAC_SYS_TIME_NSCND_UPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_SYS_TIME_NSCND_UPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSS` reader - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
pub type TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSSS` writer - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
pub type TSSS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_SYS_TIME_NSCND_UPD_SPEC, u32, u32, 31, O>;
#[doc = "Field `ADDSUB` reader - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
pub type ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUB` writer - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
pub type ADDSUB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAC_SYS_TIME_NSCND_UPD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<0> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 31 - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W<31> {
        ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_nscnd_upd](index.html) module"]
pub struct MAC_SYS_TIME_NSCND_UPD_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIME_NSCND_UPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_sys_time_nscnd_upd::R](R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_NSCND_UPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_sys_time_nscnd_upd::W](W) writer structure"]
impl crate::Writable for MAC_SYS_TIME_NSCND_UPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_SYS_TIME_NSCND_UPD to value 0"]
impl crate::Resettable for MAC_SYS_TIME_NSCND_UPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
