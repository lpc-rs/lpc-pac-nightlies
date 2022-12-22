#[doc = "Register `IBE` reader"]
pub struct R(crate::R<IBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBE` writer"]
pub struct W(crate::W<IBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBE_SPEC>;
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
impl From<crate::W<IBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBE0` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE0_R = crate::BitReader<bool>;
#[doc = "Field `IBE0` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE1` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE1_R = crate::BitReader<bool>;
#[doc = "Field `IBE1` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE2` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE2_R = crate::BitReader<bool>;
#[doc = "Field `IBE2` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE3` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE3_R = crate::BitReader<bool>;
#[doc = "Field `IBE3` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE4` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE4_R = crate::BitReader<bool>;
#[doc = "Field `IBE4` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE5` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE5_R = crate::BitReader<bool>;
#[doc = "Field `IBE5` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE6` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE6_R = crate::BitReader<bool>;
#[doc = "Field `IBE6` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE7` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE7_R = crate::BitReader<bool>;
#[doc = "Field `IBE7` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE8` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE8_R = crate::BitReader<bool>;
#[doc = "Field `IBE8` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE9` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE9_R = crate::BitReader<bool>;
#[doc = "Field `IBE9` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE10` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE10_R = crate::BitReader<bool>;
#[doc = "Field `IBE10` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
#[doc = "Field `IBE11` reader - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE11_R = crate::BitReader<bool>;
#[doc = "Field `IBE11` writer - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
pub type IBE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe0(&self) -> IBE0_R {
        IBE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe1(&self) -> IBE1_R {
        IBE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe2(&self) -> IBE2_R {
        IBE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe3(&self) -> IBE3_R {
        IBE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe4(&self) -> IBE4_R {
        IBE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe5(&self) -> IBE5_R {
        IBE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe6(&self) -> IBE6_R {
        IBE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe7(&self) -> IBE7_R {
        IBE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe8(&self) -> IBE8_R {
        IBE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe9(&self) -> IBE9_R {
        IBE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe10(&self) -> IBE10_R {
        IBE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe11(&self) -> IBE11_R {
        IBE11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe0(&mut self) -> IBE0_W<0> {
        IBE0_W::new(self)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe1(&mut self) -> IBE1_W<1> {
        IBE1_W::new(self)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe2(&mut self) -> IBE2_W<2> {
        IBE2_W::new(self)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe3(&mut self) -> IBE3_W<3> {
        IBE3_W::new(self)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe4(&mut self) -> IBE4_W<4> {
        IBE4_W::new(self)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe5(&mut self) -> IBE5_W<5> {
        IBE5_W::new(self)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe6(&mut self) -> IBE6_W<6> {
        IBE6_W::new(self)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe7(&mut self) -> IBE7_W<7> {
        IBE7_W::new(self)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe8(&mut self) -> IBE8_W<8> {
        IBE8_W::new(self)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe9(&mut self) -> IBE9_W<9> {
        IBE9_W::new(self)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe10(&mut self) -> IBE10_W<10> {
        IBE10_W::new(self)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe11(&mut self) -> IBE11_W<11> {
        IBE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt both edges register for port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibe](index.html) module"]
pub struct IBE_SPEC;
impl crate::RegisterSpec for IBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibe::R](R) reader structure"]
impl crate::Readable for IBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibe::W](W) writer structure"]
impl crate::Writable for IBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBE to value 0"]
impl crate::Resettable for IBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
