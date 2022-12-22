#[doc = "Register `SWCLK_PIO0_10` reader"]
pub struct R(crate::R<SWCLK_PIO0_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWCLK_PIO0_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWCLK_PIO0_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWCLK_PIO0_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWCLK_PIO0_10` writer"]
pub struct W(crate::W<SWCLK_PIO0_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWCLK_PIO0_10_SPEC>;
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
impl From<crate::W<SWCLK_PIO0_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWCLK_PIO0_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. All other values are reserved."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Selects function SWCLK."]
    SWC = 0,
    #[doc = "1: Selects function PIO0_10."]
    PIO = 1,
    #[doc = "2: Selects function SCK0 (only if pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 selected in Table 139)."]
    SCK = 2,
    #[doc = "3: Selects function CT16B0_MAT2."]
    CT1 = 3,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::SWC),
            1 => Some(FUNC_A::PIO),
            2 => Some(FUNC_A::SCK),
            3 => Some(FUNC_A::CT1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWC`"]
    #[inline(always)]
    pub fn is_swc(&self) -> bool {
        *self == FUNC_A::SWC
    }
    #[doc = "Checks if the value of the field is `PIO`"]
    #[inline(always)]
    pub fn is_pio(&self) -> bool {
        *self == FUNC_A::PIO
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == FUNC_A::SCK
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline(always)]
    pub fn is_ct1(&self) -> bool {
        *self == FUNC_A::CT1
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. All other values are reserved."]
pub type FUNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWCLK_PIO0_10_SPEC, u8, FUNC_A, 3, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Selects function SWCLK."]
    #[inline(always)]
    pub fn swc(self) -> &'a mut W {
        self.variant(FUNC_A::SWC)
    }
    #[doc = "Selects function PIO0_10."]
    #[inline(always)]
    pub fn pio(self) -> &'a mut W {
        self.variant(FUNC_A::PIO)
    }
    #[doc = "Selects function SCK0 (only if pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 selected in Table 139)."]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(FUNC_A::SCK)
    }
    #[doc = "Selects function CT16B0_MAT2."]
    #[inline(always)]
    pub fn ct1(self) -> &'a mut W {
        self.variant(FUNC_A::CT1)
    }
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)"]
    INACTIVE_NO_PULL_DO = 0,
    #[doc = "1: Pull-down resistor enabled"]
    PULL_DOWN_RESISTOR_E = 1,
    #[doc = "2: Pull-up resistor enabled"]
    PULL_UP_RESISTOR_ENA = 2,
    #[doc = "3: Repeater mode"]
    REPEATER_MODE = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE_NO_PULL_DO,
            1 => MODE_A::PULL_DOWN_RESISTOR_E,
            2 => MODE_A::PULL_UP_RESISTOR_ENA,
            3 => MODE_A::REPEATER_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODE_A::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODE_A::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODE_A::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE`"]
    #[inline(always)]
    pub fn is_repeater_mode(&self) -> bool {
        *self == MODE_A::REPEATER_MODE
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)"]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SWCLK_PIO0_10_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)"]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled"]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled"]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode"]
    #[inline(always)]
    pub fn repeater_mode(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER_MODE)
    }
}
#[doc = "Field `HYS` reader - Hysteresis"]
pub type HYS_R = crate::BitReader<HYS_A>;
#[doc = "Hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
impl HYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::DISABLE,
            true => HYS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYS_A::ENABLE
    }
}
#[doc = "Field `HYS` writer - Hysteresis"]
pub type HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWCLK_PIO0_10_SPEC, HYS_A, O>;
impl<'a, const O: u8> HYS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYS_A::ENABLE)
    }
}
#[doc = "Field `OD` reader - Selects pseudo open-drain mode."]
pub type OD_R = crate::BitReader<OD_A>;
#[doc = "Selects pseudo open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "0: Standard GPIO output"]
    STANDARD_GPIO_OUTPUT = 0,
    #[doc = "1: Open-drain output"]
    OPEN_DRAIN_OUTPUT = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
impl OD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::STANDARD_GPIO_OUTPUT,
            true => OD_A::OPEN_DRAIN_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_GPIO_OUTPUT`"]
    #[inline(always)]
    pub fn is_standard_gpio_output(&self) -> bool {
        *self == OD_A::STANDARD_GPIO_OUTPUT
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_OUTPUT`"]
    #[inline(always)]
    pub fn is_open_drain_output(&self) -> bool {
        *self == OD_A::OPEN_DRAIN_OUTPUT
    }
}
#[doc = "Field `OD` writer - Selects pseudo open-drain mode."]
pub type OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWCLK_PIO0_10_SPEC, OD_A, O>;
impl<'a, const O: u8> OD_W<'a, O> {
    #[doc = "Standard GPIO output"]
    #[inline(always)]
    pub fn standard_gpio_output(self) -> &'a mut W {
        self.variant(OD_A::STANDARD_GPIO_OUTPUT)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn open_drain_output(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN_OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Hysteresis"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - Hysteresis"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W<5> {
        HYS_W::new(self)
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W<10> {
        OD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swclk_pio0_10](index.html) module"]
pub struct SWCLK_PIO0_10_SPEC;
impl crate::RegisterSpec for SWCLK_PIO0_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swclk_pio0_10::R](R) reader structure"]
impl crate::Readable for SWCLK_PIO0_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swclk_pio0_10::W](W) writer structure"]
impl crate::Writable for SWCLK_PIO0_10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWCLK_PIO0_10 to value 0xd0"]
impl crate::Resettable for SWCLK_PIO0_10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd0
    }
}
