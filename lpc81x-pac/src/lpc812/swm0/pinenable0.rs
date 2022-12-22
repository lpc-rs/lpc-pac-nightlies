#[doc = "Register `PINENABLE0` reader"]
pub struct R(crate::R<PINENABLE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINENABLE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINENABLE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINENABLE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINENABLE0` writer"]
pub struct W(crate::W<PINENABLE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINENABLE0_SPEC>;
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
impl From<crate::W<PINENABLE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINENABLE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP_I1` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type ACMP_I1_R = crate::BitReader<ACMP_I1_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I1_A {
    #[doc = "0: Enable ACMP_I1. This function is enabled on pin PIO0_0."]
    ENABLED = 0,
    #[doc = "1: Disable ACMP_I1. GPIO function PIO0_0 (default) or any other movable function can be assigned to pin PIO0_0."]
    DISABLED = 1,
}
impl From<ACMP_I1_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I1_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_I1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I1_A {
        match self.bits {
            false => ACMP_I1_A::ENABLED,
            true => ACMP_I1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I1_A::DISABLED
    }
}
#[doc = "Field `ACMP_I1` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type ACMP_I1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, ACMP_I1_A, O>;
impl<'a, const O: u8> ACMP_I1_W<'a, O> {
    #[doc = "Enable ACMP_I1. This function is enabled on pin PIO0_0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I1_A::ENABLED)
    }
    #[doc = "Disable ACMP_I1. GPIO function PIO0_0 (default) or any other movable function can be assigned to pin PIO0_0."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I1_A::DISABLED)
    }
}
#[doc = "Field `ACMP_I2` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
pub type ACMP_I2_R = crate::BitReader<ACMP_I2_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2_A {
    #[doc = "0: Enable ACMP_I2. This function is enabled on pin PIO0_1."]
    ACMP_I2_0 = 0,
    #[doc = "1: Disable ACMP_I2. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin PIO0_1."]
    ACMP_I2_1 = 1,
}
impl From<ACMP_I2_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I2_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_I2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I2_A {
        match self.bits {
            false => ACMP_I2_A::ACMP_I2_0,
            true => ACMP_I2_A::ACMP_I2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_0`"]
    #[inline(always)]
    pub fn is_acmp_i2_0(&self) -> bool {
        *self == ACMP_I2_A::ACMP_I2_0
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_1`"]
    #[inline(always)]
    pub fn is_acmp_i2_1(&self) -> bool {
        *self == ACMP_I2_A::ACMP_I2_1
    }
}
#[doc = "Field `ACMP_I2` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
pub type ACMP_I2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, ACMP_I2_A, O>;
impl<'a, const O: u8> ACMP_I2_W<'a, O> {
    #[doc = "Enable ACMP_I2. This function is enabled on pin PIO0_1."]
    #[inline(always)]
    pub fn acmp_i2_0(self) -> &'a mut W {
        self.variant(ACMP_I2_A::ACMP_I2_0)
    }
    #[doc = "Disable ACMP_I2. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin PIO0_1."]
    #[inline(always)]
    pub fn acmp_i2_1(self) -> &'a mut W {
        self.variant(ACMP_I2_A::ACMP_I2_1)
    }
}
#[doc = "Field `SWCLK` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type SWCLK_R = crate::BitReader<SWCLK_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLK_A {
    #[doc = "0: Enable SWCLK. This function is enabled on pin PIO0_3."]
    ENABLED = 0,
    #[doc = "1: Disable SWCLK. GPIO function PIO0_3 is selected on this pin. Any other movable function can be assigned to pin PIO0_3."]
    DISABLED = 1,
}
impl From<SWCLK_A> for bool {
    #[inline(always)]
    fn from(variant: SWCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SWCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWCLK_A {
        match self.bits {
            false => SWCLK_A::ENABLED,
            true => SWCLK_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWCLK_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWCLK_A::DISABLED
    }
}
#[doc = "Field `SWCLK` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type SWCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, SWCLK_A, O>;
impl<'a, const O: u8> SWCLK_W<'a, O> {
    #[doc = "Enable SWCLK. This function is enabled on pin PIO0_3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWCLK_A::ENABLED)
    }
    #[doc = "Disable SWCLK. GPIO function PIO0_3 is selected on this pin. Any other movable function can be assigned to pin PIO0_3."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWCLK_A::DISABLED)
    }
}
#[doc = "Field `SWDIO` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type SWDIO_R = crate::BitReader<SWDIO_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIO_A {
    #[doc = "0: Enable SWDIO. This function is enabled on pin PIO0_2."]
    ENABLED = 0,
    #[doc = "1: Disable SWDIO. GPIO function PIO0_2 is selected on this pin. Any other movable function can be assigned to pin PIO0_2."]
    DISABLED = 1,
}
impl From<SWDIO_A> for bool {
    #[inline(always)]
    fn from(variant: SWDIO_A) -> Self {
        variant as u8 != 0
    }
}
impl SWDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDIO_A {
        match self.bits {
            false => SWDIO_A::ENABLED,
            true => SWDIO_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWDIO_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWDIO_A::DISABLED
    }
}
#[doc = "Field `SWDIO` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type SWDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, SWDIO_A, O>;
impl<'a, const O: u8> SWDIO_W<'a, O> {
    #[doc = "Enable SWDIO. This function is enabled on pin PIO0_2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWDIO_A::ENABLED)
    }
    #[doc = "Disable SWDIO. GPIO function PIO0_2 is selected on this pin. Any other movable function can be assigned to pin PIO0_2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWDIO_A::DISABLED)
    }
}
#[doc = "Field `XTALIN` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type XTALIN_R = crate::BitReader<XTALIN_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALIN_A {
    #[doc = "0: Enable XTALIN. This function is enabled on pin PIO0_8."]
    ENABLED = 0,
    #[doc = "1: Disable XTALIN. GPIO function PIO0_8 (default) or any other movable function can be assigned to pin PIO0_8."]
    DISABLED = 1,
}
impl From<XTALIN_A> for bool {
    #[inline(always)]
    fn from(variant: XTALIN_A) -> Self {
        variant as u8 != 0
    }
}
impl XTALIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALIN_A {
        match self.bits {
            false => XTALIN_A::ENABLED,
            true => XTALIN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == XTALIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == XTALIN_A::DISABLED
    }
}
#[doc = "Field `XTALIN` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type XTALIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, XTALIN_A, O>;
impl<'a, const O: u8> XTALIN_W<'a, O> {
    #[doc = "Enable XTALIN. This function is enabled on pin PIO0_8."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALIN_A::ENABLED)
    }
    #[doc = "Disable XTALIN. GPIO function PIO0_8 (default) or any other movable function can be assigned to pin PIO0_8."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALIN_A::DISABLED)
    }
}
#[doc = "Field `XTALOUT` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type XTALOUT_R = crate::BitReader<XTALOUT_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUT_A {
    #[doc = "0: Enable XTALOUT. This function is enabled on pin PIO0_9."]
    ENABLED = 0,
    #[doc = "1: Disable XTALOUT. GPIO function PIO0_9 (default) or any other movable function can be assigned to pin PIO0_9."]
    DISABLED = 1,
}
impl From<XTALOUT_A> for bool {
    #[inline(always)]
    fn from(variant: XTALOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl XTALOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOUT_A {
        match self.bits {
            false => XTALOUT_A::ENABLED,
            true => XTALOUT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == XTALOUT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == XTALOUT_A::DISABLED
    }
}
#[doc = "Field `XTALOUT` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type XTALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, XTALOUT_A, O>;
impl<'a, const O: u8> XTALOUT_W<'a, O> {
    #[doc = "Enable XTALOUT. This function is enabled on pin PIO0_9."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALOUT_A::ENABLED)
    }
    #[doc = "Disable XTALOUT. GPIO function PIO0_9 (default) or any other movable function can be assigned to pin PIO0_9."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALOUT_A::DISABLED)
    }
}
#[doc = "Field `RESETN` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type RESETN_R = crate::BitReader<RESETN_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETN_A {
    #[doc = "0: Enable RESETN. This function is enabled on pin PIO0_5."]
    ENABLED = 0,
    #[doc = "1: Disable RESETN. GPIO function PIO0_5 is selected on this pin. Any other movable function can be assigned to pin PIO0_5."]
    DISABLED = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::ENABLED,
            true => RESETN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETN_A::DISABLED
    }
}
#[doc = "Field `RESETN` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
pub type RESETN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, RESETN_A, O>;
impl<'a, const O: u8> RESETN_W<'a, O> {
    #[doc = "Enable RESETN. This function is enabled on pin PIO0_5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETN_A::ENABLED)
    }
    #[doc = "Disable RESETN. GPIO function PIO0_5 is selected on this pin. Any other movable function can be assigned to pin PIO0_5."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETN_A::DISABLED)
    }
}
#[doc = "Field `CLKIN` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
pub type CLKIN_R = crate::BitReader<CLKIN_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIN_A {
    #[doc = "0: Enable CLKIN. This function is enabled on pin PIO0_1."]
    ENABLED = 0,
    #[doc = "1: Disable CLKIN. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin CLKIN."]
    DISABLED = 1,
}
impl From<CLKIN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKIN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKIN_A {
        match self.bits {
            false => CLKIN_A::ENABLED,
            true => CLKIN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKIN_A::DISABLED
    }
}
#[doc = "Field `CLKIN` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
pub type CLKIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, CLKIN_A, O>;
impl<'a, const O: u8> CLKIN_W<'a, O> {
    #[doc = "Enable CLKIN. This function is enabled on pin PIO0_1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKIN_A::ENABLED)
    }
    #[doc = "Disable CLKIN. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin CLKIN."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKIN_A::DISABLED)
    }
}
#[doc = "Field `VDDCMP` reader - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type VDDCMP_R = crate::BitReader<VDDCMP_A>;
#[doc = "Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMP_A {
    #[doc = "0: Enable VDDCMP. This function is enabled on pin PIO0_6."]
    ENABLED = 0,
    #[doc = "1: Disable VDDCMP. GPIO function PIO0_6 (default) or any other movable function can be assigned to pin PIO0_6."]
    DISABLED = 1,
}
impl From<VDDCMP_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCMP_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCMP_A {
        match self.bits {
            false => VDDCMP_A::ENABLED,
            true => VDDCMP_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDCMP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDCMP_A::DISABLED
    }
}
#[doc = "Field `VDDCMP` writer - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
pub type VDDCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PINENABLE0_SPEC, VDDCMP_A, O>;
impl<'a, const O: u8> VDDCMP_W<'a, O> {
    #[doc = "Enable VDDCMP. This function is enabled on pin PIO0_6."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VDDCMP_A::ENABLED)
    }
    #[doc = "Disable VDDCMP. GPIO function PIO0_6 (default) or any other movable function can be assigned to pin PIO0_6."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VDDCMP_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn acmp_i1(&self) -> ACMP_I1_R {
        ACMP_I1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
    #[inline(always)]
    pub fn acmp_i2(&self) -> ACMP_I2_R {
        ACMP_I2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn swclk(&self) -> SWCLK_R {
        SWCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn swdio(&self) -> SWDIO_R {
        SWDIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn xtalin(&self) -> XTALIN_R {
        XTALIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn xtalout(&self) -> XTALOUT_R {
        XTALOUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn vddcmp(&self) -> VDDCMP_R {
        VDDCMP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn acmp_i1(&mut self) -> ACMP_I1_W<0> {
        ACMP_I1_W::new(self)
    }
    #[doc = "Bit 1 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
    #[inline(always)]
    pub fn acmp_i2(&mut self) -> ACMP_I2_W<1> {
        ACMP_I2_W::new(self)
    }
    #[doc = "Bit 2 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn swclk(&mut self) -> SWCLK_W<2> {
        SWCLK_W::new(self)
    }
    #[doc = "Bit 3 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn swdio(&mut self) -> SWDIO_W<3> {
        SWDIO_W::new(self)
    }
    #[doc = "Bit 4 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn xtalin(&mut self) -> XTALIN_W<4> {
        XTALIN_W::new(self)
    }
    #[doc = "Bit 5 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn xtalout(&mut self) -> XTALOUT_W<5> {
        XTALOUT_W::new(self)
    }
    #[doc = "Bit 6 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W<6> {
        RESETN_W::new(self)
    }
    #[doc = "Bit 7 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W<7> {
        CLKIN_W::new(self)
    }
    #[doc = "Bit 8 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline(always)]
    pub fn vddcmp(&mut self) -> VDDCMP_W<8> {
        VDDCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinenable0](index.html) module"]
pub struct PINENABLE0_SPEC;
impl crate::RegisterSpec for PINENABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinenable0::R](R) reader structure"]
impl crate::Readable for PINENABLE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinenable0::W](W) writer structure"]
impl crate::Writable for PINENABLE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINENABLE0 to value 0x01b3"]
impl crate::Resettable for PINENABLE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01b3
    }
}
