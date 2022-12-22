#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGESEL` reader - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
pub type EDGESEL_R = crate::FieldReader<u8, EDGESEL_A>;
#[doc = "This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGESEL_A {
    #[doc = "0: Falling edges"]
    FALLING_EDGES = 0,
    #[doc = "1: Rising edges"]
    RISING_EDGES = 1,
    #[doc = "2: Both edges"]
    BOTH_EDGES0 = 2,
    #[doc = "3: Both edges"]
    BOTH_EDGES1 = 3,
}
impl From<EDGESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGESEL_A) -> Self {
        variant as _
    }
}
impl EDGESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGESEL_A {
        match self.bits {
            0 => EDGESEL_A::FALLING_EDGES,
            1 => EDGESEL_A::RISING_EDGES,
            2 => EDGESEL_A::BOTH_EDGES0,
            3 => EDGESEL_A::BOTH_EDGES1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_falling_edges(&self) -> bool {
        *self == EDGESEL_A::FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES`"]
    #[inline(always)]
    pub fn is_rising_edges(&self) -> bool {
        *self == EDGESEL_A::RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES0`"]
    #[inline(always)]
    pub fn is_both_edges0(&self) -> bool {
        *self == EDGESEL_A::BOTH_EDGES0
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES1`"]
    #[inline(always)]
    pub fn is_both_edges1(&self) -> bool {
        *self == EDGESEL_A::BOTH_EDGES1
    }
}
#[doc = "Field `EDGESEL` writer - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
pub type EDGESEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, EDGESEL_A, 2, O>;
impl<'a, const O: u8> EDGESEL_W<'a, O> {
    #[doc = "Falling edges"]
    #[inline(always)]
    pub fn falling_edges(self) -> &'a mut W {
        self.variant(EDGESEL_A::FALLING_EDGES)
    }
    #[doc = "Rising edges"]
    #[inline(always)]
    pub fn rising_edges(self) -> &'a mut W {
        self.variant(EDGESEL_A::RISING_EDGES)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both_edges0(self) -> &'a mut W {
        self.variant(EDGESEL_A::BOTH_EDGES0)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both_edges1(self) -> &'a mut W {
        self.variant(EDGESEL_A::BOTH_EDGES1)
    }
}
#[doc = "Field `COMPSA` reader - Comparator output control"]
pub type COMPSA_R = crate::BitReader<COMPSA_A>;
#[doc = "Comparator output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPSA_A {
    #[doc = "0: Comparator output is used directly."]
    COMPSA_0 = 0,
    #[doc = "1: Comparator output is synchronized to the bus clock for output to other modules."]
    COMPSA_1 = 1,
}
impl From<COMPSA_A> for bool {
    #[inline(always)]
    fn from(variant: COMPSA_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPSA_A {
        match self.bits {
            false => COMPSA_A::COMPSA_0,
            true => COMPSA_A::COMPSA_1,
        }
    }
    #[doc = "Checks if the value of the field is `COMPSA_0`"]
    #[inline(always)]
    pub fn is_compsa_0(&self) -> bool {
        *self == COMPSA_A::COMPSA_0
    }
    #[doc = "Checks if the value of the field is `COMPSA_1`"]
    #[inline(always)]
    pub fn is_compsa_1(&self) -> bool {
        *self == COMPSA_A::COMPSA_1
    }
}
#[doc = "Field `COMPSA` writer - Comparator output control"]
pub type COMPSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, COMPSA_A, O>;
impl<'a, const O: u8> COMPSA_W<'a, O> {
    #[doc = "Comparator output is used directly."]
    #[inline(always)]
    pub fn compsa_0(self) -> &'a mut W {
        self.variant(COMPSA_A::COMPSA_0)
    }
    #[doc = "Comparator output is synchronized to the bus clock for output to other modules."]
    #[inline(always)]
    pub fn compsa_1(self) -> &'a mut W {
        self.variant(COMPSA_A::COMPSA_1)
    }
}
#[doc = "Field `COMP_VP_SEL` reader - Selects positive voltage input"]
pub type COMP_VP_SEL_R = crate::FieldReader<u8, COMP_VP_SEL_A>;
#[doc = "Selects positive voltage input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_VP_SEL_A {
    #[doc = "0: VOLTAGE_LADDER_OUTPUT"]
    VOLTAGE_LADDER_OUTPUT = 0,
    #[doc = "1: ACMP_I1"]
    ACMP_I1 = 1,
    #[doc = "2: ACMP_I2"]
    ACMP_I2 = 2,
    #[doc = "3: ACMP_I3"]
    ACMP_I3 = 3,
    #[doc = "4: ACMP_I4"]
    ACMP_I4 = 4,
    #[doc = "5: ACMP_I5"]
    ACMP_I5 = 5,
    #[doc = "6: Band gap. Internal reference voltage."]
    BAND_GAP = 6,
    #[doc = "7: DAC0 output"]
    DACOUT0 = 7,
}
impl From<COMP_VP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_VP_SEL_A) -> Self {
        variant as _
    }
}
impl COMP_VP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_VP_SEL_A {
        match self.bits {
            0 => COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT,
            1 => COMP_VP_SEL_A::ACMP_I1,
            2 => COMP_VP_SEL_A::ACMP_I2,
            3 => COMP_VP_SEL_A::ACMP_I3,
            4 => COMP_VP_SEL_A::ACMP_I4,
            5 => COMP_VP_SEL_A::ACMP_I5,
            6 => COMP_VP_SEL_A::BAND_GAP,
            7 => COMP_VP_SEL_A::DACOUT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPUT`"]
    #[inline(always)]
    pub fn is_voltage_ladder_output(&self) -> bool {
        *self == COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline(always)]
    pub fn is_acmp_i1(&self) -> bool {
        *self == COMP_VP_SEL_A::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline(always)]
    pub fn is_acmp_i2(&self) -> bool {
        *self == COMP_VP_SEL_A::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline(always)]
    pub fn is_acmp_i3(&self) -> bool {
        *self == COMP_VP_SEL_A::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline(always)]
    pub fn is_acmp_i4(&self) -> bool {
        *self == COMP_VP_SEL_A::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `ACMP_I5`"]
    #[inline(always)]
    pub fn is_acmp_i5(&self) -> bool {
        *self == COMP_VP_SEL_A::ACMP_I5
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline(always)]
    pub fn is_band_gap(&self) -> bool {
        *self == COMP_VP_SEL_A::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `DACOUT0`"]
    #[inline(always)]
    pub fn is_dacout0(&self) -> bool {
        *self == COMP_VP_SEL_A::DACOUT0
    }
}
#[doc = "Field `COMP_VP_SEL` writer - Selects positive voltage input"]
pub type COMP_VP_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, COMP_VP_SEL_A, 3, O>;
impl<'a, const O: u8> COMP_VP_SEL_W<'a, O> {
    #[doc = "VOLTAGE_LADDER_OUTPUT"]
    #[inline(always)]
    pub fn voltage_ladder_output(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT)
    }
    #[doc = "ACMP_I1"]
    #[inline(always)]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline(always)]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline(always)]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline(always)]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I4)
    }
    #[doc = "ACMP_I5"]
    #[inline(always)]
    pub fn acmp_i5(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I5)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline(always)]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::BAND_GAP)
    }
    #[doc = "DAC0 output"]
    #[inline(always)]
    pub fn dacout0(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::DACOUT0)
    }
}
#[doc = "Field `COMP_VM_SEL` reader - Selects negative voltage input"]
pub type COMP_VM_SEL_R = crate::FieldReader<u8, COMP_VM_SEL_A>;
#[doc = "Selects negative voltage input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_VM_SEL_A {
    #[doc = "0: VOLTAGE_LADDER_OUTPUT"]
    VOLTAGE_LADDER_OUTPUT = 0,
    #[doc = "1: ACMP_I1"]
    ACMP_I1 = 1,
    #[doc = "2: ACMP_I2"]
    ACMP_I2 = 2,
    #[doc = "3: ACMP_I3"]
    ACMP_I3 = 3,
    #[doc = "4: ACMP_I4"]
    ACMP_I4 = 4,
    #[doc = "5: ACMP_I5"]
    ACMP_I5 = 5,
    #[doc = "6: Band gap. Internal reference voltage."]
    BAND_GAP = 6,
    #[doc = "7: DAC0 output"]
    DACOUT0 = 7,
}
impl From<COMP_VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_VM_SEL_A) -> Self {
        variant as _
    }
}
impl COMP_VM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_VM_SEL_A {
        match self.bits {
            0 => COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT,
            1 => COMP_VM_SEL_A::ACMP_I1,
            2 => COMP_VM_SEL_A::ACMP_I2,
            3 => COMP_VM_SEL_A::ACMP_I3,
            4 => COMP_VM_SEL_A::ACMP_I4,
            5 => COMP_VM_SEL_A::ACMP_I5,
            6 => COMP_VM_SEL_A::BAND_GAP,
            7 => COMP_VM_SEL_A::DACOUT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPUT`"]
    #[inline(always)]
    pub fn is_voltage_ladder_output(&self) -> bool {
        *self == COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline(always)]
    pub fn is_acmp_i1(&self) -> bool {
        *self == COMP_VM_SEL_A::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline(always)]
    pub fn is_acmp_i2(&self) -> bool {
        *self == COMP_VM_SEL_A::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline(always)]
    pub fn is_acmp_i3(&self) -> bool {
        *self == COMP_VM_SEL_A::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline(always)]
    pub fn is_acmp_i4(&self) -> bool {
        *self == COMP_VM_SEL_A::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `ACMP_I5`"]
    #[inline(always)]
    pub fn is_acmp_i5(&self) -> bool {
        *self == COMP_VM_SEL_A::ACMP_I5
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline(always)]
    pub fn is_band_gap(&self) -> bool {
        *self == COMP_VM_SEL_A::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `DACOUT0`"]
    #[inline(always)]
    pub fn is_dacout0(&self) -> bool {
        *self == COMP_VM_SEL_A::DACOUT0
    }
}
#[doc = "Field `COMP_VM_SEL` writer - Selects negative voltage input"]
pub type COMP_VM_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, COMP_VM_SEL_A, 3, O>;
impl<'a, const O: u8> COMP_VM_SEL_W<'a, O> {
    #[doc = "VOLTAGE_LADDER_OUTPUT"]
    #[inline(always)]
    pub fn voltage_ladder_output(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT)
    }
    #[doc = "ACMP_I1"]
    #[inline(always)]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline(always)]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline(always)]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline(always)]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I4)
    }
    #[doc = "ACMP_I5"]
    #[inline(always)]
    pub fn acmp_i5(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I5)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline(always)]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::BAND_GAP)
    }
    #[doc = "DAC0 output"]
    #[inline(always)]
    pub fn dacout0(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::DACOUT0)
    }
}
#[doc = "Field `EDGECLR` reader - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
pub type EDGECLR_R = crate::BitReader<bool>;
#[doc = "Field `EDGECLR` writer - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
pub type EDGECLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `COMPSTAT` reader - Comparator status. This bit reflects the state of the comparator output."]
pub type COMPSTAT_R = crate::BitReader<bool>;
#[doc = "Field `COMPSTAT` writer - Comparator status. This bit reflects the state of the comparator output."]
pub type COMPSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `COMPEDGE` reader - Comparator edge-detect status."]
pub type COMPEDGE_R = crate::BitReader<bool>;
#[doc = "Field `COMPEDGE` writer - Comparator edge-detect status."]
pub type COMPEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HYS` reader - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
pub type HYS_R = crate::FieldReader<u8, HYS_A>;
#[doc = "Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYS_A {
    #[doc = "0: None (the output will switch as the voltages cross)"]
    HYS_0 = 0,
    #[doc = "1: 5 mv"]
    HYS_1 = 1,
    #[doc = "2: 10 mv"]
    HYS_2 = 2,
    #[doc = "3: 20 mv"]
    HYS_3 = 3,
}
impl From<HYS_A> for u8 {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as _
    }
}
impl HYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            0 => HYS_A::HYS_0,
            1 => HYS_A::HYS_1,
            2 => HYS_A::HYS_2,
            3 => HYS_A::HYS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYS_0`"]
    #[inline(always)]
    pub fn is_hys_0(&self) -> bool {
        *self == HYS_A::HYS_0
    }
    #[doc = "Checks if the value of the field is `HYS_1`"]
    #[inline(always)]
    pub fn is_hys_1(&self) -> bool {
        *self == HYS_A::HYS_1
    }
    #[doc = "Checks if the value of the field is `HYS_2`"]
    #[inline(always)]
    pub fn is_hys_2(&self) -> bool {
        *self == HYS_A::HYS_2
    }
    #[doc = "Checks if the value of the field is `HYS_3`"]
    #[inline(always)]
    pub fn is_hys_3(&self) -> bool {
        *self == HYS_A::HYS_3
    }
}
#[doc = "Field `HYS` writer - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
pub type HYS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, HYS_A, 2, O>;
impl<'a, const O: u8> HYS_W<'a, O> {
    #[doc = "None (the output will switch as the voltages cross)"]
    #[inline(always)]
    pub fn hys_0(self) -> &'a mut W {
        self.variant(HYS_A::HYS_0)
    }
    #[doc = "5 mv"]
    #[inline(always)]
    pub fn hys_1(self) -> &'a mut W {
        self.variant(HYS_A::HYS_1)
    }
    #[doc = "10 mv"]
    #[inline(always)]
    pub fn hys_2(self) -> &'a mut W {
        self.variant(HYS_A::HYS_2)
    }
    #[doc = "20 mv"]
    #[inline(always)]
    pub fn hys_3(self) -> &'a mut W {
        self.variant(HYS_A::HYS_3)
    }
}
impl R {
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline(always)]
    pub fn compsa(&self) -> COMPSA_R {
        COMPSA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline(always)]
    pub fn comp_vp_sel(&self) -> COMP_VP_SEL_R {
        COMP_VP_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline(always)]
    pub fn comp_vm_sel(&self) -> COMP_VM_SEL_R {
        COMP_VM_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline(always)]
    pub fn edgeclr(&self) -> EDGECLR_R {
        EDGECLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline(always)]
    pub fn compstat(&self) -> COMPSTAT_R {
        COMPSTAT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline(always)]
    pub fn compedge(&self) -> COMPEDGE_R {
        COMPEDGE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EDGESEL_W<3> {
        EDGESEL_W::new(self)
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline(always)]
    pub fn compsa(&mut self) -> COMPSA_W<6> {
        COMPSA_W::new(self)
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline(always)]
    pub fn comp_vp_sel(&mut self) -> COMP_VP_SEL_W<8> {
        COMP_VP_SEL_W::new(self)
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline(always)]
    pub fn comp_vm_sel(&mut self) -> COMP_VM_SEL_W<11> {
        COMP_VM_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline(always)]
    pub fn edgeclr(&mut self) -> EDGECLR_W<20> {
        EDGECLR_W::new(self)
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline(always)]
    pub fn compstat(&mut self) -> COMPSTAT_W<21> {
        COMPSTAT_W::new(self)
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline(always)]
    pub fn compedge(&mut self) -> COMPEDGE_W<23> {
        COMPEDGE_W::new(self)
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W<25> {
        HYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
