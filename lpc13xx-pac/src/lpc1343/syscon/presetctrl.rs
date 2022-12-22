#[doc = "Register `PRESETCTRL` reader"]
pub struct R(crate::R<PRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL` writer"]
pub struct W(crate::W<PRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_SPEC>;
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
impl From<crate::W<PRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSP0_RST_N` reader - SSP0 reset control"]
pub type SSP0_RST_N_R = crate::BitReader<SSP0_RST_N_A>;
#[doc = "SSP0 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_N_A {
    #[doc = "0: Reset SSP0."]
    RESET_SSP0_ = 0,
    #[doc = "1: De-assert SSP0 reset."]
    DE_ASSERT_SSP0_RESET = 1,
}
impl From<SSP0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP0_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_RST_N_A {
        match self.bits {
            false => SSP0_RST_N_A::RESET_SSP0_,
            true => SSP0_RST_N_A::DE_ASSERT_SSP0_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_SSP0_`"]
    #[inline(always)]
    pub fn is_reset_ssp0_(&self) -> bool {
        *self == SSP0_RST_N_A::RESET_SSP0_
    }
    #[doc = "Checks if the value of the field is `DE_ASSERT_SSP0_RESET`"]
    #[inline(always)]
    pub fn is_de_assert_ssp0_reset(&self) -> bool {
        *self == SSP0_RST_N_A::DE_ASSERT_SSP0_RESET
    }
}
#[doc = "Field `SSP0_RST_N` writer - SSP0 reset control"]
pub type SSP0_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SSP0_RST_N_A, O>;
impl<'a, const O: u8> SSP0_RST_N_W<'a, O> {
    #[doc = "Reset SSP0."]
    #[inline(always)]
    pub fn reset_ssp0_(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::RESET_SSP0_)
    }
    #[doc = "De-assert SSP0 reset."]
    #[inline(always)]
    pub fn de_assert_ssp0_reset(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::DE_ASSERT_SSP0_RESET)
    }
}
#[doc = "Field `I2C_RST_N` reader - I2C reset control"]
pub type I2C_RST_N_R = crate::BitReader<I2C_RST_N_A>;
#[doc = "I2C reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_N_A {
    #[doc = "0: Reset I2C."]
    RESET_I2C_ = 0,
    #[doc = "1: De-asset I2C reset."]
    DE_ASSET_I2C_RESET_ = 1,
}
impl From<I2C_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_RST_N_A {
        match self.bits {
            false => I2C_RST_N_A::RESET_I2C_,
            true => I2C_RST_N_A::DE_ASSET_I2C_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_I2C_`"]
    #[inline(always)]
    pub fn is_reset_i2c_(&self) -> bool {
        *self == I2C_RST_N_A::RESET_I2C_
    }
    #[doc = "Checks if the value of the field is `DE_ASSET_I2C_RESET_`"]
    #[inline(always)]
    pub fn is_de_asset_i2c_reset_(&self) -> bool {
        *self == I2C_RST_N_A::DE_ASSET_I2C_RESET_
    }
}
#[doc = "Field `I2C_RST_N` writer - I2C reset control"]
pub type I2C_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, I2C_RST_N_A, O>;
impl<'a, const O: u8> I2C_RST_N_W<'a, O> {
    #[doc = "Reset I2C."]
    #[inline(always)]
    pub fn reset_i2c_(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::RESET_I2C_)
    }
    #[doc = "De-asset I2C reset."]
    #[inline(always)]
    pub fn de_asset_i2c_reset_(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::DE_ASSET_I2C_RESET_)
    }
}
#[doc = "Field `SSP1_RST_N` reader - SPISP1 reset control"]
pub type SSP1_RST_N_R = crate::BitReader<SSP1_RST_N_A>;
#[doc = "SPISP1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_N_A {
    #[doc = "0: Reset the SPISP1."]
    RESET_THE_SPISP1_ = 0,
    #[doc = "1: De-assert SPISP1 reset."]
    DE_ASSERT_SPISP1_RES = 1,
}
impl From<SSP1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP1_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_RST_N_A {
        match self.bits {
            false => SSP1_RST_N_A::RESET_THE_SPISP1_,
            true => SSP1_RST_N_A::DE_ASSERT_SPISP1_RES,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_THE_SPISP1_`"]
    #[inline(always)]
    pub fn is_reset_the_spisp1_(&self) -> bool {
        *self == SSP1_RST_N_A::RESET_THE_SPISP1_
    }
    #[doc = "Checks if the value of the field is `DE_ASSERT_SPISP1_RES`"]
    #[inline(always)]
    pub fn is_de_assert_spisp1_res(&self) -> bool {
        *self == SSP1_RST_N_A::DE_ASSERT_SPISP1_RES
    }
}
#[doc = "Field `SSP1_RST_N` writer - SPISP1 reset control"]
pub type SSP1_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SSP1_RST_N_A, O>;
impl<'a, const O: u8> SSP1_RST_N_W<'a, O> {
    #[doc = "Reset the SPISP1."]
    #[inline(always)]
    pub fn reset_the_spisp1_(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::RESET_THE_SPISP1_)
    }
    #[doc = "De-assert SPISP1 reset."]
    #[inline(always)]
    pub fn de_assert_spisp1_res(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::DE_ASSERT_SPISP1_RES)
    }
}
impl R {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_N_R {
        SSP0_RST_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&self) -> I2C_RST_N_R {
        I2C_RST_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPISP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_N_R {
        SSP1_RST_N_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> SSP0_RST_N_W<0> {
        SSP0_RST_N_W::new(self)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&mut self) -> I2C_RST_N_W<1> {
        I2C_RST_N_W::new(self)
    }
    #[doc = "Bit 2 - SPISP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> SSP1_RST_N_W<2> {
        SSP1_RST_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl](index.html) module"]
pub struct PRESETCTRL_SPEC;
impl crate::RegisterSpec for PRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0"]
impl crate::Resettable for PRESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
