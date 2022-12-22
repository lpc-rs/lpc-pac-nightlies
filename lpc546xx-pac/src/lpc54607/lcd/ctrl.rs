#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDEN` reader - LCD enable control bit."]
pub type LCDEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDEN` writer - LCD enable control bit."]
pub type LCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDBPP` reader - LCD bits per pixel."]
pub type LCDBPP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDBPP` writer - LCD bits per pixel."]
pub type LCDBPP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LCDBW` reader - STN LCD monochrome/color selection."]
pub type LCDBW_R = crate::BitReader<bool>;
#[doc = "Field `LCDBW` writer - STN LCD monochrome/color selection."]
pub type LCDBW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDTFT` reader - LCD panel TFT type selection."]
pub type LCDTFT_R = crate::BitReader<bool>;
#[doc = "Field `LCDTFT` writer - LCD panel TFT type selection."]
pub type LCDTFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDMONO8` reader - Monochrome LCD interface width."]
pub type LCDMONO8_R = crate::BitReader<bool>;
#[doc = "Field `LCDMONO8` writer - Monochrome LCD interface width."]
pub type LCDMONO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDDUAL` reader - Single or Dual LCD panel selection."]
pub type LCDDUAL_R = crate::BitReader<bool>;
#[doc = "Field `LCDDUAL` writer - Single or Dual LCD panel selection."]
pub type LCDDUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BGR` reader - Color format selection."]
pub type BGR_R = crate::BitReader<bool>;
#[doc = "Field `BGR` writer - Color format selection."]
pub type BGR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BEBO` reader - Big-endian Byte Order."]
pub type BEBO_R = crate::BitReader<bool>;
#[doc = "Field `BEBO` writer - Big-endian Byte Order."]
pub type BEBO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BEPO` reader - Big-Endian Pixel Ordering."]
pub type BEPO_R = crate::BitReader<bool>;
#[doc = "Field `BEPO` writer - Big-Endian Pixel Ordering."]
pub type BEPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDPWR` reader - LCD power enable."]
pub type LCDPWR_R = crate::BitReader<bool>;
#[doc = "Field `LCDPWR` writer - LCD power enable."]
pub type LCDPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LCDVCOMP` reader - LCD Vertical Compare Interrupt."]
pub type LCDVCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDVCOMP` writer - LCD Vertical Compare Interrupt."]
pub type LCDVCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WATERMARK` reader - LCD DMA FIFO watermark level."]
pub type WATERMARK_R = crate::BitReader<bool>;
#[doc = "Field `WATERMARK` writer - LCD DMA FIFO watermark level."]
pub type WATERMARK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&self) -> LCDBPP_R {
        LCDBPP_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&self) -> LCDBW_R {
        LCDBW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&self) -> LCDTFT_R {
        LCDTFT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&self) -> LCDMONO8_R {
        LCDMONO8_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&self) -> LCDDUAL_R {
        LCDDUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&self) -> BGR_R {
        BGR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&self) -> BEBO_R {
        BEBO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&self) -> BEPO_R {
        BEPO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&self) -> LCDPWR_R {
        LCDPWR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&self) -> LCDVCOMP_R {
        LCDVCOMP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<0> {
        LCDEN_W::new(self)
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&mut self) -> LCDBPP_W<1> {
        LCDBPP_W::new(self)
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&mut self) -> LCDBW_W<4> {
        LCDBW_W::new(self)
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&mut self) -> LCDTFT_W<5> {
        LCDTFT_W::new(self)
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&mut self) -> LCDMONO8_W<6> {
        LCDMONO8_W::new(self)
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&mut self) -> LCDDUAL_W<7> {
        LCDDUAL_W::new(self)
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&mut self) -> BGR_W<8> {
        BGR_W::new(self)
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&mut self) -> BEBO_W<9> {
        BEBO_W::new(self)
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&mut self) -> BEPO_W<10> {
        BEPO_W::new(self)
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&mut self) -> LCDPWR_W<11> {
        LCDPWR_W::new(self)
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&mut self) -> LCDVCOMP_W<12> {
        LCDVCOMP_W::new(self)
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W<16> {
        WATERMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
