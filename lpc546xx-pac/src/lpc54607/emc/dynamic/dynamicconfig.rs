#[doc = "Register `DYNAMICCONFIG` reader"]
pub struct R(crate::R<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONFIG` writer"]
pub struct W(crate::W<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONFIG_SPEC>;
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
impl From<crate::W<DYNAMICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - Memory device."]
pub type MD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MD` writer - Memory device."]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICCONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `AM0` reader - See Table 933."]
pub type AM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AM0` writer - See Table 933."]
pub type AM0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICCONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `AM1` reader - See Table 933."]
pub type AM1_R = crate::BitReader<bool>;
#[doc = "Field `AM1` writer - See Table 933."]
pub type AM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONFIG_SPEC, bool, O>;
#[doc = "Field `B` reader - Buffer enable."]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Buffer enable."]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONFIG_SPEC, bool, O>;
#[doc = "Field `P` reader - Write protect."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - Write protect."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&self) -> AM0_R {
        AM0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&self) -> AM1_R {
        AM1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<3> {
        MD_W::new(self)
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&mut self) -> AM0_W<7> {
        AM0_W::new(self)
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&mut self) -> AM1_W<14> {
        AM1_W::new(self)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<19> {
        B_W::new(self)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W<20> {
        P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration information for EMC_DYCSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicconfig](index.html) module"]
pub struct DYNAMICCONFIG_SPEC;
impl crate::RegisterSpec for DYNAMICCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicconfig::R](R) reader structure"]
impl crate::Readable for DYNAMICCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicconfig::W](W) writer structure"]
impl crate::Writable for DYNAMICCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONFIG to value 0"]
impl crate::Resettable for DYNAMICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
