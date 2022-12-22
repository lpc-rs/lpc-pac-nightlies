#[doc = "Register `FROCTRL` reader"]
pub struct R(crate::R<FROCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROCTRL` writer"]
pub struct W(crate::W<FROCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROCTRL_SPEC>;
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
impl From<crate::W<FROCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FROCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - This value is factory trimmed to account for bias and temperature compensation."]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - This value is factory trimmed to account for bias and temperature compensation."]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FROCTRL_SPEC, u16, u16, 14, O>;
#[doc = "Field `SEL` reader - Select the FRO HF output frequency."]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `SEL` writer - Select the FRO HF output frequency."]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROCTRL_SPEC, bool, O>;
#[doc = "Field `FREQTRIM` reader - Frequency trim."]
pub type FREQTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQTRIM` writer - Frequency trim."]
pub type FREQTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FROCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `USBCLKADJ` reader - USB clock adjust mode."]
pub type USBCLKADJ_R = crate::BitReader<bool>;
#[doc = "Field `USBCLKADJ` writer - USB clock adjust mode."]
pub type USBCLKADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROCTRL_SPEC, bool, O>;
#[doc = "Field `USBMODCHG` reader - USB Mode value Change flag."]
pub type USBMODCHG_R = crate::BitReader<bool>;
#[doc = "Field `USBMODCHG` writer - USB Mode value Change flag."]
pub type USBMODCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROCTRL_SPEC, bool, O>;
#[doc = "Field `HSPDCLK` reader - High speed clock enable."]
pub type HSPDCLK_R = crate::BitReader<bool>;
#[doc = "Field `HSPDCLK` writer - High speed clock enable."]
pub type HSPDCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROCTRL_SPEC, bool, O>;
#[doc = "Field `WRTRIM` reader - Write Trim value."]
pub type WRTRIM_R = crate::BitReader<bool>;
#[doc = "Field `WRTRIM` writer - Write Trim value."]
pub type WRTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&self) -> FREQTRIM_R {
        FREQTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&self) -> HSPDCLK_R {
        HSPDCLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&self) -> WRTRIM_R {
        WRTRIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<14> {
        SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&mut self) -> FREQTRIM_W<16> {
        FREQTRIM_W::new(self)
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W<24> {
        USBCLKADJ_W::new(self)
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&mut self) -> USBMODCHG_W<25> {
        USBMODCHG_W::new(self)
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&mut self) -> HSPDCLK_W<30> {
        HSPDCLK_W::new(self)
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WRTRIM_W<31> {
        WRTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froctrl](index.html) module"]
pub struct FROCTRL_SPEC;
impl crate::RegisterSpec for FROCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [froctrl::R](R) reader structure"]
impl crate::Readable for FROCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [froctrl::W](W) writer structure"]
impl crate::Writable for FROCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FROCTRL to value 0x4000"]
impl crate::Resettable for FROCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
