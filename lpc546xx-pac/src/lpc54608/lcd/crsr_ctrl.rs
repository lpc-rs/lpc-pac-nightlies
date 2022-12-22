#[doc = "Register `CRSR_CTRL` reader"]
pub struct R(crate::R<CRSR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_CTRL` writer"]
pub struct W(crate::W<CRSR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_CTRL_SPEC>;
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
impl From<crate::W<CRSR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRON` reader - Cursor enable."]
pub type CRSRON_R = crate::BitReader<bool>;
#[doc = "Field `CRSRON` writer - Cursor enable."]
pub type CRSRON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRSR_CTRL_SPEC, bool, O>;
#[doc = "Field `CRSRNUM1_0` reader - Cursor image number."]
pub type CRSRNUM1_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRSRNUM1_0` writer - Cursor image number."]
pub type CRSRNUM1_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&self) -> CRSRON_R {
        CRSRON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&self) -> CRSRNUM1_0_R {
        CRSRNUM1_0_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&mut self) -> CRSRON_W<0> {
        CRSRON_W::new(self)
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&mut self) -> CRSRNUM1_0_W<4> {
        CRSRNUM1_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_ctrl](index.html) module"]
pub struct CRSR_CTRL_SPEC;
impl crate::RegisterSpec for CRSR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_ctrl::R](R) reader structure"]
impl crate::Readable for CRSR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_ctrl::W](W) writer structure"]
impl crate::Writable for CRSR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_CTRL to value 0"]
impl crate::Resettable for CRSR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
