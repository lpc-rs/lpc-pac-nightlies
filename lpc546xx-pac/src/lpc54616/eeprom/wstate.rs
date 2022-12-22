#[doc = "Register `WSTATE` reader"]
pub struct R(crate::R<WSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WSTATE` writer"]
pub struct W(crate::W<WSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSTATE_SPEC>;
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
impl From<crate::W<WSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE3` reader - Wait states for phase 3 (minus 1 encoded)."]
pub type PHASE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE3` writer - Wait states for phase 3 (minus 1 encoded)."]
pub type PHASE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WSTATE_SPEC, u8, u8, 8, O>;
#[doc = "Field `PHASE2` reader - Wait states for phase 2 (minus 1 encoded)."]
pub type PHASE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE2` writer - Wait states for phase 2 (minus 1 encoded)."]
pub type PHASE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WSTATE_SPEC, u8, u8, 8, O>;
#[doc = "Field `PHASE1` reader - Wait states for phase 1 (minus 1 encoded)."]
pub type PHASE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE1` writer - Wait states for phase 1 (minus 1 encoded)."]
pub type PHASE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WSTATE_SPEC, u8, u8, 8, O>;
#[doc = "Field `LCK_PARWEP` reader - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
pub type LCK_PARWEP_R = crate::BitReader<bool>;
#[doc = "Field `LCK_PARWEP` writer - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
pub type LCK_PARWEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WSTATE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&self) -> LCK_PARWEP_R {
        LCK_PARWEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W<0> {
        PHASE3_W::new(self)
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W<8> {
        PHASE2_W::new(self)
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W<16> {
        PHASE1_W::new(self)
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&mut self) -> LCK_PARWEP_W<31> {
        LCK_PARWEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wstate](index.html) module"]
pub struct WSTATE_SPEC;
impl crate::RegisterSpec for WSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wstate::R](R) reader structure"]
impl crate::Readable for WSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wstate::W](W) writer structure"]
impl crate::Writable for WSTATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WSTATE to value 0x0004_0802"]
impl crate::Resettable for WSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0802
    }
}
