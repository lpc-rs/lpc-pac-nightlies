#[doc = "Register `IS` reader"]
pub struct R(crate::R<IS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS` writer"]
pub struct W(crate::W<IS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IS_SPEC>;
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
impl From<crate::W<IS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISENSE0` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE0_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE0` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE1` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE1_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE1` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE2` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE2_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE2` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE3` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE3_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE3` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE4` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE4_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE4` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE5` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE5_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE5` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE6` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE6_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE6` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE7` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE7_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE7` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE8` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE8_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE8` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE9` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE9_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE9` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE10` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE10_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE10` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
#[doc = "Field `ISENSE11` reader - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE11_R = crate::BitReader<bool>;
#[doc = "Field `ISENSE11` writer - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
pub type ISENSE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense0(&self) -> ISENSE0_R {
        ISENSE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense1(&self) -> ISENSE1_R {
        ISENSE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense2(&self) -> ISENSE2_R {
        ISENSE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense3(&self) -> ISENSE3_R {
        ISENSE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense4(&self) -> ISENSE4_R {
        ISENSE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense5(&self) -> ISENSE5_R {
        ISENSE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense6(&self) -> ISENSE6_R {
        ISENSE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense7(&self) -> ISENSE7_R {
        ISENSE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense8(&self) -> ISENSE8_R {
        ISENSE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense9(&self) -> ISENSE9_R {
        ISENSE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense10(&self) -> ISENSE10_R {
        ISENSE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense11(&self) -> ISENSE11_R {
        ISENSE11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense0(&mut self) -> ISENSE0_W<0> {
        ISENSE0_W::new(self)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense1(&mut self) -> ISENSE1_W<1> {
        ISENSE1_W::new(self)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense2(&mut self) -> ISENSE2_W<2> {
        ISENSE2_W::new(self)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense3(&mut self) -> ISENSE3_W<3> {
        ISENSE3_W::new(self)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense4(&mut self) -> ISENSE4_W<4> {
        ISENSE4_W::new(self)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense5(&mut self) -> ISENSE5_W<5> {
        ISENSE5_W::new(self)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense6(&mut self) -> ISENSE6_W<6> {
        ISENSE6_W::new(self)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense7(&mut self) -> ISENSE7_W<7> {
        ISENSE7_W::new(self)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense8(&mut self) -> ISENSE8_W<8> {
        ISENSE8_W::new(self)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense9(&mut self) -> ISENSE9_W<9> {
        ISENSE9_W::new(self)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense10(&mut self) -> ISENSE10_W<10> {
        ISENSE10_W::new(self)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense11(&mut self) -> ISENSE11_W<11> {
        ISENSE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt sense register for port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](index.html) module"]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [is::R](R) reader structure"]
impl crate::Readable for IS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [is::W](W) writer structure"]
impl crate::Writable for IS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
