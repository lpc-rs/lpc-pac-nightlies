#[doc = "Register `DIR` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO0` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO0_R = crate::BitReader<bool>;
#[doc = "Field `IO0` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO1` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO1_R = crate::BitReader<bool>;
#[doc = "Field `IO1` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO2` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO2_R = crate::BitReader<bool>;
#[doc = "Field `IO2` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO3` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO3_R = crate::BitReader<bool>;
#[doc = "Field `IO3` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO4` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO4_R = crate::BitReader<bool>;
#[doc = "Field `IO4` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO5` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO5_R = crate::BitReader<bool>;
#[doc = "Field `IO5` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO6` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO6_R = crate::BitReader<bool>;
#[doc = "Field `IO6` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO7` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO7_R = crate::BitReader<bool>;
#[doc = "Field `IO7` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO8` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO8_R = crate::BitReader<bool>;
#[doc = "Field `IO8` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO9` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO9_R = crate::BitReader<bool>;
#[doc = "Field `IO9` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO10` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO10_R = crate::BitReader<bool>;
#[doc = "Field `IO10` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
#[doc = "Field `IO11` reader - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO11_R = crate::BitReader<bool>;
#[doc = "Field `IO11` writer - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
pub type IO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io8(&self) -> IO8_R {
        IO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io9(&self) -> IO9_R {
        IO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io10(&self) -> IO10_R {
        IO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io11(&self) -> IO11_R {
        IO11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W<0> {
        IO0_W::new(self)
    }
    #[doc = "Bit 1 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W<1> {
        IO1_W::new(self)
    }
    #[doc = "Bit 2 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W<2> {
        IO2_W::new(self)
    }
    #[doc = "Bit 3 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W<3> {
        IO3_W::new(self)
    }
    #[doc = "Bit 4 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W<4> {
        IO4_W::new(self)
    }
    #[doc = "Bit 5 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W<5> {
        IO5_W::new(self)
    }
    #[doc = "Bit 6 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W<6> {
        IO6_W::new(self)
    }
    #[doc = "Bit 7 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W<7> {
        IO7_W::new(self)
    }
    #[doc = "Bit 8 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io8(&mut self) -> IO8_W<8> {
        IO8_W::new(self)
    }
    #[doc = "Bit 9 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io9(&mut self) -> IO9_W<9> {
        IO9_W::new(self)
    }
    #[doc = "Bit 10 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io10(&mut self) -> IO10_W<10> {
        IO10_W::new(self)
    }
    #[doc = "Bit 11 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io11(&mut self) -> IO11_W<11> {
        IO11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data direction register for port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
