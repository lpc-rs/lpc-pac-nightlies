#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK0` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK0_R = crate::BitReader<bool>;
#[doc = "Field `MASK0` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK1` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK1_R = crate::BitReader<bool>;
#[doc = "Field `MASK1` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK2` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK2_R = crate::BitReader<bool>;
#[doc = "Field `MASK2` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK3` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK3_R = crate::BitReader<bool>;
#[doc = "Field `MASK3` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK4` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK4_R = crate::BitReader<bool>;
#[doc = "Field `MASK4` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK5` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK5_R = crate::BitReader<bool>;
#[doc = "Field `MASK5` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK6` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK6_R = crate::BitReader<bool>;
#[doc = "Field `MASK6` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK7` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK7_R = crate::BitReader<bool>;
#[doc = "Field `MASK7` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK8` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK8_R = crate::BitReader<bool>;
#[doc = "Field `MASK8` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK9` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK9_R = crate::BitReader<bool>;
#[doc = "Field `MASK9` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK10` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK10_R = crate::BitReader<bool>;
#[doc = "Field `MASK10` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MASK11` reader - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK11_R = crate::BitReader<bool>;
#[doc = "Field `MASK11` writer - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
pub type MASK11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask8(&self) -> MASK8_R {
        MASK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask9(&self) -> MASK9_R {
        MASK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask10(&self) -> MASK10_R {
        MASK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask11(&self) -> MASK11_R {
        MASK11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask0(&mut self) -> MASK0_W<0> {
        MASK0_W::new(self)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W<1> {
        MASK1_W::new(self)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W<2> {
        MASK2_W::new(self)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W<3> {
        MASK3_W::new(self)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask4(&mut self) -> MASK4_W<4> {
        MASK4_W::new(self)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask5(&mut self) -> MASK5_W<5> {
        MASK5_W::new(self)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask6(&mut self) -> MASK6_W<6> {
        MASK6_W::new(self)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask7(&mut self) -> MASK7_W<7> {
        MASK7_W::new(self)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask8(&mut self) -> MASK8_W<8> {
        MASK8_W::new(self)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask9(&mut self) -> MASK9_W<9> {
        MASK9_W::new(self)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask10(&mut self) -> MASK10_W<10> {
        MASK10_W::new(self)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = Interrupt on pin PIOn_x is masked. 1 = Interrupt on pin PIOn_x is not masked."]
    #[inline(always)]
    pub fn mask11(&mut self) -> MASK11_W<11> {
        MASK11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register for port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
