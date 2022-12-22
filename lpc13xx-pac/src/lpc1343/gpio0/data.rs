#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA0_R = crate::BitReader<bool>;
#[doc = "Field `DATA0` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA1` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA1_R = crate::BitReader<bool>;
#[doc = "Field `DATA1` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA2` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA2_R = crate::BitReader<bool>;
#[doc = "Field `DATA2` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA3` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA3_R = crate::BitReader<bool>;
#[doc = "Field `DATA3` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA4` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA4_R = crate::BitReader<bool>;
#[doc = "Field `DATA4` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA5` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA5_R = crate::BitReader<bool>;
#[doc = "Field `DATA5` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA6` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA6_R = crate::BitReader<bool>;
#[doc = "Field `DATA6` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA7` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA7_R = crate::BitReader<bool>;
#[doc = "Field `DATA7` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA8` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA8_R = crate::BitReader<bool>;
#[doc = "Field `DATA8` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA9` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA9_R = crate::BitReader<bool>;
#[doc = "Field `DATA9` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA10` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA10_R = crate::BitReader<bool>;
#[doc = "Field `DATA10` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `DATA11` reader - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA11_R = crate::BitReader<bool>;
#[doc = "Field `DATA11` writer - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
pub type DATA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Bit 1 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<1> {
        DATA1_W::new(self)
    }
    #[doc = "Bit 2 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<2> {
        DATA2_W::new(self)
    }
    #[doc = "Bit 3 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<3> {
        DATA3_W::new(self)
    }
    #[doc = "Bit 4 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<4> {
        DATA4_W::new(self)
    }
    #[doc = "Bit 5 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W<5> {
        DATA5_W::new(self)
    }
    #[doc = "Bit 6 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W<6> {
        DATA6_W::new(self)
    }
    #[doc = "Bit 7 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W<7> {
        DATA7_W::new(self)
    }
    #[doc = "Bit 8 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<8> {
        DATA8_W::new(self)
    }
    #[doc = "Bit 9 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W<9> {
        DATA9_W::new(self)
    }
    #[doc = "Bit 10 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W<10> {
        DATA10_W::new(self)
    }
    #[doc = "Bit 11 - Logic levels for pins PIOn_0 to PIOn_11. HIGH = 1, LOW = 0."]
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W<11> {
        DATA11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port n data register for pins PIOn_0 to PIOn_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
