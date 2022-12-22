#[doc = "Register `IEV` reader"]
pub struct R(crate::R<IEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEV` writer"]
pub struct W(crate::W<IEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEV_SPEC>;
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
impl From<crate::W<IEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEV0` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV0_R = crate::BitReader<bool>;
#[doc = "Field `IEV0` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV1` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV1_R = crate::BitReader<bool>;
#[doc = "Field `IEV1` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV2` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV2_R = crate::BitReader<bool>;
#[doc = "Field `IEV2` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV3` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV3_R = crate::BitReader<bool>;
#[doc = "Field `IEV3` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV4` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV4_R = crate::BitReader<bool>;
#[doc = "Field `IEV4` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV5` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV5_R = crate::BitReader<bool>;
#[doc = "Field `IEV5` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV6` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV6_R = crate::BitReader<bool>;
#[doc = "Field `IEV6` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV7` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV7_R = crate::BitReader<bool>;
#[doc = "Field `IEV7` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV8` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV8_R = crate::BitReader<bool>;
#[doc = "Field `IEV8` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV9` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV9_R = crate::BitReader<bool>;
#[doc = "Field `IEV9` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV10` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV10_R = crate::BitReader<bool>;
#[doc = "Field `IEV10` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
#[doc = "Field `IEV11` reader - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV11_R = crate::BitReader<bool>;
#[doc = "Field `IEV11` writer - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
pub type IEV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEV_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev0(&self) -> IEV0_R {
        IEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev1(&self) -> IEV1_R {
        IEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev2(&self) -> IEV2_R {
        IEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev3(&self) -> IEV3_R {
        IEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev4(&self) -> IEV4_R {
        IEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev5(&self) -> IEV5_R {
        IEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev6(&self) -> IEV6_R {
        IEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev7(&self) -> IEV7_R {
        IEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev8(&self) -> IEV8_R {
        IEV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev9(&self) -> IEV9_R {
        IEV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev10(&self) -> IEV10_R {
        IEV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev11(&self) -> IEV11_R {
        IEV11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev0(&mut self) -> IEV0_W<0> {
        IEV0_W::new(self)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev1(&mut self) -> IEV1_W<1> {
        IEV1_W::new(self)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev2(&mut self) -> IEV2_W<2> {
        IEV2_W::new(self)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev3(&mut self) -> IEV3_W<3> {
        IEV3_W::new(self)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev4(&mut self) -> IEV4_W<4> {
        IEV4_W::new(self)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev5(&mut self) -> IEV5_W<5> {
        IEV5_W::new(self)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev6(&mut self) -> IEV6_W<6> {
        IEV6_W::new(self)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev7(&mut self) -> IEV7_W<7> {
        IEV7_W::new(self)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev8(&mut self) -> IEV8_W<8> {
        IEV8_W::new(self)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev9(&mut self) -> IEV9_W<9> {
        IEV9_W::new(self)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev10(&mut self) -> IEV10_W<10> {
        IEV10_W::new(self)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev11(&mut self) -> IEV11_W<11> {
        IEV11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt event register for port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iev](index.html) module"]
pub struct IEV_SPEC;
impl crate::RegisterSpec for IEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iev::R](R) reader structure"]
impl crate::Readable for IEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iev::W](W) writer structure"]
impl crate::Writable for IEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
